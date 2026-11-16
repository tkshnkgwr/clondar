//! # 共通テーマ・パレットモジュール (`theme.rs`)
//!
//! ダークモード / ライトモードおよび背景透過モードに応じた統一カラーパレットと、
//! モーダルオーバーレイ共通の枠組みを提供します。

use eframe::egui;

/// テーマに応じた統一カラーパレット。
#[derive(Clone, Copy)]
pub struct Palette {
    pub window_bg: egui::Color32,
    pub header_bg: egui::Color32,
    pub card_bg: egui::Color32,
    pub modal_bg: egui::Color32,
    pub sub_surface: egui::Color32,
    pub border: egui::Color32,
    pub text_primary: egui::Color32,
    pub text_secondary: egui::Color32,
    pub text_muted: egui::Color32,
    pub pill_bg: egui::Color32,
    pub pill_active_bg: egui::Color32,
    pub accent: egui::Color32,
    pub sunday_holiday: egui::Color32,
    pub saturday: egui::Color32,
    pub hover_cell: egui::Color32,
    pub danger_hover: egui::Color32,
}

impl Palette {
    pub fn resolve(is_dark: bool, is_transparent: bool) -> Self {
        if is_dark {
            Self {
                window_bg: if is_transparent {
                    egui::Color32::from_rgba_unmultiplied(15, 23, 42, 135)
                } else {
                    egui::Color32::from_rgb(2, 6, 23)
                },
                header_bg: if is_transparent {
                    egui::Color32::from_rgba_unmultiplied(15, 23, 42, 165)
                } else {
                    egui::Color32::from_rgb(15, 23, 42)
                },
                card_bg: if is_transparent {
                    egui::Color32::from_rgba_unmultiplied(15, 23, 42, 115)
                } else {
                    egui::Color32::from_rgb(15, 23, 42)
                },
                modal_bg: egui::Color32::from_rgb(15, 23, 42),
                sub_surface: egui::Color32::from_rgb(2, 6, 23),
                border: egui::Color32::from_rgba_unmultiplied(51, 65, 85, 175),
                text_primary: egui::Color32::from_rgb(248, 250, 252),
                text_secondary: egui::Color32::from_rgb(203, 213, 225),
                text_muted: egui::Color32::from_rgb(148, 163, 184),
                pill_bg: egui::Color32::from_rgb(30, 41, 59),
                pill_active_bg: egui::Color32::from_rgb(51, 65, 85),
                accent: egui::Color32::from_rgb(59, 130, 246),
                sunday_holiday: egui::Color32::from_rgb(239, 68, 68),
                saturday: egui::Color32::from_rgb(96, 165, 250),
                hover_cell: egui::Color32::from_rgba_unmultiplied(30, 41, 59, 185),
                danger_hover: egui::Color32::from_rgb(239, 68, 68),
            }
        } else {
            Self {
                window_bg: if is_transparent {
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 150)
                } else {
                    egui::Color32::from_rgb(248, 250, 252)
                },
                header_bg: if is_transparent {
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 180)
                } else {
                    egui::Color32::from_rgb(255, 255, 255)
                },
                card_bg: if is_transparent {
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 130)
                } else {
                    egui::Color32::from_rgb(255, 255, 255)
                },
                modal_bg: egui::Color32::from_rgb(255, 255, 255),
                sub_surface: egui::Color32::from_rgb(248, 250, 252),
                border: egui::Color32::from_rgba_unmultiplied(203, 213, 225, 200),
                text_primary: egui::Color32::from_rgb(15, 23, 42),
                text_secondary: egui::Color32::from_rgb(51, 65, 85),
                text_muted: egui::Color32::from_rgb(100, 116, 139),
                pill_bg: egui::Color32::from_rgb(241, 245, 249),
                pill_active_bg: egui::Color32::from_rgb(255, 255, 255),
                accent: egui::Color32::from_rgb(37, 99, 235),
                sunday_holiday: egui::Color32::from_rgb(220, 38, 38),
                saturday: egui::Color32::from_rgb(37, 99, 235),
                hover_cell: egui::Color32::from_rgba_unmultiplied(226, 232, 240, 185),
                danger_hover: egui::Color32::from_rgb(239, 68, 68),
            }
        }
    }
}

/// ウィンドウ全面にモーダル用オーバーレイ背景と中央カードを描画する共通ヘルパー。
///
/// カード外側をクリックした場合は `true`（閉じる要求）を返します。
pub fn show_modal_overlay(
    ctx: &egui::Context,
    id_salt: &str,
    modal_size: egui::Vec2,
    pal: &Palette,
    content: impl FnOnce(&mut egui::Ui),
) -> bool {
    let mut close_requested = false;
    let screen_rect = ctx.input(|i| i.content_rect());

    egui::Area::new(egui::Id::unique(id_salt))
        .fixed_pos(screen_rect.min)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            // 背面へのクリック透過を防ぐため、全面を暗色半透明で覆う
            let backdrop_resp = ui.allocate_rect(screen_rect, egui::Sense::click());
            ui.painter().rect_filled(
                screen_rect,
                egui::CornerRadius::same(16),
                egui::Color32::from_rgba_unmultiplied(15, 23, 42, 190),
            );

            let card_rect = egui::Rect::from_center_size(screen_rect.center(), modal_size);
            if backdrop_resp.clicked() {
                if let Some(pos) = backdrop_resp.interact_pointer_pos() {
                    if !card_rect.contains(pos) {
                        close_requested = true;
                    }
                }
            }

            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(card_rect)
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );

            egui::Frame::new()
                .fill(pal.modal_bg)
                .stroke(egui::Stroke::new(1.0, pal.border))
                .corner_radius(egui::CornerRadius::same(18))
                .inner_margin(egui::Margin::same(0))
                .show(&mut child_ui, |ui| {
                    ui.set_min_size(modal_size);
                    ui.set_max_size(modal_size);
                    content(ui);
                });
        });

    close_requested
}
