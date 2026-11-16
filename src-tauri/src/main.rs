//! # Clondar Pro メイン実行モジュール (main.rs)
//!
//! Windows デスクトップ専用のウィジェット型時計＆カレンダーアプリケーションのメインプロセスです。
//!
//! ## 責務とアーキテクチャ
//! - **多重起動防止**: 名前付きミューテックス（Named Mutex）を用いてプロセスの重複実行を遮断します。
//! - **OSネイティブウィンドウ装飾制御**:
//!   - タイトルバー・枠線の除去（ボーダレス: `decorations = false`）
//!   - OS標準ドロップシャドウの物理的無効化（`set_shadow(false)`）
//!   - 透過背景のサポートおよび常時最前面（Always on Top）表示
//!   - クローズ要求のインターセプトによる非表示（常駐トレイ化）
//! - **システムトレイ（Task Tray）**: 右クリックメニューからの表示切替、最前面ピン留め、位置リセット、完全終了の制御。
//! - **IPC コマンド群**:
//!   - [`get_holidays`][]: 日本の祝日定義に基づき、指定西暦年の祝日一覧を高速・型安全に算出。
//!   - [`load_holidays_json`][]: ローカル AppData から祝日定義を読み込み（存在しない場合は内蔵デフォルトで初期化生成）。
//!   - [`save_holidays_json`][]: 厳格なスキーマおよび範囲検証を行った上で祝日定義ファイルを安全に上書き保存。
//!   - [`reset_holidays_json`][]: 祝日定義を内蔵の初期デフォルト設定で安全に強制上書き復元。
//!   - [`get_holidays_diff`][]: 共有ライブラリ `common_lib` の LCS Diff アルゴリズムを用いた設定差分の計算。
//!   - [`get_word_counts`][]: 共有ライブラリ `common_lib` による複数キーワードの出現回数一括集計。

// リリースビルド時にWindowsで追加のコンソールウィンドウが表示されるのを防ぎます。削除しないでください！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager, WebviewWindowBuilder,
};

mod holidays;

/// アプリケーションが明示的な終了処理中（システムトレイメニューの「終了」選択時など）であるかを管理する共有状態。
///
/// ## 仕組みと役割
/// トレイ格納のためにメインウィンドウを破棄（クローズ）した際、Tauri の全ウィンドウ終了フック（`RunEvent::ExitRequested`）が
/// 発火します。その際アプリを終了させずにトレイ常駐を維持するため、このフラグを確認します。
/// - `false`: ウィンドウ破棄による終了要求をキャンセル（`api.prevent_exit()`）し、Rust 本体のみの超軽量トレイ常駐へ移行します。
/// - `true`: トレイメニュー「終了」からの明示的な終了指示であり、プロセスの正規終了を許可します。
struct QuittingState(Mutex<bool>);

/// メインウィンドウの再生成処理が進行中かを管理する排他ガード。
struct RecreatingState(AtomicBool);

/// アプリケーションに組み込まれている内蔵標準の祝日設定データ（JSON形式）。
///
/// ## 特徴
/// `src/config/holidays.json` を単一ソースとしてコンパイル時に埋め込みます。
/// 外部祝日設定ファイル（`%LOCALAPPDATA%/com.clondar.pro/holidays.json`）が存在しない初期起動時、
/// またはファイルが削除された場合に、本文字列から新規設定ファイルが自動生成されます。
const DEFAULT_HOLIDAYS_JSON: &str = include_str!("../../src/config/holidays.json");

/// 祝日設定ファイル（`holidays.json`）の保存先となるローカルデータ領域の絶対パスを取得します。
///
/// ## 動作仕様
/// 1. OSのユーザー固有ローカルデータ領域（Windowsでは `%LOCALAPPDATA%/com.clondar.pro/`）のパスを解決します。
/// 2. 対象ディレクトリが存在しない場合は、自動的に親ディレクトリを含めて新規作成します。
/// 3. ファイル名 `holidays.json` を付与した絶対パスを返します。
///
/// # 引数
/// * `app` - Tauri アプリケーションの管理ハンドル（OSパス解決に使用）
///
/// # 戻り値
/// * `Ok(PathBuf)` - 祝日設定ファイルへの完全なファイルパス
/// * `Err(String)` - ディレクトリの取得または作成に失敗した場合のエラーメッセージ
fn get_holidays_file_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    path.push("holidays.json");
    Ok(path)
}

/// 祝日設定ファイル（`holidays.json`）の内容を文字列としてロードする IPC コマンド。
///
/// ## 処理フロー
/// 1. [`get_holidays_file_path`] で保存先パスを取得します。
/// 2. ファイルが既に存在する場合は、その内容を読み取って返します。
/// 3. ファイルが存在しない場合は、内蔵の標準定義 [`DEFAULT_HOLIDAYS_JSON`] をファイルに書き出して初期化し、その内容を返却します。
///
/// # 引数
/// * `app` - Tauri アプリケーションの管理ハンドル
///
/// # 戻り値
/// * `Ok(String)` - 祝日設定の JSON 文字列
/// * `Err(String)` - ファイルの読み込みまたは初期生成時の書き込みに失敗した場合のエラーメッセージ
#[tauri::command]
fn load_holidays_json(app: tauri::AppHandle) -> Result<String, String> {
    let path = get_holidays_file_path(&app)?;
    if path.exists() {
        fs::read_to_string(&path).map_err(|e| e.to_string())
    } else {
        fs::write(&path, DEFAULT_HOLIDAYS_JSON).map_err(|e| e.to_string())?;
        Ok(DEFAULT_HOLIDAYS_JSON.to_string())
    }
}

/// 祝日設定を内蔵の初期デフォルト設定で強制上書き復元（リセット）する IPC コマンド。
///
/// # 引数
/// * `app` - Tauri アプリケーションの管理ハンドル
///
/// # 戻り値
/// * `Ok(String)` - 復元されたデフォルト JSON 文字列
/// * `Err(String)` - ファイル書き込みエラー時のメッセージ
#[tauri::command]
fn reset_holidays_json(app: tauri::AppHandle) -> Result<String, String> {
    let path = get_holidays_file_path(&app)?;
    fs::write(&path, DEFAULT_HOLIDAYS_JSON).map_err(|e| e.to_string())?;
    Ok(DEFAULT_HOLIDAYS_JSON.to_string())
}

/// 指定された西暦年の日本の祝日マップ（日付文字列 => 祝日名称）を計算して返却する IPC コマンド。
///
/// ## 処理フロー
/// 1. 設定ファイルから現在の祝日定義をロードし、Serde 構造体にパース。
/// 2. `holidays::calculate_holidays` を用いて、固定祝日、ハッピーマンデー、天文計算、振替休日、
///    国民の休日、および特定年の上書きを高速一括計算。
///
/// # 引数
/// * `app` - Tauri アプリケーションの管理ハンドル
/// * `year` - 対象の西暦年 (例: 2026)
///
/// # 戻り値
/// * `Ok(HashMap<String, String>)` - 日付文字列をキー、祝日名称を値とするマップ
/// * `Err(String)` - 設定読み込みまたはパース失敗時のエラーメッセージ
#[tauri::command]
fn get_holidays(app: tauri::AppHandle, year: u32) -> Result<HashMap<String, String>, String> {
    let json_text = load_holidays_json(app)?;
    let config = holidays::validate_and_parse_config(&json_text)?;
    Ok(holidays::calculate_holidays(year, &config))
}

/// フロントエンドから送信された祝日設定の JSON 文字列をファイルに保存する IPC コマンド。
///
/// ## 堅牢性と安全設計
/// ファイルへの書き込みを行う前に、[`holidays::validate_and_parse_config`] を用いて
/// スキーマおよび月・日・週番号の範囲妥当性チェックを実施します。
/// 不正な構文や無効なデータが検出された場合は保存を行わず、詳細なエラーを返却して
/// 設定ファイルの破損を未然に防止します。
///
/// # 引数
/// * `app` - Tauri アプリケーションの管理ハンドル
/// * `json_content` - 保存対象の JSON 文字列
///
/// # 戻り値
/// * `Ok(())` - 保存が正常に完了した場合
/// * `Err(String)` - JSON構文エラーまたはディスク書き込みエラー時のメッセージ
#[tauri::command]
fn save_holidays_json(app: tauri::AppHandle, json_content: String) -> Result<(), String> {
    let _ = holidays::validate_and_parse_config(&json_content)?;
    let path = get_holidays_file_path(&app)?;
    fs::write(&path, json_content).map_err(|e| e.to_string())?;
    Ok(())
}

/// キーワード出現回数の集計結果。
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct WordCountItem {
    /// 検索対象キーワード
    word: String,
    /// 出現回数
    count: usize,
}

/// 2つのテキスト（内蔵デフォルト設定と現在適用中の設定）間のテキスト行単位差分を計算する IPC コマンド。
///
/// ## 連携
/// 共通ライブラリ `common_lib::compute_diff` (最長共通部分列: LCS アルゴリズム) を呼び出し、
/// 高速に追加行・削除行・変更なし行を判定・分類します。
///
/// # 引数
/// * `old_text` - 比較元テキスト（省略時は内蔵デフォルト [`DEFAULT_HOLIDAYS_JSON`] を使用）
/// * `new_text` - 比較先テキスト（ユーザーがカスタマイズした祝日 JSON など）
///
/// # 戻り値
/// * `Ok(Vec<common_lib::DiffPart>)` - 追加・削除・変更なしの差分パーツ配列
#[tauri::command]
fn get_holidays_diff(
    old_text: Option<String>,
    new_text: String,
) -> Result<Vec<common_lib::DiffPart>, String> {
    let base = old_text.as_deref().unwrap_or(DEFAULT_HOLIDAYS_JSON);
    Ok(common_lib::compute_diff(base, &new_text))
}

/// 指定されたテキスト全体の中から、複数キーワードの出現回数を一括集計する IPC コマンド。
///
/// ## 連携
/// 共通ライブラリ `common_lib::count_occurrences` を呼び出して一括集計し、
/// IPC 往復回数とテキスト転送の重複を削減します。
///
/// # 引数
/// * `text` - 検索対象となるテキスト全体（JSON 文字列など）
/// * `words` - カウントする検索キーワード一覧
///
/// # 戻り値
/// * `Ok(Vec<WordCountItem>)` - キーワードごとの出現回数一覧
#[tauri::command]
fn get_word_counts(text: String, words: Vec<String>) -> Result<Vec<WordCountItem>, String> {
    let results = words
        .into_iter()
        .map(|word| {
            let count = common_lib::count_occurrences(&text, &word);
            WordCountItem { word, count }
        })
        .collect();
    Ok(results)
}

/// メインウィンドウにウィジェット用のネイティブ装飾（枠なし・影なし・初期最前面）を適用します。
fn configure_main_window(window: &tauri::WebviewWindow) {
    let _ = window.set_decorations(false);
    let _ = window.set_shadow(false);
    let _ = window.set_always_on_top(true);
}

/// トレイ格納中に破棄されたメインウィンドウを別スレッドで再生成します。
///
/// Windows 環境の WebView2 では同期イベントハンドラ内でのウィンドウ生成がデッドロックするため、
/// 必ず独立スレッド上から `WebviewWindowBuilder::from_config` を呼び出します。
fn spawn_recreate_main_window(app: tauri::AppHandle) {
    let state = app.state::<RecreatingState>();
    if state
        .0
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }

    std::thread::spawn(move || {
        struct ResetGuard(tauri::AppHandle);
        impl Drop for ResetGuard {
            fn drop(&mut self) {
                self.0
                    .state::<RecreatingState>()
                    .0
                    .store(false, Ordering::SeqCst);
            }
        }
        let _guard = ResetGuard(app.clone());

        if let Some(existing) = app.get_webview_window("main") {
            let _ = existing.show();
            let _ = existing.set_focus();
            return;
        }

        if let Some(window_config) = app.config().app.windows.first().cloned() {
            match WebviewWindowBuilder::from_config(&app, &window_config) {
                Ok(builder) => {
                    if let Ok(window) = builder.build() {
                        configure_main_window(&window);
                    }
                }
                Err(e) => {
                    eprintln!("メインウィンドウの再生成に失敗しました: {}", e);
                }
            }
        }
    });
}

/// アプリケーションのエントリーポイント。
///
/// ## 実行ライフサイクル
/// 1. **二重起動防止チェック**:
///    `common_lib::check_single_instance` を呼び出し、同一マシン上で既に Clondar が起動している場合は
///    メッセージを出力して直ちにプロセスを安全終了（`std::process::exit(0)`）します。
/// 2. **Tauri ビルダーの初期化**:
///    - `QuittingState`（終了フラグ）と `RecreatingState`（再生成排他フラグ）をマネージドステートとして登録します。
///    - IPC コマンドハンドラー（祝日計算、差分計算、単語一括集計、祝日ロード・保存・リセット）を登録します。
/// 3. **メインウィンドウのネイティブ装飾セットアップ (`setup` フック)**:
///    - `configure_main_window`: OS標準枠線・影を除去し、最前面表示を設定（表示自体は座標復元後にフロントエンドが1回実行）。
/// 4. **システムトレイ（タスクトレイ）メニューの構築**:
///    - 「表示 / 非表示」「最前面表示の切替」「位置をリセット」「終了」メニュー項目を作成。
///    - 非表示（トレイ格納）時は WebView ウィンドウを破棄して WebView2 メモリを完全解放し、再表示時に再生成します。
/// 5. **常駐イベントループ (`RunEvent::ExitRequested`)**:
///    - トレイ格納で全ウィンドウが破棄されても、`QuittingState` が `false` の場合は `api.prevent_exit()` で常駐を維持します。
///
/// # パニック
/// * Tauri アプリケーションの初期化または実行ループ中に回復不能なエラーが発生した場合にパニックします。
fn main() {
    // 名前付き Mutex を用いた二重起動チェック
    if let Err(e) = common_lib::check_single_instance("com.clondar.pro.mutex", "Clondar") {
        eprintln!("起動失敗: {}", e);
        std::process::exit(0);
    }

    let app = tauri::Builder::default()
        .manage(QuittingState(Mutex::new(false)))
        .manage(RecreatingState(AtomicBool::new(false)))
        .invoke_handler(tauri::generate_handler![
            get_holidays,
            get_holidays_diff,
            get_word_counts,
            load_holidays_json,
            reset_holidays_json,
            save_holidays_json
        ])
        .setup(|app| {
            // メインウィンドウを取得 (Tauri v2)
            let window = app.get_webview_window("main").unwrap();
            configure_main_window(&window);

            // --- システムトレイメニューの構築 ---
            let toggle =
                MenuItem::with_id(app, "toggle", "表示 / 非表示", true, None::<&str>).unwrap();
            let always_on_top =
                MenuItem::with_id(app, "always_on_top", "最前面表示の切替", true, None::<&str>)
                    .unwrap();
            let reset_pos =
                MenuItem::with_id(app, "reset_pos", "位置をリセット", true, None::<&str>).unwrap();
            let quit = MenuItem::with_id(app, "quit", "終了", true, None::<&str>).unwrap();

            let menu =
                Menu::with_items(app, &[&toggle, &always_on_top, &reset_pos, &quit]).unwrap();

            let _tray = TrayIconBuilder::new()
                // デフォルトのウィンドウアイコンを使用
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "toggle" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                // トレイ格納時は WebView ウィンドウを破棄し WebView2 プロセスを解放
                                let _ = window.close();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        } else {
                            spawn_recreate_main_window(app.clone());
                        }
                    }
                    "always_on_top" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if let Ok(is_on_top) = window.is_always_on_top() {
                                let new_state = !is_on_top;
                                if window.set_always_on_top(new_state).is_ok() {
                                    let _ = window.emit("always-on-top-toggled", new_state);
                                }
                            }
                        }
                    }
                    "reset_pos" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.center().is_ok() {
                                if let Ok(pos) = window.outer_position() {
                                    let _ = window.emit("position-reset", (pos.x, pos.y));
                                }
                            }
                        }
                    }
                    "quit" => {
                        let quitting_state = app.state::<QuittingState>();
                        *quitting_state.0.lock().unwrap() = true;
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)
                .unwrap();

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Tauri アプリケーションの構築中にエラーが発生しました");

    app.run(|app_handle, event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            let quitting_state = app_handle.state::<QuittingState>();
            let is_quitting = *quitting_state.0.lock().unwrap();
            if !is_quitting {
                api.prevent_exit();
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 内蔵されているデフォルトの祝日設定 (`DEFAULT_HOLIDAYS_JSON`) が
    /// `holidays::validate_and_parse_config` の厳格なバリデーションを完全に通過することをテストします。
    #[test]
    fn test_default_holidays_json_validity() {
        let config = holidays::validate_and_parse_config(DEFAULT_HOLIDAYS_JSON)
            .expect("デフォルトの祝日 JSON のバリデーションに失敗しました");

        assert_eq!(
            config.fixed.len(),
            9,
            "固定祝日数は9件（天皇誕生日はemperor_birthdaysで動的制御）である必要があります"
        );
        assert_eq!(
            config.happy_mondays.len(),
            4,
            "ハッピーマンデーは4件である必要があります"
        );
        assert_eq!(
            config.happy_mondays_legacy.len(),
            5,
            "旧ハッピーマンデーは5件である必要があります"
        );
        assert_eq!(
            config.emperor_birthdays.len(),
            3,
            "天皇誕生日の定義は3件である必要があります"
        );
        assert!(
            config.custom_overrides.contains_key("2020"),
            "2020年の上書き設定が必要です"
        );
        assert!(
            config.custom_overrides.contains_key("2021"),
            "2021年の上書き設定が必要です"
        );
    }

    /// 差分計算コマンド (`get_holidays_diff`) のロジックをテストします。
    #[test]
    fn test_holidays_diff_logic() {
        let old_text = "line1\nline2\nline3";
        let new_text = "line1\nline2_modified\nline3";
        let diff = common_lib::compute_diff(old_text, new_text);

        assert!(!diff.is_empty(), "差分リストが生成される必要があります");
        let has_removed = diff
            .iter()
            .any(|p| matches!(p.diff_type, common_lib::DiffType::Removed));
        let has_added = diff
            .iter()
            .any(|p| matches!(p.diff_type, common_lib::DiffType::Added));
        assert!(
            has_removed,
            "変更前の行が削除として検出される必要があります"
        );
        assert!(has_added, "変更後の行が追加として検出される必要があります");
    }

    /// キーワード出現回数一括集計コマンド (`get_word_counts`) のロジックをテストします。
    #[test]
    fn test_word_counts_logic() {
        let text = r#"{"fixed": {"01-01": "元日", "02-11": "建国記念の日", "04-29": "昭和の日"}}"#;
        let words = vec!["の日".to_string(), "元日".to_string()];
        let results = get_word_counts(text.to_string(), words).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].count, 2, "「の日」が2件検出される必要があります");
        assert_eq!(results[1].count, 1, "「元日」が1件検出される必要があります");
    }
}
