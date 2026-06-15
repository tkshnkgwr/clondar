#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use chrono::{Local, Datelike, Timelike, NaiveDate, Weekday, Months};
use std::collections::HashMap;

// --- 祝日ロジック ---
struct HolidayLogic;

impl HolidayLogic {
    fn get_japanese_holidays(year: i32) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert(format!("{}-01-01", year), "元日".to_string());
        h.insert(format!("{}-02-11", year), "建国記念の日".to_string());
        h.insert(format!("{}-02-23", year), "天皇誕生日".to_string());
        h.insert(format!("{}-04-29", year), "昭和の日".to_string());
        h.insert(format!("{}-05-03", year), "憲法記念日".to_string());
        h.insert(format!("{}-05-04", year), "みどりの日".to_string());
        h.insert(format!("{}-05-05", year), "こどもの日".to_string());
        h.insert(format!("{}-08-11", year), "山の日".to_string());
        h.insert(format!("{}-11-03", year), "文化の日".to_string());
        h.insert(format!("{}-11-23", year), "勤労感謝の日".to_string());

        let spring = (20.8431 + 0.242194 * (year - 1980) as f64 - ((year - 1980) / 4) as f64) as u32;
        h.insert(format!("{}-03-{:02}", year, spring), "春分の日".to_string());
        let autumn = (23.2488 + 0.242194 * (year - 1980) as f64 - ((year - 1980) / 4) as f64) as u32;
        h.insert(format!("{}-09-{:02}", year, autumn), "秋分の日".to_string());
        h
    }
}

// --- アプリケーション状態 ---
#[derive(PartialEq)]
enum ClockType { Digital, Analog }

struct ClondarApp {
    clock_type: ClockType,
    is_24h: bool,
    show_seconds: bool,
    show_yearly: bool,
    yearly_view_year: i32,
    is_dark: bool,
    is_always_on_top: bool,
    last_update: chrono::DateTime<Local>,
    view_date: NaiveDate,
}

impl Default for ClondarApp {
    fn default() -> Self {
        let now = Local::now();
        Self {
            clock_type: ClockType::Digital,
            is_24h: true,
            show_seconds: true,
            show_yearly: false,
            yearly_view_year: now.year(),
            is_dark: true,
            is_always_on_top: false,
            last_update: now,
            view_date: NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap(),
        }
    }
}

impl eframe::App for ClondarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.last_update = Local::now();
        ctx.request_repaint_after(std::time::Duration::from_millis(50));

        // テーマ設定
        let mut visuals = if self.is_dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        // ホバー色などの微調整
        visuals.widgets.hovered.bg_fill = if self.is_dark {
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20)
        } else {
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 20)
        };
        ctx.set_visuals(visuals);

        let bg_color = if self.is_dark {
            egui::Color32::from_rgba_unmultiplied(10, 10, 15, 245)
        } else {
            egui::Color32::from_rgba_unmultiplied(245, 245, 250, 245)
        };

        let panel_frame = egui::Frame {
            fill: bg_color,
            rounding: egui::Rounding::same(16.0),
            stroke: egui::Stroke::new(1.0, if self.is_dark { egui::Color32::from_white_alpha(30) } else { egui::Color32::from_black_alpha(30) }),
            inner_margin: egui::Margin::same(30.0),
            ..Default::default()
        };

        let text_color = if self.is_dark { egui::Color32::WHITE } else { egui::Color32::from_rgb(30, 30, 40) };

        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            panel_frame.show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    // --- 左半分: 時計セクション ---
                    ui.allocate_ui_with_layout(
                        egui::vec2(450.0, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                ui.selectable_value(&mut self.clock_type, ClockType::Digital, "DIGITAL");
                                ui.selectable_value(&mut self.clock_type, ClockType::Analog, "ANALOG");
                            });

                            // 時計表示エリア (上下中央寄せ)
                            ui.allocate_ui_with_layout(
                                egui::vec2(ui.available_width(), 350.0),
                                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                                |ui| {
                                    match self.clock_type {
                                        ClockType::Digital => self.draw_digital_clock(ui, text_color),
                                        ClockType::Analog => self.draw_analog_clock(ui, text_color),
                                    }
                                }
                            );

                            // 下部設定エリア
                            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                                ui.add_space(10.0);
                                ui.horizontal(|ui| {
                                    // トグルボタン形式
                                    ui.toggle_value(&mut self.is_24h, "24H");
                                    ui.toggle_value(&mut self.show_seconds, "SEC");

                                    if ui.button(if self.is_dark { "☀" } else { "🌙" }).on_hover_text("テーマ切替").clicked() {
                                        self.is_dark = !self.is_dark;
                                    }

                                    let pin_label = if self.is_always_on_top { "📌" } else { "📍" };
                                    if ui.button(pin_label).on_hover_text("常時最前面").clicked() {
                                        self.is_always_on_top = !self.is_always_on_top;
                                        let level = if self.is_always_on_top { egui::WindowLevel::AlwaysOnTop } else { egui::WindowLevel::Normal };
                                        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(level));
                                    }

                                    if ui.button("EXIT").clicked() {
                                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                    }
                                });
                            });
                        }
                    );

                    ui.add_space(30.0);
                    ui.separator();
                    ui.add_space(30.0);

                    // --- 右半分: カレンダーセクション ---
                    ui.vertical(|ui| {
                        self.draw_calendar(ui, text_color);
                    });
                });
            });
        });

        if self.show_yearly {
            self.draw_yearly_window(ctx, text_color);
        }
    }
}

impl ClondarApp {
    fn draw_digital_clock(&self, ui: &mut egui::Ui, color: egui::Color32) {
        let time = self.last_update;
        let format = if self.show_seconds {
            if self.is_24h { "%H:%M:%S" } else { "%I:%M:%S %p" }
        } else {
            if self.is_24h { "%H:%M" } else { "%I:%M %p" }
        };
        let time_str = time.format(format).to_string();

        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new(time_str)
                .font(egui::FontId::new(90.0, egui::FontFamily::Name("impact".into())))
                .strong()
                .color(color));
        });
    }

    fn draw_analog_clock(&self, ui: &mut egui::Ui, color: egui::Color32) {
        // サイズを220pxに縮小して収まりを改善
        let (rect, _response) = ui.allocate_at_least(egui::vec2(220.0, 220.0), egui::Sense::hover());
        let painter = ui.painter_at(rect);
        let center = rect.center();
        let radius = rect.width() / 2.0 - 10.0;

        for i in 0..12 {
            let angle = (i as f32 * 30.0).to_radians();
            let length = if i % 3 == 0 { 16.0 } else { 10.0 };
            let start = center + egui::vec2(angle.sin(), -angle.cos()) * (radius - length);
            let end = center + egui::vec2(angle.sin(), -angle.cos()) * radius;
            painter.line_segment([start, end], egui::Stroke::new(if i % 3 == 0 { 4.0 } else { 2.0 }, egui::Color32::GRAY));
        }

        let time = self.last_update;
        let sec = time.second() as f32 + time.nanosecond() as f32 / 1e9;
        let min = time.minute() as f32 + sec / 60.0;
        let hour = (time.hour() % 12) as f32 + min / 60.0;

        painter.line_segment([center, center + egui::vec2((hour * 30.0).to_radians().sin(), -(hour * 30.0).to_radians().cos()) * (radius * 0.5)], egui::Stroke::new(6.0, color));
        painter.line_segment([center, center + egui::vec2((min * 6.0).to_radians().sin(), -(min * 6.0).to_radians().cos()) * (radius * 0.8)], egui::Stroke::new(3.0, egui::Color32::GRAY));
        if self.show_seconds {
            painter.line_segment([center, center + egui::vec2((sec * 6.0).to_radians().sin(), -(sec * 6.0).to_radians().cos()) * (radius - 5.0)], egui::Stroke::new(1.5, egui::Color32::RED));
        }
        painter.circle_filled(center, 5.0, color);
    }

    fn draw_calendar(&mut self, ui: &mut egui::Ui, color: egui::Color32) {
        let year = self.view_date.year();
        let month = self.view_date.month();

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("{}年 {}月", year, month)).size(26.0).strong().color(color));
            ui.add_space(20.0);
            if ui.button("◀").clicked() { self.view_date = self.view_date.checked_sub_months(Months::new(1)).unwrap(); }
            if ui.button("今月").clicked() {
                let now = Local::now();
                self.view_date = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();
            }
            if ui.button("▶").clicked() { self.view_date = self.view_date.checked_add_months(Months::new(1)).unwrap(); }
            ui.add_space(10.0);
            if ui.button("今年").clicked() {
                self.yearly_view_year = year;
                self.show_yearly = true;
            }
        });
        ui.add_space(20.0);
        self.draw_mini_calendar(ui, year, month, color, true);
    }

    fn draw_mini_calendar(&self, ui: &mut egui::Ui, year: i32, month: u32, color: egui::Color32, large: bool) {
        let holidays = HolidayLogic::get_japanese_holidays(year);
        let now = Local::now();
        let cell_size = if large { 48.0 } else { 24.0 };
        let text_size = if large { 18.0 } else { 10.0 };

        egui::Grid::new(format!("cal_grid_{}_{}", year, month)).spacing([8.0, 8.0]).show(ui, |ui| {
            let weekdays = ["日", "月", "火", "水", "木", "金", "土"];
            for (i, wd) in weekdays.iter().enumerate() {
                let c = if i == 0 { egui::Color32::RED } else if i == 6 { egui::Color32::BLUE } else { egui::Color32::GRAY };
                ui.vertical_centered(|ui| { ui.label(egui::RichText::new(*wd).color(c).strong().size(text_size)); });
            }
            ui.end_row();

            let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let start_wd = first_day.weekday().num_days_from_sunday();
            for _ in 0..start_wd { ui.label(""); }

            let mut curr = first_day;
            while curr.month() == month {
                let date_str = format!("{}-{:02}-{:02}", year, month, curr.day());
                let holiday_name = holidays.get(&date_str);
                let is_today = curr.day() == now.day() && curr.month() == now.month() && year == now.year();

                let mut cell_color = if self.is_dark { egui::Color32::from_white_alpha(10) } else { egui::Color32::from_black_alpha(10) };
                let mut text_color = color;

                if is_today {
                    cell_color = egui::Color32::from_rgb(60, 130, 246);
                    text_color = egui::Color32::WHITE;
                } else if holiday_name.is_some() || curr.weekday() == Weekday::Sun {
                    text_color = egui::Color32::RED;
                } else if curr.weekday() == Weekday::Sat {
                    text_color = egui::Color32::BLUE;
                }

                let frame = egui::Frame::default().fill(cell_color).rounding(egui::Rounding::same(8.0)).inner_margin(egui::Margin::same(2.0));
                let response = frame.show(ui, |ui| {
                    ui.set_min_size(egui::vec2(cell_size, cell_size));
                    ui.centered_and_justified(|ui| { ui.label(egui::RichText::new(format!("{}", curr.day())).size(text_size).color(text_color)); });
                }).response;

                if let Some(name) = holiday_name { response.on_hover_text(name); }
                if (curr.weekday().num_days_from_sunday() + 1) % 7 == 0 { ui.end_row(); }
                curr = curr.succ_opt().unwrap_or(curr);
            }
        });
    }

    fn draw_yearly_window(&mut self, ctx: &egui::Context, color: egui::Color32) {
        let bg_color = if self.is_dark { egui::Color32::from_rgba_unmultiplied(15, 15, 20, 255) } else { egui::Color32::from_rgba_unmultiplied(240, 240, 245, 255) };

        egui::Window::new("年間カレンダー")
            .collapsible(false)
            .resizable(false)
            .fixed_size([1050.0, 750.0])
            .frame(egui::Frame::window(&ctx.style()).fill(bg_color))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(egui::RichText::new(format!("{}年 年間カレンダー", self.yearly_view_year)).color(color));
                    ui.add_space(20.0);
                    if ui.button("前年").clicked() { self.yearly_view_year -= 1; }
                    if ui.button("今年").clicked() { self.yearly_view_year = Local::now().year(); }
                    if ui.button("翌年").clicked() { self.yearly_view_year += 1; }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("閉じる").clicked() { self.show_yearly = false; }
                    });
                });
                ui.add_space(20.0);

                // 4列×3行で表示して1画面に収める
                egui::Grid::new("yearly_grid").spacing([25.0, 20.0]).show(ui, |ui| {
                    for m in 1..=12 {
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(format!("{}月", m)).size(16.0).strong().color(egui::Color32::from_rgb(60, 130, 246)));
                            self.draw_mini_calendar(ui, self.yearly_view_year, m, color, false);
                        });
                        if m % 4 == 0 { ui.end_row(); }
                    }
                });
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 600.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Clondar Pro",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            #[cfg(target_os = "windows")]
            {
                if let Ok(data) = std::fs::read("C:\\Windows\\Fonts\\impact.ttf") {
                    fonts.font_data.insert("impact".to_owned(), egui::FontData::from_owned(data));
                    fonts.families.insert(egui::FontFamily::Name("impact".into()), vec!["impact".into()]);
                }
                if let Ok(data) = std::fs::read("C:\\Windows\\Fonts\\msgothic.ttc") {
                    fonts.font_data.insert("japanese".to_owned(), egui::FontData::from_owned(data));
                    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "japanese".to_owned());
                }
            }
            cc.egui_ctx.set_fonts(fonts);
            Box::new(ClondarApp::default())
        }),
    )
}
