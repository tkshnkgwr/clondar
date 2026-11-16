//! # システムトレイ常駐モジュール (`tray.rs`)
//!
//! `tray-icon` クレートを用いてタスクトレイアイコンおよび右クリックメニューを構築し、
//! 非表示待機中でも即座に `egui` イベントループを起床させてコマンドを処理します。

use eframe::egui;
use std::collections::VecDeque;
use std::sync::{Mutex, Once, OnceLock};
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};

const MENU_ID_TOGGLE: &str = "toggle";
const MENU_ID_ALWAYS_ON_TOP: &str = "always_on_top";
const MENU_ID_RESET_POS: &str = "reset_pos";
const MENU_ID_QUIT: &str = "quit";

/// トレイ操作から UI ループへ渡すコマンド。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayCommand {
    ToggleVisibility,
    ToggleAlwaysOnTop,
    ResetPosition,
    Quit,
}

static TRAY_WAKEUP_CTX: OnceLock<Mutex<Option<egui::Context>>> = OnceLock::new();
static TRAY_HANDLER_INIT: Once = Once::new();
static TRAY_MENU_QUEUE: Mutex<VecDeque<MenuEvent>> = Mutex::new(VecDeque::new());
static TRAY_ICON_QUEUE: Mutex<VecDeque<TrayIconEvent>> = Mutex::new(VecDeque::new());

/// 32x32 の時計モチーフ RGBA トレイアイコンをコード生成します（外部画像デコーダ不要）。
fn build_clock_tray_icon() -> Result<Icon, String> {
    const SIZE: usize = 32;
    let mut rgba = vec![0u8; SIZE * SIZE * 4];
    let center = 15.5_f32;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt();
            let idx = (y * SIZE + x) * 4;

            if dist <= 14.2 {
                // 外枠と文字盤（ブルーベース）
                if dist >= 11.8 {
                    rgba[idx] = 59;
                    rgba[idx + 1] = 130;
                    rgba[idx + 2] = 246;
                    rgba[idx + 3] = 255;
                } else {
                    rgba[idx] = 15;
                    rgba[idx + 1] = 23;
                    rgba[idx + 2] = 42;
                    rgba[idx + 3] = 255;
                }
            }
        }
    }

    // 針（10時10分風の白線）を簡易描画
    let mut set_px = |px: usize, py: usize, r: u8, g: u8, b: u8| {
        if px < SIZE && py < SIZE {
            let idx = (py * SIZE + px) * 4;
            rgba[idx] = r;
            rgba[idx + 1] = g;
            rgba[idx + 2] = b;
            rgba[idx + 3] = 255;
        }
    };

    // 中心点
    for dy in 14..=17 {
        for dx in 14..=17 {
            set_px(dx, dy, 255, 255, 255);
        }
    }
    // 縦針（12時方向）
    for py in 7..=15 {
        set_px(15, py, 255, 255, 255);
        set_px(16, py, 255, 255, 255);
    }
    // 右針（3時方向）
    for px in 16..=23 {
        set_px(px, 15, 96, 165, 250);
        set_px(px, 16, 96, 165, 250);
    }

    Icon::from_rgba(rgba, SIZE as u32, SIZE as u32)
        .map_err(|e| format!("トレイアイコン生成失敗: {e}"))
}

/// トレイイベント受信時に `egui` を即座に再描画・起床させるフックを登録します。
pub fn install_tray_wakeup_hook(ctx: egui::Context) {
    let cell = TRAY_WAKEUP_CTX.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = cell.lock() {
        *guard = Some(ctx);
    }

    TRAY_HANDLER_INIT.call_once(|| {
        MenuEvent::set_event_handler(Some(|event: MenuEvent| {
            if let Ok(mut queue) = TRAY_MENU_QUEUE.lock() {
                queue.push_back(event);
            }
            if let Some(cell) = TRAY_WAKEUP_CTX.get() {
                if let Ok(guard) = cell.lock() {
                    if let Some(ctx) = guard.as_ref() {
                        ctx.request_repaint();
                    }
                }
            }
        }));

        TrayIconEvent::set_event_handler(Some(|event: TrayIconEvent| {
            if let Ok(mut queue) = TRAY_ICON_QUEUE.lock() {
                queue.push_back(event);
            }
            if let Some(cell) = TRAY_WAKEUP_CTX.get() {
                if let Ok(guard) = cell.lock() {
                    if let Some(ctx) = guard.as_ref() {
                        ctx.request_repaint();
                    }
                }
            }
        }));
    });
}

/// システムトレイアイコンおよび右クリックメニューを保持するサービス。
pub struct TrayService {
    _tray_icon: TrayIcon,
}

impl TrayService {
    /// 標準メニュー構成でトレイアイコンを初期化します。
    pub fn new() -> Result<Self, String> {
        let icon = build_clock_tray_icon()?;
        let menu = Menu::new();

        let toggle = MenuItem::with_id(MENU_ID_TOGGLE, "表示 / 非表示", true, None);
        let always_on_top =
            MenuItem::with_id(MENU_ID_ALWAYS_ON_TOP, "最前面表示の切替", true, None);
        let reset_pos = MenuItem::with_id(MENU_ID_RESET_POS, "位置をリセット", true, None);
        let separator = PredefinedMenuItem::separator();
        let quit = MenuItem::with_id(MENU_ID_QUIT, "終了", true, None);

        menu.append_items(&[&toggle, &always_on_top, &reset_pos, &separator, &quit])
            .map_err(|e| format!("メニュー追加失敗: {e}"))?;

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Clondar Pro")
            .with_icon(icon)
            .build()
            .map_err(|e| format!("トレイ登録失敗: {e}"))?;

        Ok(Self {
            _tray_icon: tray_icon,
        })
    }

    /// 未処理のトレイコマンドを1件取り出します。
    pub fn poll_command(&self) -> Option<TrayCommand> {
        if let Ok(mut queue) = TRAY_MENU_QUEUE.lock() {
            while let Some(event) = queue.pop_front() {
                match event.id().0.as_str() {
                    MENU_ID_TOGGLE => return Some(TrayCommand::ToggleVisibility),
                    MENU_ID_ALWAYS_ON_TOP => return Some(TrayCommand::ToggleAlwaysOnTop),
                    MENU_ID_RESET_POS => return Some(TrayCommand::ResetPosition),
                    MENU_ID_QUIT => return Some(TrayCommand::Quit),
                    _ => {}
                }
            }
        }

        if let Ok(mut queue) = TRAY_ICON_QUEUE.lock() {
            while let Some(event) = queue.pop_front() {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    return Some(TrayCommand::ToggleVisibility);
                }
            }
        }

        None
    }
}
