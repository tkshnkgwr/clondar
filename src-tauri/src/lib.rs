//! # Clondar Pro コアライブラリ (lib.rs)
//!
//! 本モジュールは、Clondar Pro デスクトップアプリケーションのエントリーポイントおよび
//! 共通初期化・セットアップ処理を提供します。
//!
//! ## 概要
//! Tauri v2 フレームワークのビルダーパターンを利用して、アプリケーションインスタンスを構築・起動します。
//! デバッグビルド時にはログプラグインを自動登録し、開発・トラブルシューティング時の可観測性を高めます。

/// Tauri アプリケーションの構築・初期化およびイベントループの開始を行います。
///
/// ## 処理フロー
/// 1. デバッグビルド時（`debug_assertions` 有効時）に `tauri-plugin-log` を登録し、
///    ログレベル `Info` での出力を開始します。
/// 2. `tauri.conf.json` およびコンテキスト（権限マニフェスト、ウィンドウ設定）をロードします。
/// 3. アプリケーションのメインイベントループを実行します。
///
/// ## モバイル対応
/// `#[cfg_attr(mobile, tauri::mobile_entry_point)]` 属性により、将来的なモバイルプラットフォーム
/// （Android / iOS）への移植時にもエントリーポイントとして機能します。
///
/// ## パニック
/// * Tauri アプリケーションのコンテキスト生成または起動処理中に致命的なエラーが発生した場合に
///   エラーメッセージを出力してパニック（プロセス終了）します。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri アプリケーションの実行中にエラーが発生しました");
}
