//! # 年間カレンダー表示モーダル (`yearly_view.rs`)
//!
//! 指定された西暦年の 12ヶ月分（1月〜12月）を一覧表示し、前年・翌年・「今年」へ
//! 素早く切り替えられる全画面モーダルビューを提供します。

use crate::holidays::{self, HolidayConfig};
use crate::icons;
use crate::theme::{show_modal_overlay, Palette};
use chrono::{Datelike, Local, NaiveDate};
use eframe::egui;
use std::collections::HashMap;

const WEEKDAYS: [&str; 7] = ["日", "月", "火", "水", "木", "金", "土"];

struct MonthDayCell {
    day: u32,
    date_str: String,
    is_sun: bool,
    is_sat: bool,
}

struct MonthBlock {
    month_num: u32,
    start_offset: usize,
    days: Vec<MonthDayCell>,
}

/// 年間カレンダーモーダルの状態管理。
pub struct YearlyViewState {
    pub year: i32,
    cached_year: i32,
    months: Vec<MonthBlock>,
}

impl YearlyViewState {
    pub fn new(initial_year: i32) -> Self {
        let months = build_year_blocks(initial_year);
        Self {
            year: initial_year,
            cached_year: initial_year,
            months,
        }
    }

    pub fn set_year(&mut self, next_year: i32) {
        self.year = next_year;
        if self.cached_year != next_year {
            self.months = build_year_blocks(next_year);
            self.cached_year = next_year;
        }
    }
}

fn build_year_blocks(year: i32) -> Vec<MonthBlock> {
    (1..=12)
        .map(|month| {
            let first = NaiveDate::from_ymd_opt(year, month, 1).expect("有効な月初日");
            let start_offset = first.weekday().num_days_from_sunday() as usize;
            let days_count = holidays::days_in_month(year.max(1) as u32, month);
            let mut days = Vec::with_capacity(days_count as usize);

            for day in 1..=days_count {
                let d = NaiveDate::from_ymd_opt(year, month, day).expect("有効な日付");
                let w = d.weekday().num_days_from_sunday();
                days.push(MonthDayCell {
                    day,
                    date_str: format!("{:04}-{:02}-{:02}", year, month, day),
                    is_sun: w == 0,
                    is_sat: w == 6,
                });
            }

            MonthBlock {
                month_num: month,
                start_offset,
                days,
            }
        })
        .collect()
}

/// 年間カレンダーモーダルを描画します。閉じる要求があった場合は `true` を返します。
pub fn render_yearly_view(
    ctx: &egui::Context,
    state: &mut YearlyViewState,
    holiday_config: &HolidayConfig,
    holidays_cache: &mut HashMap<u32, HashMap<String, String>>,
    pal: &Palette,
) -> bool {
    let mut should_close = false;
    let target_year = state.year.max(1948) as u32;
    let holidays_map = holidays_cache
        .entry(target_year)
        .or_insert_with(|| holidays::calculate_holidays(target_year, holiday_config))
        .clone();

    let today = Local::now();
    let today_str = format!(
        "{:04}-{:02}-{:02}",
        today.year(),
        today.month(),
        today.day()
    );
    let modal_size = egui::vec2(1020.0, 500.0);

    let clicked_outside = show_modal_overlay(ctx, "yearly_view_modal", modal_size, pal, |ui| {
        // ヘッダー
        egui::Frame::new()
            .fill(pal.sub_surface)
            .inner_margin(egui::Margin::symmetric(22, 14))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!("{}年 年間カレンダー", state.year))
                            .size(22.0)
                            .color(pal.text_primary),
                    );
                    ui.add_space(12.0);

                    ui.allocate_ui_with_layout(
                        egui::vec2(112.0, 30.0),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            egui::Frame::new()
                                .fill(pal.pill_bg)
                                .corner_radius(egui::CornerRadius::same(10))
                                .inner_margin(egui::Margin::same(3))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 4.0;
                                        let prev = icons::icon_button(
                                            ui,
                                            egui::vec2(26.0, 24.0),
                                            pal.pill_active_bg,
                                            |p, r, _| {
                                                icons::draw_chevron_icon(
                                                    p,
                                                    r,
                                                    pal.text_secondary,
                                                    false,
                                                )
                                            },
                                        );
                                        if prev.clicked() {
                                            state.set_year(state.year - 1);
                                        }

                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new("今年")
                                                        .size(12.5)
                                                        .color(pal.text_secondary),
                                                )
                                                .fill(egui::Color32::TRANSPARENT)
                                                .corner_radius(egui::CornerRadius::same(6))
                                                .min_size(egui::vec2(46.0, 24.0)),
                                            )
                                            .clicked()
                                        {
                                            state.set_year(Local::now().year());
                                        }

                                        let next = icons::icon_button(
                                            ui,
                                            egui::vec2(26.0, 24.0),
                                            pal.pill_active_bg,
                                            |p, r, _| {
                                                icons::draw_chevron_icon(
                                                    p,
                                                    r,
                                                    pal.text_secondary,
                                                    true,
                                                )
                                            },
                                        );
                                        if next.clicked() {
                                            state.set_year(state.year + 1);
                                        }
                                    });
                                });
                        },
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let resp = icons::icon_button(
                            ui,
                            egui::vec2(28.0, 28.0),
                            pal.hover_cell,
                            |p, r, _| icons::draw_close_icon(p, r, pal.text_muted),
                        );
                        if resp.clicked() {
                            should_close = true;
                        }
                    });
                });
            });

        // 12ヶ月グリッド（4列 × 3行、スクロール対応）
        egui::ScrollArea::vertical()
            .max_height(430.0)
            .show(ui, |ui| {
                egui::Frame::new()
                    .inner_margin(egui::Margin::symmetric(22, 14))
                    .show(ui, |ui| {
                        let avail_w = ui.available_width();
                        let col_gap = 18.0;
                        let month_w = (avail_w - col_gap * 3.0) / 4.0;

                        for row_months in state.months.chunks(4) {
                            ui.horizontal_top(|ui| {
                                for (idx, block) in row_months.iter().enumerate() {
                                    if idx > 0 {
                                        ui.add_space(col_gap);
                                    }
                                    render_month_card(
                                        ui,
                                        block,
                                        month_w,
                                        &holidays_map,
                                        &today_str,
                                        pal,
                                    );
                                }
                            });
                            ui.add_space(16.0);
                        }
                    });
            });
    });

    should_close || clicked_outside
}

fn render_month_card(
    ui: &mut egui::Ui,
    block: &MonthBlock,
    width: f32,
    holidays_map: &HashMap<String, String>,
    today_str: &str,
    pal: &Palette,
) {
    ui.allocate_ui_with_layout(
        egui::vec2(width, 196.0),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            ui.label(
                egui::RichText::new(format!("{}月", block.month_num))
                    .size(15.0)
                    .color(pal.accent),
            );
            ui.add_space(4.0);

            let cell_w = (width - 12.0) / 7.0;
            // 曜日ヘッダ
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                for (idx, wday) in WEEKDAYS.iter().enumerate() {
                    let color = if idx == 0 {
                        pal.sunday_holiday
                    } else if idx == 6 {
                        pal.saturday
                    } else {
                        pal.text_muted
                    };
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(cell_w, 16.0), egui::Sense::hover());
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        *wday,
                        egui::FontId::proportional(10.5),
                        color,
                    );
                }
            });

            // 6週分スロット（空セル＋日付セル）
            let total_slots = block.start_offset + block.days.len();
            let rows = total_slots.div_ceil(7);
            for r in 0..rows {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    for c in 0..7 {
                        let slot = r * 7 + c;
                        let (rect, resp) =
                            ui.allocate_exact_size(egui::vec2(cell_w, 22.0), egui::Sense::hover());

                        if slot < block.start_offset {
                            continue;
                        }
                        let day_idx = slot - block.start_offset;
                        if day_idx >= block.days.len() {
                            continue;
                        }
                        let cell = &block.days[day_idx];
                        let holiday_name = holidays_map.get(&cell.date_str);
                        let is_today = cell.date_str == today_str;

                        if is_today {
                            ui.painter()
                                .rect_filled(rect, egui::CornerRadius::same(5), pal.accent);
                        } else if resp.hovered() {
                            ui.painter().rect_filled(
                                rect,
                                egui::CornerRadius::same(5),
                                pal.hover_cell,
                            );
                        }

                        let text_color = if is_today {
                            egui::Color32::WHITE
                        } else if holiday_name.is_some() || cell.is_sun {
                            pal.sunday_holiday
                        } else if cell.is_sat {
                            pal.saturday
                        } else {
                            pal.text_secondary
                        };

                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            cell.day.to_string(),
                            egui::FontId::proportional(11.5),
                            text_color,
                        );

                        if holiday_name.is_some() {
                            ui.painter().circle_filled(
                                egui::pos2(rect.center().x, rect.max.y - 2.5),
                                1.6,
                                if is_today {
                                    egui::Color32::WHITE
                                } else {
                                    pal.sunday_holiday
                                },
                            );
                        }

                        if let Some(name) = holiday_name {
                            resp.on_hover_text(name.as_str());
                        }
                    }
                });
            }
        },
    );
}
