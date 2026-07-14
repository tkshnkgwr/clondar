//! アプリケーションのエントリーポイントおよび共通セットアップ処理を提供するライブラリ。

/// アプリケーションのビルドと起動を行います。
///
/// デバッグビルド時にはログプラグイン（`tauri-plugin-log`）を有効にし、
/// ログレベル `Info` でロギングを開始します。
///
/// # パニック
/// Tauri アプリケーションの初期化または実行中にエラーが発生した場合は、メッセージを出力してパニックします。
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

