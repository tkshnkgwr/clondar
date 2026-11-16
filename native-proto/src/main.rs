#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! # Clondar Pro Phase C ネイティブ GUI プロトタイプ (`clondar-native`)
//!
//! WebView2（Chromium）を使用せず、`egui` + `glow`（OpenGL）による純粋な Rust ネイティブ描画で
//! 枠なし・影なし・背景透過・最前面固定・システムトレイ常駐を実現する軽量アプリです。
//! 既存の祝日計算エンジン（`src-tauri/src/holidays.rs`）および `common_lib` を直接再利用しています。

#[allow(dead_code)]
#[path = "../../src-tauri/src/holidays.rs"]
mod holidays;

mod config;
mod font;
mod help_modal;
mod holidays_manager;
mod icons;
mod theme;
mod tray;
mod yearly_view;

use chrono::{Datelike, Local, NaiveDate, Timelike};
use config::{AppStateConfig, ClockType};
use eframe::egui;
use holidays_manager::HolidaysManagerState;
use std::collections::HashMap;
use std::f32::consts::TAU;
use std::time::Duration;
use theme::Palette;
use tray::{install_tray_wakeup_hook, TrayCommand, TrayService};
use yearly_view::YearlyViewState;

const WINDOW_WIDTH: f32 = 1100.0;
const WINDOW_HEIGHT: f32 = 540.0;
const PROTO_MUTEX_NAME: &str = "com.clondar.native.proto.mutex";

const WEEKDAYS: [&str; 7] = ["日", "月", "火", "水", "木", "金", "土"];

/// カレンダー1マス分の事前計算データ。
struct CalendarCell {
    date_str: String,
    day: u32,
    weekday_idx: usize,
    is_current_month: bool,
}

/// Phase C ネイティブ GUI アプリケーション本体。
struct ClondarNativeApp {
    state: AppStateConfig,
    holiday_config: holidays::HolidayConfig,
    holidays_cache: HashMap<u32, HashMap<String, String>>,
    display_year: i32,
    display_month: u32,
    cached_grid_ym: (i32, u32),
    cached_grid: Vec<CalendarCell>,
    is_visible: bool,
    is_quitting: bool,
    tray_service: Option<TrayService>,
    show_help: bool,
    yearly_modal: Option<YearlyViewState>,
    holidays_modal: Option<HolidaysManagerState>,
}

impl ClondarNativeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        font::setup_fonts(&cc.egui_ctx);
        install_tray_wakeup_hook(cc.egui_ctx.clone());

        let state = AppStateConfig::load_or_default();
        let json_text = config::load_holidays_json_text();
        let holiday_config = holidays::validate_and_parse_config(&json_text).unwrap_or_else(|_| {
            holidays::validate_and_parse_config(config::DEFAULT_HOLIDAYS_JSON)
                .expect("内蔵デフォルト祝日JSONのパースに失敗")
        });

        // 初期位置およびピン留め状態を反映
        cc.egui_ctx
            .send_viewport_cmd(egui::ViewportCommand::WindowLevel(if state.is_pinned {
                egui::WindowLevel::AlwaysOnTop
            } else {
                egui::WindowLevel::Normal
            }));
        if let Some([x, y]) = state.window_pos {
            cc.egui_ctx
                .send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(x, y)));
        }

        let now = Local::now();
        let display_year = now.year();
        let display_month = now.month();
        let cached_grid = build_calendar_grid(display_year, display_month);

        let mut app = Self {
            state,
            holiday_config,
            holidays_cache: HashMap::new(),
            display_year,
            display_month,
            cached_grid_ym: (display_year, display_month),
            cached_grid,
            is_visible: true,
            is_quitting: false,
            tray_service: TrayService::new().ok(),
            show_help: false,
            yearly_modal: None,
            holidays_modal: None,
        };
        app.ensure_holidays_loaded(display_year as u32);
        app
    }

    fn ensure_holidays_loaded(&mut self, year: u32) -> &HashMap<String, String> {
        self.holidays_cache
            .entry(year)
            .or_insert_with(|| holidays::calculate_holidays(year, &self.holiday_config))
    }

    fn update_grid_if_needed(&mut self) {
        if self.cached_grid_ym != (self.display_year, self.display_month) {
            self.cached_grid = build_calendar_grid(self.display_year, self.display_month);
            self.cached_grid_ym = (self.display_year, self.display_month);
        }
    }

    fn hide_to_tray(&mut self, ctx: &egui::Context) {
        self.is_visible = false;
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
    }

    fn show_from_tray(&mut self, ctx: &egui::Context) {
        self.is_visible = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        ctx.request_repaint();
    }

    fn apply_pinned(&mut self, ctx: &egui::Context, pinned: bool) {
        self.state.is_pinned = pinned;
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(if pinned {
            egui::WindowLevel::AlwaysOnTop
        } else {
            egui::WindowLevel::Normal
        }));
        self.state.save();
    }

    fn poll_tray(&mut self, ctx: &egui::Context) {
        let mut commands = Vec::new();
        if let Some(tray) = &self.tray_service {
            while let Some(cmd) = tray.poll_command() {
                commands.push(cmd);
            }
        }

        for cmd in commands {
            match cmd {
                TrayCommand::ToggleVisibility => {
                    if self.is_visible {
                        self.hide_to_tray(ctx);
                    } else {
                        self.show_from_tray(ctx);
                    }
                }
                TrayCommand::ToggleAlwaysOnTop => {
                    let next = !self.state.is_pinned;
                    self.apply_pinned(ctx, next);
                }
                TrayCommand::ResetPosition => {
                    if let Some(monitor_size) = ctx.input(|i| i.viewport().monitor_size) {
                        let x = ((monitor_size.x - WINDOW_WIDTH) * 0.5).max(0.0);
                        let y = ((monitor_size.y - WINDOW_HEIGHT) * 0.5).max(0.0);
                        self.state.window_pos = Some([x, y]);
                        self.state.save();
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(
                            x, y,
                        )));
                    }
                    if !self.is_visible {
                        self.show_from_tray(ctx);
                    }
                }
                TrayCommand::Quit => {
                    self.is_quitting = true;
                    self.state.save();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
    }

    fn sync_window_position(&mut self, ctx: &egui::Context) {
        if let Some(outer_rect) = ctx.input(|i| i.viewport().outer_rect) {
            let new_pos = [outer_rect.min.x, outer_rect.min.y];
            let changed = match self.state.window_pos {
                Some([px, py]) => (px - new_pos[0]).abs() > 0.5 || (py - new_pos[1]).abs() > 0.5,
                None => true,
            };
            if changed {
                self.state.window_pos = Some(new_pos);
                self.state.save();
            }
        }
    }

    fn render_header(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, pal: &Palette) {
        let header_frame = egui::Frame::new()
            .fill(pal.header_bg)
            .inner_margin(egui::Margin::symmetric(16, 8));

        header_frame.show(ui, |ui| {
            let row_w = ui.available_width();
            ui.allocate_ui_with_layout(
                egui::vec2(row_w, 26.0),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    // 左側タイトル
                    let (dot_rect, _) =
                        ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                    ui.painter()
                        .circle_filled(dot_rect.center(), 5.0, pal.accent);

                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("CLONDAR PRO")
                            .size(12.5)
                            .color(pal.text_muted),
                    );
                    ui.label(
                        egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .size(11.0)
                            .color(pal.text_muted),
                    );

                    // 右端コントロールボタン（絵文字なし・すべてベクター線画アイコン）
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let close_resp = icons::icon_button(
                            ui,
                            egui::vec2(26.0, 24.0),
                            pal.danger_hover,
                            |p, r, hovered| {
                                let color = if hovered {
                                    egui::Color32::WHITE
                                } else {
                                    pal.text_muted
                                };
                                icons::draw_close_icon(p, r, color);
                            },
                        );
                        if close_resp.on_hover_text("トレイへ格納").clicked() {
                            self.hide_to_tray(ctx);
                        }

                        ui.add_space(4.0);
                        // 区切り線
                        let (sep_rect, _) =
                            ui.allocate_exact_size(egui::vec2(1.0, 14.0), egui::Sense::hover());
                        ui.painter().line_segment(
                            [
                                egui::pos2(sep_rect.center().x, sep_rect.min.y),
                                egui::pos2(sep_rect.center().x, sep_rect.max.y),
                            ],
                            egui::Stroke::new(1.0, pal.border),
                        );
                        ui.add_space(4.0);

                        let trans_color = if self.state.is_transparent {
                            pal.accent
                        } else {
                            pal.text_muted
                        };
                        let trans_resp = icons::icon_button(
                            ui,
                            egui::vec2(26.0, 24.0),
                            pal.hover_cell,
                            |p, r, _| icons::draw_sun_icon(p, r, trans_color),
                        );
                        if trans_resp.on_hover_text("背景透過").clicked() {
                            self.state.is_transparent = !self.state.is_transparent;
                            self.state.save();
                        }

                        ui.add_space(4.0);
                        let pin_color = if self.state.is_pinned {
                            pal.accent
                        } else {
                            pal.text_muted
                        };
                        let pinned = self.state.is_pinned;
                        let pin_resp = icons::icon_button(
                            ui,
                            egui::vec2(26.0, 24.0),
                            pal.hover_cell,
                            |p, r, _| icons::draw_pin_icon(p, r, pin_color, pinned),
                        );
                        if pin_resp.on_hover_text("最前面表示").clicked() {
                            self.apply_pinned(ctx, !pinned);
                        }

                        ui.add_space(4.0);
                        let help_resp = icons::icon_button(
                            ui,
                            egui::vec2(26.0, 24.0),
                            pal.hover_cell,
                            |p, r, hovered| {
                                let color = if hovered { pal.accent } else { pal.text_muted };
                                icons::draw_help_icon(p, r, color);
                            },
                        );
                        if help_resp.on_hover_text("ヘルプ・アプリ情報").clicked() {
                            self.show_help = true;
                        }

                        // 残り余白をヘッダードラッグ領域にする
                        let drag_space = ui.available_size();
                        if drag_space.x > 8.0 {
                            let resp = ui.allocate_response(
                                egui::vec2(drag_space.x, 24.0),
                                egui::Sense::drag(),
                            );
                            if resp.drag_started() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                            }
                        }
                    });
                },
            );
        });

        // 下端 1px 区切り線
        let sep_rect = ui.available_rect_before_wrap();
        ui.painter().line_segment(
            [
                egui::pos2(sep_rect.min.x, sep_rect.min.y),
                egui::pos2(sep_rect.max.x, sep_rect.min.y),
            ],
            egui::Stroke::new(1.0, pal.border),
        );
    }

    fn render_clock_pane(&mut self, ui: &mut egui::Ui, pal: &Palette) {
        ui.vertical_centered(|ui| {
            ui.add_space(4.0);

            // DIGITAL / ANALOG 切替ピル（左右中央配置＆余白なしピッタリ幅）
            let pill_inner_w = 88.0 * 2.0 + 4.0;
            let pill_outer_w = pill_inner_w + 8.0;
            let left_pad = ((ui.available_width() - pill_outer_w) * 0.5).max(0.0);
            ui.horizontal(|ui| {
                ui.add_space(left_pad);
                ui.allocate_ui_with_layout(
                    egui::vec2(pill_outer_w, 36.0),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        egui::Frame::new()
                            .fill(pal.pill_bg)
                            .corner_radius(egui::CornerRadius::same(10))
                            .inner_margin(egui::Margin::same(4))
                            .show(ui, |ui| {
                                ui.set_min_width(pill_inner_w);
                                ui.set_max_width(pill_inner_w);
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 4.0;
                                    let dig_active = self.state.clock_type == ClockType::Digital;
                                    if pill_tab_button(ui, "DIGITAL", dig_active, pal).clicked() {
                                        self.state.clock_type = ClockType::Digital;
                                        self.state.save();
                                    }
                                    if pill_tab_button(ui, "ANALOG", !dig_active, pal).clicked() {
                                        self.state.clock_type = ClockType::Analog;
                                        self.state.save();
                                    }
                                });
                            });
                    },
                );
            });

            ui.add_space(10.0);

            // メイン時計エリア（Digital / Analog 切替時に下部ボタンが動かないよう最小高さを強制固定）
            const CLOCK_AREA_HEIGHT: f32 = 312.0;
            let now = Local::now();
            ui.allocate_ui_with_layout(
                egui::vec2(450.0, CLOCK_AREA_HEIGHT),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.set_min_size(egui::vec2(450.0, CLOCK_AREA_HEIGHT));
                    ui.set_max_size(egui::vec2(450.0, CLOCK_AREA_HEIGHT));
                    match self.state.clock_type {
                        ClockType::Digital => {
                            ui.add_space(74.0);
                            render_digital_clock(
                                ui,
                                &now,
                                self.state.is_24_hour,
                                self.state.show_seconds,
                                self.state.is_dark_mode,
                                pal,
                            );
                        }
                        ClockType::Analog => {
                            ui.add_space(6.0);
                            render_analog_clock(
                                ui,
                                &now,
                                self.state.show_seconds,
                                self.state.is_dark_mode,
                                pal,
                            );
                        }
                    }
                },
            );

            ui.add_space(14.0);

            // 下部コントロールボタン群（文字サイズ 12.5px、線画テーマアイコン、固定幅で位置ブレ防止）
            ui.horizontal(|ui| {
                let total_w = 210.0;
                let offset = ((ui.available_width() - total_w) * 0.5).max(0.0);
                ui.add_space(offset);

                let is_digital = self.state.clock_type == ClockType::Digital;
                let btn_24h = toggle_chip_button(
                    ui,
                    if self.state.is_24_hour { "24H" } else { "12H" },
                    self.state.is_24_hour && is_digital,
                    is_digital,
                    64.0,
                    pal,
                )
                .on_disabled_hover_text("アナログ表示中は 12H/24H を切り替えられません");
                if btn_24h.clicked() {
                    self.state.is_24_hour = !self.state.is_24_hour;
                    self.state.save();
                }

                ui.add_space(8.0);
                let sec_label = if self.state.show_seconds {
                    "秒表示: ON"
                } else {
                    "秒表示: OFF"
                };
                if toggle_chip_button(ui, sec_label, self.state.show_seconds, true, 96.0, pal)
                    .clicked()
                {
                    self.state.show_seconds = !self.state.show_seconds;
                    self.state.save();
                }

                ui.add_space(8.0);
                // テーマ切替（絵文字廃止・ベクター太陽/月アイコン）
                let (theme_rect, theme_resp) =
                    ui.allocate_exact_size(egui::vec2(34.0, 28.0), egui::Sense::click());
                let bg = if theme_resp.hovered() {
                    pal.hover_cell
                } else {
                    pal.pill_bg
                };
                ui.painter()
                    .rect_filled(theme_rect, egui::CornerRadius::same(8), bg);
                if self.state.is_dark_mode {
                    icons::draw_sun_icon(ui.painter(), theme_rect, pal.text_secondary);
                } else {
                    icons::draw_moon_icon(ui.painter(), theme_rect, pal.text_secondary, bg);
                }
                if theme_resp
                    .on_hover_text(if self.state.is_dark_mode {
                        "ライトモードへ切替"
                    } else {
                        "ダークモードへ切替"
                    })
                    .clicked()
                {
                    self.state.is_dark_mode = !self.state.is_dark_mode;
                    self.state.save();
                }
            });
        });
    }

    fn render_calendar_pane(&mut self, ui: &mut egui::Ui, pal: &Palette) {
        self.update_grid_if_needed();
        let target_year = self.display_year.max(1948) as u32;
        let holidays_map = self.ensure_holidays_loaded(target_year).clone();

        let today = Local::now();
        let today_str = format!(
            "{:04}-{:02}-{:02}",
            today.year(),
            today.month(),
            today.day()
        );

        ui.label(
            egui::RichText::new("CALENDAR")
                .size(12.0)
                .color(pal.text_muted),
        );
        ui.add_space(6.0);

        egui::Frame::new()
            .fill(pal.card_bg)
            .stroke(egui::Stroke::new(1.0, pal.border))
            .corner_radius(egui::CornerRadius::same(16))
            .inner_margin(egui::Margin::same(18))
            .show(ui, |ui| {
                // 月ナビゲーションヘッダ（今年・今月・祝日設定・<・>）
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "{}年 {}月",
                            self.display_year, self.display_month
                        ))
                        .size(22.0)
                        .color(pal.text_primary),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let next_resp = icons::icon_button(
                            ui,
                            egui::vec2(28.0, 26.0),
                            pal.pill_bg,
                            |p, r, _| icons::draw_chevron_icon(p, r, pal.text_secondary, true),
                        );
                        if next_resp.clicked() {
                            if self.display_month == 12 {
                                self.display_year += 1;
                                self.display_month = 1;
                            } else {
                                self.display_month += 1;
                            }
                        }

                        let prev_resp = icons::icon_button(
                            ui,
                            egui::vec2(28.0, 26.0),
                            pal.pill_bg,
                            |p, r, _| icons::draw_chevron_icon(p, r, pal.text_secondary, false),
                        );
                        if prev_resp.clicked() {
                            if self.display_month == 1 {
                                self.display_year -= 1;
                                self.display_month = 12;
                            } else {
                                self.display_month -= 1;
                            }
                        }

                        ui.add_space(4.0);
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("祝日設定")
                                        .size(12.5)
                                        .color(pal.text_secondary),
                                )
                                .fill(pal.pill_bg)
                                .corner_radius(egui::CornerRadius::same(8))
                                .min_size(egui::vec2(68.0, 26.0)),
                            )
                            .clicked()
                        {
                            self.holidays_modal = Some(HolidaysManagerState::load());
                        }

                        ui.add_space(2.0);
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("今月")
                                        .size(12.5)
                                        .color(pal.text_secondary),
                                )
                                .fill(pal.pill_bg)
                                .corner_radius(egui::CornerRadius::same(8))
                                .min_size(egui::vec2(48.0, 26.0)),
                            )
                            .clicked()
                        {
                            let now = Local::now();
                            self.display_year = now.year();
                            self.display_month = now.month();
                        }

                        ui.add_space(2.0);
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("今年").size(12.5).color(pal.accent),
                                )
                                .fill(egui::Color32::from_rgba_unmultiplied(59, 130, 246, 26))
                                .corner_radius(egui::CornerRadius::same(8))
                                .min_size(egui::vec2(48.0, 26.0)),
                            )
                            .clicked()
                        {
                            self.yearly_modal = Some(YearlyViewState::new(self.display_year));
                        }
                    });
                });

                ui.add_space(12.0);

                let avail_w = ui.available_width();
                let col_w = (avail_w - 6.0 * 4.0) / 7.0;

                // 曜日ヘッダ
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    for (idx, wday) in WEEKDAYS.iter().enumerate() {
                        let color = if idx == 0 {
                            pal.sunday_holiday
                        } else if idx == 6 {
                            pal.saturday
                        } else {
                            pal.text_muted
                        };
                        let (rect, _) =
                            ui.allocate_exact_size(egui::vec2(col_w, 22.0), egui::Sense::hover());
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            *wday,
                            egui::FontId::proportional(12.0),
                            color,
                        );
                    }
                });

                ui.add_space(4.0);

                // 6週間 (42セル) グリッド
                for row in 0..6 {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        for col in 0..7 {
                            let cell = &self.cached_grid[row * 7 + col];
                            let holiday_name = holidays_map.get(&cell.date_str);
                            let is_today = cell.date_str == today_str;

                            let (rect, response) = ui
                                .allocate_exact_size(egui::vec2(col_w, 42.0), egui::Sense::hover());

                            if is_today {
                                ui.painter().rect_filled(
                                    rect,
                                    egui::CornerRadius::same(8),
                                    pal.accent,
                                );
                            } else if response.hovered() {
                                ui.painter().rect_filled(
                                    rect,
                                    egui::CornerRadius::same(8),
                                    pal.hover_cell,
                                );
                            }

                            let mut text_color = if is_today {
                                egui::Color32::WHITE
                            } else if holiday_name.is_some() || cell.weekday_idx == 0 {
                                pal.sunday_holiday
                            } else if cell.weekday_idx == 6 {
                                pal.saturday
                            } else {
                                pal.text_secondary
                            };

                            if !cell.is_current_month {
                                text_color = egui::Color32::from_rgba_unmultiplied(
                                    text_color.r(),
                                    text_color.g(),
                                    text_color.b(),
                                    60,
                                );
                            }

                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                cell.day.to_string(),
                                egui::FontId::proportional(15.5),
                                text_color,
                            );

                            if holiday_name.is_some() {
                                let dot_alpha = if cell.is_current_month { 255 } else { 80 };
                                let dot_color = if is_today {
                                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, dot_alpha)
                                } else {
                                    egui::Color32::from_rgba_unmultiplied(248, 113, 113, dot_alpha)
                                };
                                ui.painter().circle_filled(
                                    egui::pos2(rect.center().x, rect.max.y - 6.0),
                                    2.2,
                                    dot_color,
                                );
                            }

                            if let Some(name) = holiday_name {
                                response.on_hover_text(name.as_str());
                            }
                        }
                    });
                    ui.add_space(4.0);
                }
            });
    }

    fn render_modals(&mut self, ctx: &egui::Context, pal: &Palette) {
        if self.show_help && help_modal::render_help_modal(ctx, pal) {
            self.show_help = false;
        }

        if let Some(mut yearly) = self.yearly_modal.take() {
            let close = yearly_view::render_yearly_view(
                ctx,
                &mut yearly,
                &self.holiday_config,
                &mut self.holidays_cache,
                pal,
            );
            if !close {
                self.yearly_modal = Some(yearly);
            }
        }

        if let Some(mut mgr) = self.holidays_modal.take() {
            let (close, updated_cfg) =
                holidays_manager::render_holidays_manager(ctx, &mut mgr, pal);
            if let Some(cfg) = updated_cfg {
                self.holiday_config = cfg;
                self.holidays_cache.clear();
            }
            if !close {
                self.holidays_modal = Some(mgr);
            }
        }
    }
}

impl eframe::App for ClondarNativeApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.state.save();
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) && !self.is_quitting {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.hide_to_tray(ctx);
        }

        self.poll_tray(ctx);

        if self.is_visible && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            if self.show_help {
                self.show_help = false;
            } else if self.yearly_modal.is_some() {
                self.yearly_modal = None;
            } else if self.holidays_modal.is_some() {
                self.holidays_modal = None;
            } else {
                self.hide_to_tray(ctx);
            }
        }

        if self.is_visible {
            self.sync_window_position(ctx);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if !self.is_visible {
            return;
        }

        let ctx = ui.ctx().clone();

        // 表示モードに応じた省電力リペイント予約（秒・分境界に合わせて不要な再描画を排除）
        let now_for_repaint = Local::now();
        let ms_in_sec = u64::from(now_for_repaint.nanosecond() / 1_000_000);
        let repaint_interval = match (self.state.clock_type, self.state.show_seconds) {
            (ClockType::Analog, true) => Duration::from_millis(100),
            (ClockType::Analog, false) => {
                let sec_mod = u64::from(now_for_repaint.second() % 10);
                let remaining = ((10 - sec_mod) * 1000).saturating_sub(ms_in_sec) + 20;
                Duration::from_millis(remaining)
            }
            (ClockType::Digital, true) => {
                Duration::from_millis(1000u64.saturating_sub(ms_in_sec) + 15)
            }
            (ClockType::Digital, false) => {
                let sec = u64::from(now_for_repaint.second());
                let remaining = ((60 - sec) * 1000).saturating_sub(ms_in_sec) + 20;
                Duration::from_millis(remaining)
            }
        };
        ctx.request_repaint_after(repaint_interval);

        let pal = Palette::resolve(self.state.is_dark_mode, self.state.is_transparent);
        let root_frame = egui::Frame::new()
            .fill(pal.window_bg)
            .stroke(egui::Stroke::new(1.0, pal.border))
            .corner_radius(egui::CornerRadius::same(16));

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                root_frame.show(ui, |ui| {
                    ui.style_mut().interaction.selectable_labels = false;
                    ui.set_min_size(egui::vec2(WINDOW_WIDTH - 2.0, WINDOW_HEIGHT - 2.0));

                    self.render_header(ui, &ctx, &pal);

                    egui::Frame::new()
                        .inner_margin(egui::Margin::same(24))
                        .show(ui, |ui| {
                            // 余白クリックでもウィンドウをドラッグ移動できるよう背景センサーを配置
                            let full_rect = ui.available_rect_before_wrap();
                            let bg_resp = ui.interact(
                                full_rect,
                                ui.scope_id().with("bg_drag_sensor"),
                                egui::Sense::drag(),
                            );
                            if bg_resp.drag_started() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                            }

                            ui.horizontal(|ui| {
                                ui.allocate_ui_with_layout(
                                    egui::vec2(450.0, ui.available_height()),
                                    egui::Layout::top_down(egui::Align::Center),
                                    |ui| {
                                        self.render_clock_pane(ui, &pal);
                                    },
                                );

                                // 縦区切り線
                                let (div_rect, _) = ui.allocate_exact_size(
                                    egui::vec2(20.0, ui.available_height()),
                                    egui::Sense::hover(),
                                );
                                ui.painter().line_segment(
                                    [
                                        egui::pos2(div_rect.center().x, div_rect.min.y),
                                        egui::pos2(div_rect.center().x, div_rect.max.y),
                                    ],
                                    egui::Stroke::new(1.0, pal.border),
                                );

                                ui.allocate_ui_with_layout(
                                    egui::vec2(ui.available_width(), ui.available_height()),
                                    egui::Layout::top_down(egui::Align::LEFT),
                                    |ui| {
                                        self.render_calendar_pane(ui, &pal);
                                    },
                                );
                            });
                        });
                });
            });

        self.render_modals(&ctx, &pal);
    }
}

/// 等幅文字配置による揺れのないデジタル時計描画。
fn render_digital_clock(
    ui: &mut egui::Ui,
    now: &chrono::DateTime<Local>,
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
fn render_analog_clock(
    ui: &mut egui::Ui,
    now: &chrono::DateTime<Local>,
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

fn pill_tab_button(ui: &mut egui::Ui, label: &str, active: bool, pal: &Palette) -> egui::Response {
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

fn toggle_chip_button(
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

/// 指定年月の 6週間 (42日間) カレンダーグリッド配列を構築します。
fn build_calendar_grid(year: i32, month: u32) -> Vec<CalendarCell> {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).expect("有効な年月");
    let start_offset = first_day.weekday().num_days_from_sunday() as i64;
    let start_date = first_day - chrono::Duration::days(start_offset);

    let mut cells = Vec::with_capacity(42);
    for i in 0..42 {
        let d = start_date + chrono::Duration::days(i);
        cells.push(CalendarCell {
            date_str: format!("{:04}-{:02}-{:02}", d.year(), d.month(), d.day()),
            day: d.day(),
            weekday_idx: d.weekday().num_days_from_sunday() as usize,
            is_current_month: d.year() == year && d.month() == month,
        });
    }
    cells
}

fn main() -> eframe::Result<()> {
    let _single_instance_guard =
        match common_lib::desktop::acquire_single_instance(PROTO_MUTEX_NAME) {
            Some(guard) => guard,
            None => return Ok(()),
        };

    let viewport = egui::ViewportBuilder::default()
        .with_title("Clondar Pro (Native Prototype)")
        .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
        .with_resizable(false)
        .with_decorations(false)
        .with_transparent(true)
        .with_has_shadow(false)
        .with_taskbar(false)
        .with_always_on_top();

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Clondar Pro (Native Prototype)",
        options,
        Box::new(|cc| Ok(Box::new(ClondarNativeApp::new(cc)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_calendar_grid_has_42_cells() {
        let grid = build_calendar_grid(2026, 11);
        assert_eq!(grid.len(), 42);
        // 2026-11-01 は日曜日（先頭セル）
        assert_eq!(grid[0].date_str, "2026-11-01");
        assert!(grid[0].is_current_month);
    }

    #[test]
    fn test_shared_holidays_engine_works() {
        let cfg = holidays::validate_and_parse_config(config::DEFAULT_HOLIDAYS_JSON).unwrap();
        let h2026 = holidays::calculate_holidays(2026, &cfg);
        assert_eq!(h2026.get("2026-01-01").map(String::as_str), Some("元日"));
        assert_eq!(
            h2026.get("2026-05-06").map(String::as_str),
            Some("振替休日")
        );
    }
}
