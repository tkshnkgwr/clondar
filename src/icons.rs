//! # ベクター線画アイコンモジュール (`icons.rs`)
//!
//! 絵文字フォント依存による文字化け（豆腐 `□`）を完全に排除するため、
//! `egui::Painter` を用いて太陽・月・ピン・ヘルプ・閉じる・矢印・ゴミ箱・プラス等の
//! ミニマルなベクター線画アイコンを描画します。

use eframe::egui;
use std::f32::consts::TAU;

/// 指定されたアイコンボタン領域を確保し、描画クロージャを実行します。
pub fn icon_button(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    hover_bg: egui::Color32,
    draw: impl FnOnce(&egui::Painter, egui::Rect, bool),
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    if response.hovered() {
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(6), hover_bg);
    }
    draw(ui.painter(), rect, response.hovered());
    response
}

/// ヘルプアイコン（丸枠＋`?`）を描画します。
pub fn draw_help_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let c = rect.center();
    let r = rect.width().min(rect.height()) * 0.36;
    painter.circle_stroke(c, r, egui::Stroke::new(1.5, color));
    painter.text(
        egui::pos2(c.x, c.y + 0.5),
        egui::Align2::CENTER_CENTER,
        "?",
        egui::FontId::proportional(r * 1.35),
        color,
    );
}

/// ピン留め（最前面固定）の線画アイコンを描画します。
pub fn draw_pin_icon(
    painter: &egui::Painter,
    rect: egui::Rect,
    color: egui::Color32,
    filled: bool,
) {
    let c = rect.center();
    let s = rect.width().min(rect.height()) * 0.34;
    // 上部ヘッド（斜め45度風）
    let head_center = egui::pos2(c.x + s * 0.28, c.y - s * 0.28);
    if filled {
        painter.circle_filled(head_center, s * 0.36, color);
    } else {
        painter.circle_stroke(head_center, s * 0.36, egui::Stroke::new(1.5, color));
    }
    // 鍔（つば）の線
    painter.line_segment(
        [
            egui::pos2(c.x - s * 0.35, c.y - s * 0.15),
            egui::pos2(c.x + s * 0.15, c.y + s * 0.35),
        ],
        egui::Stroke::new(1.7, color),
    );
    // 針先
    painter.line_segment(
        [
            egui::pos2(c.x - s * 0.10, c.y + s * 0.10),
            egui::pos2(c.x - s * 0.72, c.y + s * 0.72),
        ],
        egui::Stroke::new(1.6, color),
    );
}

/// 太陽の線画アイコンを描画します。
pub fn draw_sun_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let c = rect.center();
    let r = rect.width().min(rect.height()) * 0.19;
    painter.circle_stroke(c, r, egui::Stroke::new(1.5, color));
    let inner = r * 1.45;
    let outer = r * 2.05;
    for i in 0..8 {
        let angle = (i as f32) * (TAU / 8.0);
        let p1 = c + egui::vec2(angle.cos() * inner, angle.sin() * inner);
        let p2 = c + egui::vec2(angle.cos() * outer, angle.sin() * outer);
        painter.line_segment([p1, p2], egui::Stroke::new(1.4, color));
    }
}

/// 三日月の線画アイコンを描画します（豆腐化の心配ゼロのベクター描画）。
pub fn draw_moon_icon(
    painter: &egui::Painter,
    rect: egui::Rect,
    color: egui::Color32,
    bg_cutout: egui::Color32,
) {
    let c = rect.center();
    let r = rect.width().min(rect.height()) * 0.34;
    // 外円を塗ったあと、右上にずらした背景色円で切り抜くことで綺麗な三日月を描画
    painter.circle_filled(c, r, color);
    painter.circle_filled(
        egui::pos2(c.x + r * 0.42, c.y - r * 0.30),
        r * 0.84,
        bg_cutout,
    );
}

/// 閉じる（✕）の線画アイコンを描画します。
pub fn draw_close_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let c = rect.center();
    let d = rect.width().min(rect.height()) * 0.26;
    let stroke = egui::Stroke::new(1.6, color);
    painter.line_segment(
        [egui::pos2(c.x - d, c.y - d), egui::pos2(c.x + d, c.y + d)],
        stroke,
    );
    painter.line_segment(
        [egui::pos2(c.x + d, c.y - d), egui::pos2(c.x - d, c.y + d)],
        stroke,
    );
}

/// 左・右のシェブロン矢印（`<` / `>`）を描画します。
pub fn draw_chevron_icon(
    painter: &egui::Painter,
    rect: egui::Rect,
    color: egui::Color32,
    right: bool,
) {
    let c = rect.center();
    let w = rect.width().min(rect.height()) * 0.16;
    let h = rect.width().min(rect.height()) * 0.28;
    let dir = if right { 1.0 } else { -1.0 };
    let stroke = egui::Stroke::new(1.7, color);
    let p_top = egui::pos2(c.x - w * dir, c.y - h);
    let p_mid = egui::pos2(c.x + w * dir, c.y);
    let p_bot = egui::pos2(c.x - w * dir, c.y + h);
    painter.line_segment([p_top, p_mid], stroke);
    painter.line_segment([p_mid, p_bot], stroke);
}
