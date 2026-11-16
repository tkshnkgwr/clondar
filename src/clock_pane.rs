//! # 時計描画モジュール (`clock_pane.rs`)
//!
//! 等幅スロット配置による揺れのないデジタル時計（12H/24H・秒表示切替対応）および
//! `egui::Painter` ベクター描画による滑らかなスイープ運針アナログ時計を提供します。

use crate::theme::Palette;
use chrono::{DateTime, Local, Timelike};
use eframe::egui;
use std::f32::consts::TAU;

/// 等幅文字配置による揺れのないデジタル時計描画。
pub fn render_digital_clock(
    ui: &mut egui::Ui,
    now: &DateTime<Local>,
    is_24_hour: bool,
    show_seconds: bool,
    is_dark: bool,
    pal: &Palette,
) {
    let (hour_num, am_pm) = if is_24_hour {
        (now.hour(), None)
    } else {
        let h = now.hour() % 12;
        let display_h = if h == 0 { 12 } else { h };
        let label = if now.hour() < 12 { "AM" } else { "PM" };
        (display_h, Some(label))
    };

    let time_str = if show_seconds {
        format!("{:02}:{:02}:{:02}", hour_num, now.minute(), now.second())
    } else {
        format!("{:02}:{:02}", hour_num, now.minute())
    };

    // AM/PM エリア（高さ固定で12H/24H切替時の上下揺れを防止）
    let (ampm_rect, _) = ui.allocate_exact_size(egui::vec2(420.0, 28.0), egui::Sense::hover());
    if let Some(label) = am_pm {
        ui.painter().text(
            ampm_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(18.0),
            pal.text_muted,
        );
    }

    ui.add_space(4.0);

    // 1文字ずつ固定幅スロットへ描画し、秒変化による横ブレを完全防止
    let font_size = 92.0_f32;
    let digit_w = 54.0_f32;
    let colon_w = 24.0_f32;
    let total_w: f32 = time_str
        .chars()
        .map(|c| if c == ':' { colon_w } else { digit_w })
        .sum();

    let (clock_rect, _) = ui.allocate_exact_size(egui::vec2(total_w, 110.0), egui::Sense::hover());
    let text_color = if is_dark {
        egui::Color32::WHITE
    } else {
        pal.accent
    };
    let font_id = egui::FontId::new(font_size, egui::FontFamily::Name("impact".into()));

    let mut cursor_x = clock_rect.min.x;
    for ch in time_str.chars() {
        let w = if ch == ':' { colon_w } else { digit_w };
        let center = egui::pos2(cursor_x + w * 0.5, clock_rect.center().y);
        ui.painter().text(
            center,
            egui::Align2::CENTER_CENTER,
            ch.to_string(),
            font_id.clone(),
            text_color,
        );
        cursor_x += w;
    }
}

/// ベクター描画（`egui::Painter`）による滑らかなアナログ時計描画（直径 300px）。
pub fn render_analog_clock(
    ui: &mut egui::Ui,
    now: &DateTime<Local>,
    show_seconds: bool,
    is_dark: bool,
    pal: &Palette,
) {
    let size = 300.0;
    let scale = size / 210.0;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    let center = rect.center();
    let radius = size * 0.48;
    let painter = ui.painter();

    let face_fill = if is_dark {
        egui::Color32::from_rgba_unmultiplied(30, 41, 59, 110)
    } else {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 130)
    };
    painter.circle_filled(center, radius, face_fill);
    painter.circle_stroke(center, radius, egui::Stroke::new(2.4, pal.border));

    // 12時間目盛り
    for i in 0..12 {
        let angle = (i as f32) * (TAU / 12.0) - TAU * 0.25;
        let is_main = i % 3 == 0;
        let inner = radius - if is_main { 16.0 * scale } else { 11.0 * scale };
        let outer = radius - 4.0 * scale;
        let p1 = center + egui::vec2(angle.cos() * inner, angle.sin() * inner);
        let p2 = center + egui::vec2(angle.cos() * outer, angle.sin() * outer);
        let color = if is_main { pal.accent } else { pal.text_muted };
        let stroke_w = if is_main { 5.0 } else { 3.0 };
        painter.line_segment([p1, p2], egui::Stroke::new(stroke_w, color));
    }

    let sec = now.second() as f32 + (now.nanosecond() as f32) / 1_000_000_000.0;
    let min = now.minute() as f32 + sec / 60.0;
    let hour = (now.hour() % 12) as f32 + min / 60.0;

    // 時針
    let hour_angle = (hour / 12.0) * TAU - TAU * 0.25;
    let hour_end = center
        + egui::vec2(
            hour_angle.cos() * (52.0 * scale),
            hour_angle.sin() * (52.0 * scale),
        );
    painter.line_segment([center, hour_end], egui::Stroke::new(6.8, pal.text_primary));

    // 分針
    let min_angle = (min / 60.0) * TAU - TAU * 0.25;
    let min_end = center
        + egui::vec2(
            min_angle.cos() * (76.0 * scale),
            min_angle.sin() * (76.0 * scale),
        );
    painter.line_segment([center, min_end], egui::Stroke::new(4.2, pal.text_muted));

    // 秒針（スイープ運針）
    if show_seconds {
        let sec_angle = (sec / 60.0) * TAU - TAU * 0.25;
        let sec_end = center
            + egui::vec2(
                sec_angle.cos() * (88.0 * scale),
                sec_angle.sin() * (88.0 * scale),
            );
        painter.line_segment(
            [center, sec_end],
            egui::Stroke::new(2.2, egui::Color32::from_rgb(239, 68, 68)),
        );
    }

    painter.circle_filled(center, 8.0, pal.text_primary);
    painter.circle_stroke(center, 8.0, egui::Stroke::new(2.4, pal.card_bg));
}

/// 時計モード切替（DIGITAL / ANALOG）ピルボタンを描画します。
pub fn pill_tab_button(
    ui: &mut egui::Ui,
    label: &str,
    active: bool,
    pal: &Palette,
) -> egui::Response {
    let fill = if active {
        pal.pill_active_bg
    } else {
        egui::Color32::TRANSPARENT
    };
    let text_color = if active { pal.accent } else { pal.text_muted };
    ui.add(
        egui::Button::new(egui::RichText::new(label).size(12.5).color(text_color))
            .fill(fill)
            .corner_radius(egui::CornerRadius::same(8))
            .min_size(egui::vec2(88.0, 28.0)),
    )
}

/// 時計オプション用のチップ型トグルボタンを描画します。
pub fn toggle_chip_button(
    ui: &mut egui::Ui,
    label: &str,
    active: bool,
    enabled: bool,
    min_w: f32,
    pal: &Palette,
) -> egui::Response {
    let (fill, text_color) = if !enabled {
        (
            egui::Color32::from_rgba_unmultiplied(
                pal.pill_bg.r(),
                pal.pill_bg.g(),
                pal.pill_bg.b(),
                120,
            ),
            egui::Color32::from_rgba_unmultiplied(
                pal.text_muted.r(),
                pal.text_muted.g(),
                pal.text_muted.b(),
                120,
            ),
        )
    } else if active {
        (pal.accent, egui::Color32::WHITE)
    } else {
        (pal.pill_bg, pal.text_secondary)
    };
    ui.add_enabled(
        enabled,
        egui::Button::new(egui::RichText::new(label).size(12.5).color(text_color))
            .fill(fill)
            .corner_radius(egui::CornerRadius::same(8))
            .min_size(egui::vec2(min_w, 28.0)),
    )
}
