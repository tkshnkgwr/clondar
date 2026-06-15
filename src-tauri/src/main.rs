// Prevents additional console window on Windows in release, do not remove!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // メインウィンドウを取得 (Tauri v2)
      let window = app.get_webview_window("main").unwrap();
      
      // 強制的に枠を削除
      window.set_decorations(false).unwrap();
      
      // 強制的に影を削除 (Windows)
      window.set_shadow(false).unwrap();
      
      // 強制的に最前面に設定
      window.set_always_on_top(true).unwrap();
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
