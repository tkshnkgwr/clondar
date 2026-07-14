//! Clondar アプリケーションのメインエントリポイント。
//! ウィンドウの枠なし・最前面表示・透過処理のセットアップ、システムトレイメニューの制御、
//! および祝日設定データのロード・保存などのコマンドを提供します。

// リリースビルド時にWindowsで追加のコンソールウィンドウが表示されるのを防ぎます。削除しないでください！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};

/// アプリケーションが明示的な終了処理中（トレイメニューの終了ボタン等）であるかどうかを管理する状態。
///
/// ウィンドウの `CloseRequested` イベント時にこのフラグを確認し、
/// `false` の場合はアプリ終了をインターセプトして非表示（常駐状態）にします。
struct QuittingState(Mutex<bool>);

/// アプリケーションに内蔵されているデフォルトの祝日設定（JSON形式）。
/// 祝日設定ファイルが存在しない場合にこの文字列から初期設定ファイルが生成されます。
const DEFAULT_HOLIDAYS_JSON: &str = r#"{
  "fixed": {
    "01-01": "元日",
    "02-11": "建国記念の日",
    "02-23": "天皇誕生日",
    "04-29": "昭和の日",
    "05-03": "憲法記念日",
    "05-04": "みどりの日",
    "05-05": "こどもの日",
    "08-11": "山の日",
    "11-03": "文化の日",
    "11-23": "勤労感謝の日"
  },
  "happy_mondays": [
    {
      "month": 1,
      "week": 2,
      "name": "成人の日",
      "start_year": 2000
    },
    {
      "month": 7,
      "week": 3,
      "name": "海の日",
      "start_year": 2003
    },
    {
      "month": 9,
      "week": 3,
      "name": "敬老の日",
      "start_year": 2003
    },
    {
      "month": 10,
      "week": 2,
      "name": "スポーツの日",
      "start_year": 2020
    }
  ],
  "happy_mondays_legacy": [
    {
      "month": 1,
      "day": 15,
      "name": "成人の日",
      "start_year": 1949,
      "end_year": 1999
    },
    {
      "month": 7,
      "day": 20,
      "name": "海の日",
      "start_year": 1996,
      "end_year": 2002
    },
    {
      "month": 9,
      "day": 15,
      "name": "敬老の日",
      "start_year": 1966,
      "end_year": 2002
    },
    {
      "month": 10,
      "day": 10,
      "name": "体育の日",
      "start_year": 1966,
      "end_year": 1999
    },
    {
      "month": 10,
      "week": 2,
      "name": "体育の日",
      "start_year": 2000,
      "end_year": 2019
    }
  ],
  "emperor_birthdays": [
    {
      "start_year": 1949,
      "end_year": 1988,
      "month": 4,
      "day": 29
    },
    {
      "start_year": 1989,
      "end_year": 2018,
      "month": 12,
      "day": 23
    },
    {
      "start_year": 2020,
      "month": 2,
      "day": 23
    }
  ],
  "custom_overrides": {
    "2020": {
      "2020-07-23": "海の日",
      "2020-07-24": "スポーツの日",
      "2020-08-10": "山の日",
      "2020-07-20": null,
      "2020-08-11": null,
      "2020-10-12": null
    },
    "2021": {
      "2021-07-22": "海の日",
      "2021-07-23": "スポーツの日",
      "2021-08-08": "山の日",
      "2021-07-19": null,
      "2021-08-11": null,
      "2021-10-11": null
    }
  }
}"#;

/// 祝日設定ファイルの保存先となる絶対パスを取得します。
/// アプリケーションデータディレクトリが存在しない場合は自動で作成します。
///
/// # 引数
/// * `app` - Tauri アプリケーションのハンドル
///
/// # 戻り値
/// 祝日設定ファイル (`holidays.json`) へのパスを含む `Result`
fn get_holidays_file_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    path.push("holidays.json");
    Ok(path)
}

/// 祝日設定の JSON ファイルの内容をロードします。
/// ファイルが存在しない場合は、内蔵のデフォルトの祝日設定をファイルに書き込んで初期化します。
///
/// # 引数
/// * `app` - Tauri アプリケーションのハンドル
///
/// # 戻り値
/// 祝日設定ファイルの JSON 文字列を含む `Result`
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

/// 祝日設定の JSON 文字列をファイルに保存します。
/// 保存前に JSON の形式が正しいかをチェックし、不正な場合はエラーを返します。
///
/// # 引数
/// * `app` - Tauri アプリケーションのハンドル
/// * `json_content` - 保存する JSON 文字列
///
/// # 戻り値
/// 保存処理の結果を含む `Result`
#[tauri::command]
fn save_holidays_json(app: tauri::AppHandle, json_content: String) -> Result<(), String> {
    let _: serde_json::Value = serde_json::from_str(&json_content)
        .map_err(|e| format!("JSONのフォーマットが正しくありません: {}", e))?;
    let path = get_holidays_file_path(&app)?;
    fs::write(&path, json_content).map_err(|e| e.to_string())?;
    Ok(())
}

/// 2つのテキスト（古い設定と新しい設定）の間の差分（Diff）を計算します。
/// 共通ライブラリ `common_lib::compute_diff` を使用します。
///
/// # 引数
/// * `old_text` - 変更前の設定テキスト
/// * `new_text` - 変更後の設定テキスト
///
/// # 戻り値
/// 差分パーツのリストを含む `Result`
#[tauri::command]
fn get_holidays_diff(
    old_text: String,
    new_text: String,
) -> Result<Vec<common_lib::DiffPart>, String> {
    Ok(common_lib::compute_diff(&old_text, &new_text))
}

/// テキストデータ中において、指定された単語（祝日の名称など）が出現する回数をカウントします。
/// 共通ライブラリ `common_lib::count_occurrences` を使用します。
///
/// # 引数
/// * `text` - 検索対象のテキスト全体
/// * `word` - カウント対象の単語
///
/// # 戻り値
/// 出現回数を含む `Result`
#[tauri::command]
fn get_word_count(text: String, word: String) -> Result<usize, String> {
    Ok(common_lib::count_occurrences(&text, &word))
}

/// アプリケーションのメインエントリーポイント。
/// 多重起動のチェックを行い、Tauri アプリケーションのビルド、
/// 各種ウィンドウ設定（枠なし、最前面表示、影なし、常駐化など）、
/// およびシステムトレイメニューの設定を行います。
fn main() {
    // 名前付き Mutex を用いた二重起動チェック
    if let Err(e) = common_lib::check_single_instance("com.clondar.pro.mutex", "Clondar") {
        eprintln!("起動失敗: {}", e);
        std::process::exit(0);
    }

    tauri::Builder::default()
        .manage(QuittingState(Mutex::new(false)))
        .invoke_handler(tauri::generate_handler![
            get_holidays_diff,
            get_word_count,
            load_holidays_json,
            save_holidays_json
        ])
        .setup(|app| {
            // メインウィンドウを取得 (Tauri v2)
            let window = app.get_webview_window("main").unwrap();

            // 強制的に枠を削除
            window.set_decorations(false).unwrap();

            // 強制的に影を削除 (Windows)
            window.set_shadow(false).unwrap();

            // 強制的に最前面に設定
            window.set_always_on_top(true).unwrap();

            // ウィンドウが閉じられようとしたときは、破棄せずに非表示にする (常駐化対策)
            let app_handle = app.handle().clone();
            let win_clone = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    let quitting_state = app_handle.state::<QuittingState>();
                    let is_quitting = *quitting_state.0.lock().unwrap();
                    if !is_quitting {
                        api.prevent_close();
                        let _ = win_clone.hide();
                    }
                }
            });

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
                            if let Ok(visible) = window.is_visible() {
                                if visible {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
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
                        for window in app.webview_windows().values() {
                            let _ = window.close();
                        }
                    }
                    _ => {}
                })
                .build(app)
                .unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri アプリケーションの実行中にエラーが発生しました");
}
