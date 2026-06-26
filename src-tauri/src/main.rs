// Prevents additional console window on Windows in release, do not remove!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
  menu::{Menu, MenuItem},
  tray::TrayIconBuilder,
  Emitter, Manager,
};

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

      // --- システムトレイメニューの構築 ---
      let toggle = MenuItem::with_id(app, "toggle", "表示 / 非表示", true, None::<&str>).unwrap();
      let always_on_top = MenuItem::with_id(app, "always_on_top", "最前面表示の切替", true, None::<&str>).unwrap();
      let reset_pos = MenuItem::with_id(app, "reset_pos", "位置をリセット", true, None::<&str>).unwrap();
      let quit = MenuItem::with_id(app, "quit", "終了", true, None::<&str>).unwrap();

      let menu = Menu::with_items(app, &[&toggle, &always_on_top, &reset_pos, &quit]).unwrap();

      let _tray = TrayIconBuilder::new()
        // デフォルトのウィンドウアイコンを使用
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| {
          match event.id.as_ref() {
            "toggle" => {
              let window = app.get_webview_window("main").unwrap();
              if window.is_visible().unwrap() {
                window.hide().unwrap();
              } else {
                window.show().unwrap();
                window.set_focus().unwrap();
              }
            }
            "always_on_top" => {
              let window = app.get_webview_window("main").unwrap();
              let is_on_top = window.is_always_on_top().unwrap();
              let new_state = !is_on_top;
              window.set_always_on_top(new_state).unwrap();
              // フロントエンドに状態を通知
              window.emit("always-on-top-toggled", new_state).unwrap();
            }
            "reset_pos" => {
              let window = app.get_webview_window("main").unwrap();
              window.center().unwrap();
              // 中央位置の物理座標を取得してフロントエンドに通知
              if let Ok(pos) = window.outer_position() {
                let _ = window.emit("position-reset", (pos.x, pos.y));
              }
            }
            "quit" => {
              app.exit(0);
            }
            _ => {}
          }
        })
        .build(app)
        .unwrap();
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
