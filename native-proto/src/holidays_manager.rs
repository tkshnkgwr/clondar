//! # 祝日設定マネージャーモーダル (`holidays_manager.rs`)
//!
//! 固定祝日のビジュアル追加・削除、`common_lib::compute_diff` による内蔵デフォルト設定との
//! LCS 差分表示、`common_lib::count_occurrences` による統計集計、および JSON 生データの直接編集と
//! 厳格なスキーマ検証（`holidays::validate_and_parse_config`）を提供します。

use crate::config::{self, DEFAULT_HOLIDAYS_JSON};
use crate::holidays::{self, HolidayConfig};
use crate::icons;
use crate::theme::{show_modal_overlay, Palette};
use eframe::egui;

const KEYWORDS: [&str; 4] = ["の日", "天皇誕生日", "振替休日", "国民の休日"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManagerTab {
    Edit,
    Diff,
    Raw,
}

/// 祝日設定マネージャーの編集状態。
pub struct HolidaysManagerState {
    active_tab: ManagerTab,
    editing_config: HolidayConfig,
    raw_json_text: String,
    new_fixed_date: String,
    new_fixed_name: String,
    stats: Vec<(&'static str, usize)>,
    diff_parts: Vec<common_lib::DiffPart>,
    status_message: Option<(String, bool)>, // (メッセージ, is_error)
    confirm_reset: bool,
}

impl HolidaysManagerState {
    /// 現在の設定ファイル（またはデフォルト）を読み込んで編集状態を初期化します。
    pub fn load() -> Self {
        let raw_json_text = config::load_holidays_json_text();
        let editing_config =
            holidays::validate_and_parse_config(&raw_json_text).unwrap_or_else(|_| {
                holidays::validate_and_parse_config(DEFAULT_HOLIDAYS_JSON)
                    .expect("デフォルト祝日JSONは常に有効")
            });

        let mut state = Self {
            active_tab: ManagerTab::Edit,
            editing_config,
            raw_json_text,
            new_fixed_date: String::new(),
            new_fixed_name: String::new(),
            stats: Vec::new(),
            diff_parts: Vec::new(),
            status_message: None,
            confirm_reset: false,
        };
        state.recompute_analysis();
        state
    }

    fn recompute_analysis(&mut self) {
        self.stats = KEYWORDS
            .iter()
            .map(|&w| (w, common_lib::count_occurrences(&self.raw_json_text, w)))
            .collect();
        self.diff_parts = common_lib::compute_diff(DEFAULT_HOLIDAYS_JSON, &self.raw_json_text);
    }

    fn sync_raw_from_config(&mut self) {
        if let Ok(pretty) = serde_json::to_string_pretty(&self.editing_config) {
            self.raw_json_text = pretty;
            self.recompute_analysis();
        }
    }

    fn switch_tab(&mut self, next: ManagerTab) {
        if self.active_tab == next {
            return;
        }
        if self.active_tab == ManagerTab::Raw {
            match holidays::validate_and_parse_config(&self.raw_json_text) {
                Ok(parsed) => {
                    self.editing_config = parsed;
                    self.recompute_analysis();
                    self.status_message = None;
                }
                Err(err) => {
                    self.status_message = Some((format!("JSON エラー: {err}"), true));
                    return;
                }
            }
        } else if next == ManagerTab::Raw || next == ManagerTab::Diff {
            self.sync_raw_from_config();
        }
        self.active_tab = next;
    }

    fn add_fixed_holiday(&mut self) {
        let date_str = self.new_fixed_date.trim().to_string();
        let name_str = self.new_fixed_name.trim().to_string();
        if date_str.is_empty() || name_str.is_empty() {
            self.status_message = Some(("日付と祝日名の両方を入力してください。".into(), true));
            return;
        }

        if let Err(err) = validate_mmdd_input(&date_str) {
            self.status_message = Some((err, true));
            return;
        }

        self.editing_config.fixed.insert(date_str, name_str);
        self.new_fixed_date.clear();
        self.new_fixed_name.clear();
        self.status_message = None;
        self.sync_raw_from_config();
    }

    fn remove_fixed_holiday(&mut self, key: &str) {
        self.editing_config.fixed.remove(key);
        self.status_message = None;
        self.sync_raw_from_config();
    }

    fn perform_reset(&mut self) -> Option<HolidayConfig> {
        match config::reset_holidays_json_text() {
            Ok(default_json) => {
                if let Ok(parsed) = holidays::validate_and_parse_config(&default_json) {
                    self.editing_config = parsed.clone();
                    self.raw_json_text = default_json;
                    self.recompute_analysis();
                    self.confirm_reset = false;
                    self.status_message =
                        Some(("内蔵の初期設定に復元し、保存しました。".into(), false));
                    return Some(parsed);
                }
            }
            Err(e) => {
                self.status_message = Some((format!("初期化に失敗しました: {e}"), true));
            }
        }
        self.confirm_reset = false;
        None
    }

    fn perform_save(&mut self) -> Result<HolidayConfig, ()> {
        let json_to_save = if self.active_tab == ManagerTab::Raw {
            self.raw_json_text.clone()
        } else {
            serde_json::to_string_pretty(&self.editing_config).map_err(|e| {
                self.status_message = Some((format!("シリアライズ失敗: {e}"), true));
            })?
        };

        let validated = holidays::validate_and_parse_config(&json_to_save).map_err(|e| {
            self.status_message = Some((format!("検証エラー: {e}"), true));
        })?;

        config::save_holidays_json_text(&json_to_save).map_err(|e| {
            self.status_message = Some((e, true));
        })?;

        self.editing_config = validated.clone();
        self.raw_json_text = json_to_save;
        self.recompute_analysis();
        Ok(validated)
    }
}

/// `MM-DD` 入力値のフォーマットおよび各月日数上限チェックを行います。
pub fn validate_mmdd_input(input: &str) -> Result<(), String> {
    let bytes = input.as_bytes();
    if bytes.len() != 5
        || bytes[2] != b'-'
        || !bytes[0].is_ascii_digit()
        || !bytes[1].is_ascii_digit()
        || !bytes[3].is_ascii_digit()
        || !bytes[4].is_ascii_digit()
    {
        return Err("日付は MM-DD 形式（例: 07-07）で入力してください。".into());
    }

    let month: u32 = input[0..2].parse().unwrap_or(0);
    let day: u32 = input[3..5].parse().unwrap_or(0);
    if !(1..=12).contains(&month) {
        return Err("月は 01〜12 の範囲で入力してください。".into());
    }
    let max_days = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if day < 1 || day > max_days[month as usize] {
        return Err(format!(
            "{}月の日付は 01〜{:02} の範囲で入力してください。",
            month, max_days[month as usize]
        ));
    }
    Ok(())
}

/// 祝日設定マネージャーモーダルを描画します。
///
/// 戻り値 `(should_close, updated_config)` を返します。
pub fn render_holidays_manager(
    ctx: &egui::Context,
    state: &mut HolidaysManagerState,
    pal: &Palette,
) -> (bool, Option<HolidayConfig>) {
    let mut should_close = false;
    let mut updated_config: Option<HolidayConfig> = None;
    let modal_size = egui::vec2(960.0, 490.0);

    let clicked_outside = show_modal_overlay(
        ctx,
        "holidays_manager_modal",
        modal_size,
        pal,
        |ui| {
            // ヘッダー
            egui::Frame::new()
            .fill(pal.sub_surface)
            .inner_margin(egui::Margin::symmetric(22, 14))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("祝日設定マネージャー")
                                .size(20.0)
                                .color(pal.text_primary),
                        );
                        ui.label(
                            egui::RichText::new(
                                "祝日の追加・削除、内蔵設定との差分比較、および JSON 直接編集が可能です",
                            )
                            .size(12.0)
                            .color(pal.text_muted),
                        );
                    });

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

            // 本体（左：統計サイドバー / 右：タブ＆編集ビュー）
            let body_h = 414.0;
            let left_pane_w = 264.0;
            let right_pane_w = modal_size.x - left_pane_w;

            ui.allocate_ui_with_layout(
            egui::vec2(modal_size.x, body_h),
            egui::Layout::left_to_right(egui::Align::TOP),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                // 左サイドバー（明示的に top_down レイアウトで縦積み）
                ui.allocate_ui_with_layout(
                    egui::vec2(left_pane_w, body_h),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        egui::Frame::new()
                            .fill(pal.sub_surface)
                            .inner_margin(egui::Margin::same(16))
                            .show(ui, |ui| {
                                let inner_w = left_pane_w - 32.0;
                                let inner_h = body_h - 32.0;
                                ui.set_min_size(egui::vec2(inner_w, inner_h));
                                ui.set_max_size(egui::vec2(inner_w, inner_h));

                                ui.label(
                                    egui::RichText::new("設定統計情報")
                                        .size(12.5)
                                        .color(pal.text_muted),
                                );
                                ui.add_space(10.0);

                                for &(word, count) in &state.stats {
                                    egui::Frame::new()
                                        .fill(pal.modal_bg)
                                        .stroke(egui::Stroke::new(1.0, pal.border))
                                        .corner_radius(egui::CornerRadius::same(10))
                                        .inner_margin(egui::Margin::symmetric(12, 9))
                                        .show(ui, |ui| {
                                            ui.set_min_width(inner_w - 26.0);
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    egui::RichText::new(format!("「{word}」"))
                                                        .size(12.5)
                                                        .color(pal.text_secondary),
                                                );
                                                ui.with_layout(
                                                    egui::Layout::right_to_left(
                                                        egui::Align::Center,
                                                    ),
                                                    |ui| {
                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "{count} 件"
                                                            ))
                                                            .size(12.5)
                                                            .color(pal.accent),
                                                        );
                                                    },
                                                );
                                            });
                                        });
                                    ui.add_space(6.0);
                                }

                                ui.add_space(14.0);
                                egui::Frame::new()
                                    .fill(egui::Color32::from_rgba_unmultiplied(59, 130, 246, 18))
                                    .stroke(egui::Stroke::new(
                                        1.0,
                                        egui::Color32::from_rgba_unmultiplied(59, 130, 246, 45),
                                    ))
                                    .corner_radius(egui::CornerRadius::same(10))
                                    .inner_margin(egui::Margin::same(10))
                                    .show(ui, |ui| {
                                        ui.set_min_width(inner_w - 22.0);
                                        ui.label(
                                            egui::RichText::new("共有クレート連携")
                                                .size(12.0)
                                                .color(pal.accent),
                                        );
                                        ui.add_space(4.0);
                                        ui.label(
                                            egui::RichText::new(
                                                "差分検出に common_lib::compute_diff (LCS)、集計に count_occurrences を使用しています。",
                                            )
                                            .size(11.5)
                                            .color(pal.text_muted),
                                        );
                                    });
                            });
                    },
                );

                // 右メインエリア（明示的に top_down レイアウトで縦積み）
                ui.allocate_ui_with_layout(
                    egui::vec2(right_pane_w, body_h),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        egui::Frame::new()
                            .inner_margin(egui::Margin::same(16))
                            .show(ui, |ui| {
                                let right_inner_w = right_pane_w - 32.0;
                                ui.set_min_width(right_inner_w);
                                ui.set_max_width(right_inner_w);

                                // タブバー＆アクションボタン
                                ui.horizontal(|ui| {
                                    let tabs_inner_w = 96.0 * 3.0 + 4.0 * 2.0;
                                    ui.allocate_ui_with_layout(
                                        egui::vec2(tabs_inner_w + 8.0, 32.0),
                                        egui::Layout::top_down(egui::Align::LEFT),
                                        |ui| {
                                            egui::Frame::new()
                                                .fill(pal.pill_bg)
                                                .corner_radius(egui::CornerRadius::same(10))
                                                .inner_margin(egui::Margin::same(3))
                                                .show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.spacing_mut().item_spacing.x = 4.0;
                                                        for (tab, label) in [
                                                            (ManagerTab::Edit, "祝日の編集"),
                                                            (ManagerTab::Diff, "変更差分 (Diff)"),
                                                            (ManagerTab::Raw, "設定生データ"),
                                                        ] {
                                                            let active = state.active_tab == tab;
                                                            let fill = if active {
                                                                pal.pill_active_bg
                                                            } else {
                                                                egui::Color32::TRANSPARENT
                                                            };
                                                            let color = if active {
                                                                pal.accent
                                                            } else {
                                                                pal.text_muted
                                                            };
                                                            if ui
                                                                .add(
                                                                    egui::Button::new(
                                                                        egui::RichText::new(label)
                                                                            .size(12.5)
                                                                            .color(color),
                                                                    )
                                                                    .fill(fill)
                                                                    .corner_radius(
                                                                        egui::CornerRadius::same(7),
                                                                    )
                                                                    .min_size(egui::vec2(
                                                                        96.0, 26.0,
                                                                    )),
                                                                )
                                                                .clicked()
                                                            {
                                                                state.switch_tab(tab);
                                                            }
                                                        }
                                                    });
                                                });
                                        },
                                    );

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if ui
                                                .add(
                                                    egui::Button::new(
                                                        egui::RichText::new("保存して適用")
                                                            .size(12.5)
                                                            .color(egui::Color32::WHITE),
                                                    )
                                                    .fill(pal.accent)
                                                    .corner_radius(egui::CornerRadius::same(8))
                                                    .min_size(egui::vec2(102.0, 28.0)),
                                                )
                                                .clicked()
                                            {
                                                if let Ok(saved_cfg) = state.perform_save() {
                                                    updated_config = Some(saved_cfg);
                                                    should_close = true;
                                                }
                                            }

                                            ui.add_space(6.0);
                                            if ui
                                                .add(
                                                    egui::Button::new(
                                                        egui::RichText::new("初期化")
                                                            .size(12.5)
                                                            .color(pal.text_secondary),
                                                    )
                                                    .fill(pal.pill_bg)
                                                    .corner_radius(egui::CornerRadius::same(8))
                                                    .min_size(egui::vec2(64.0, 28.0)),
                                                )
                                                .clicked()
                                            {
                                                state.confirm_reset = true;
                                            }
                                        },
                                    );
                                });

                                // 初期化確認バー
                                if state.confirm_reset {
                                    ui.add_space(8.0);
                                    egui::Frame::new()
                                        .fill(egui::Color32::from_rgba_unmultiplied(
                                            245, 158, 11, 25,
                                        ))
                                        .stroke(egui::Stroke::new(
                                            1.0,
                                            egui::Color32::from_rgba_unmultiplied(
                                                245, 158, 11, 100,
                                            ),
                                        ))
                                        .corner_radius(egui::CornerRadius::same(8))
                                        .inner_margin(egui::Margin::symmetric(12, 8))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    egui::RichText::new(
                                                        "内蔵の初期設定へ復元して上書きしますか？",
                                                    )
                                                    .size(12.0)
                                                    .color(pal.text_primary),
                                                );
                                                ui.with_layout(
                                                    egui::Layout::right_to_left(
                                                        egui::Align::Center,
                                                    ),
                                                    |ui| {
                                                        if ui
                                                            .add(
                                                                egui::Button::new(
                                                                    egui::RichText::new(
                                                                        "キャンセル",
                                                                    )
                                                                    .size(12.0)
                                                                    .color(pal.text_secondary),
                                                                )
                                                                .fill(pal.pill_bg)
                                                                .corner_radius(
                                                                    egui::CornerRadius::same(6),
                                                                ),
                                                            )
                                                            .clicked()
                                                        {
                                                            state.confirm_reset = false;
                                                        }
                                                        ui.add_space(4.0);
                                                        if ui
                                                            .add(
                                                                egui::Button::new(
                                                                    egui::RichText::new("復元する")
                                                                        .size(12.0)
                                                                        .color(
                                                                            egui::Color32::WHITE,
                                                                        ),
                                                                )
                                                                .fill(pal.danger_hover)
                                                                .corner_radius(
                                                                    egui::CornerRadius::same(6),
                                                                ),
                                                            )
                                                            .clicked()
                                                        {
                                                            if let Some(cfg) = state.perform_reset()
                                                            {
                                                                updated_config = Some(cfg);
                                                            }
                                                        }
                                                    },
                                                );
                                            });
                                        });
                                }

                                // エラー・通知バナー
                                if let Some((msg, is_err)) = &state.status_message {
                                    ui.add_space(6.0);
                                    let color = if *is_err {
                                        pal.sunday_holiday
                                    } else {
                                        egui::Color32::from_rgb(34, 197, 94)
                                    };
                                    ui.label(egui::RichText::new(msg).size(12.0).color(color));
                                }

                                ui.add_space(10.0);

                                // タブ本体
                                match state.active_tab {
                                    ManagerTab::Edit => render_edit_tab(ui, state, pal),
                                    ManagerTab::Diff => render_diff_tab(ui, state, pal),
                                    ManagerTab::Raw => render_raw_tab(ui, state, pal),
                                }
                            });
                    },
                );
            },
        );
        },
    );

    (should_close || clicked_outside, updated_config)
}

fn render_edit_tab(ui: &mut egui::Ui, state: &mut HolidaysManagerState, pal: &Palette) {
    let total_w = ui.available_width();
    let form_w = 220.0;
    let gap = 12.0;
    let list_w = (total_w - form_w - gap).max(260.0);

    ui.allocate_ui_with_layout(
        egui::vec2(total_w, 322.0),
        egui::Layout::left_to_right(egui::Align::TOP),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // 左側：固定祝日一覧（top_down 明示）
            ui.allocate_ui_with_layout(
                egui::vec2(list_w, 322.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    egui::Frame::new()
                        .fill(pal.sub_surface)
                        .stroke(egui::Stroke::new(1.0, pal.border))
                        .corner_radius(egui::CornerRadius::same(12))
                        .inner_margin(egui::Margin::same(12))
                        .show(ui, |ui| {
                            ui.set_min_size(egui::vec2(list_w - 26.0, 296.0));
                            ui.set_max_size(egui::vec2(list_w - 26.0, 296.0));

                            ui.label(
                                egui::RichText::new(format!(
                                    "現在の固定祝日一覧 ({}件)",
                                    state.editing_config.fixed.len()
                                ))
                                .size(12.5)
                                .color(pal.text_muted),
                            );
                            ui.add_space(8.0);

                            let mut entries: Vec<(String, String)> = state
                                .editing_config
                                .fixed
                                .iter()
                                .map(|(k, v)| (k.clone(), v.clone()))
                                .collect();
                            entries.sort_by(|a, b| a.0.cmp(&b.0));

                            let mut to_remove: Option<String> = None;
                            egui::ScrollArea::vertical()
                                .max_height(262.0)
                                .show(ui, |ui| {
                                    for (date, name) in entries {
                                        ui.horizontal(|ui| {
                                            egui::Frame::new()
                                                .fill(pal.pill_bg)
                                                .corner_radius(egui::CornerRadius::same(6))
                                                .inner_margin(egui::Margin::symmetric(8, 3))
                                                .show(ui, |ui| {
                                                    ui.label(
                                                        egui::RichText::new(&date)
                                                            .monospace()
                                                            .size(12.0)
                                                            .color(pal.text_secondary),
                                                    );
                                                });
                                            ui.add_space(6.0);
                                            ui.label(
                                                egui::RichText::new(&name)
                                                    .size(12.5)
                                                    .color(pal.text_primary),
                                            );

                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    if ui
                                                        .add(
                                                            egui::Button::new(
                                                                egui::RichText::new("削除")
                                                                    .size(12.0)
                                                                    .color(pal.sunday_holiday),
                                                            )
                                                            .fill(egui::Color32::TRANSPARENT)
                                                            .corner_radius(
                                                                egui::CornerRadius::same(6),
                                                            ),
                                                        )
                                                        .clicked()
                                                    {
                                                        to_remove = Some(date);
                                                    }
                                                },
                                            );
                                        });
                                        ui.add_space(4.0);
                                    }
                                });

                            if let Some(key) = to_remove {
                                state.remove_fixed_holiday(&key);
                            }
                        });
                },
            );

            ui.add_space(gap);

            // 右側：新規追加フォーム（top_down 明示）
            ui.allocate_ui_with_layout(
                egui::vec2(form_w, 322.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    egui::Frame::new()
                        .fill(pal.sub_surface)
                        .stroke(egui::Stroke::new(1.0, pal.border))
                        .corner_radius(egui::CornerRadius::same(12))
                        .inner_margin(egui::Margin::same(14))
                        .show(ui, |ui| {
                            ui.set_min_size(egui::vec2(form_w - 30.0, 292.0));
                            ui.set_max_size(egui::vec2(form_w - 30.0, 292.0));

                            ui.label(
                                egui::RichText::new("新しい祝日の追加")
                                    .size(13.0)
                                    .color(pal.accent),
                            );
                            ui.add_space(14.0);

                            ui.label(
                                egui::RichText::new("日付 (MM-DD)")
                                    .size(12.0)
                                    .color(pal.text_muted),
                            );
                            ui.add_space(4.0);
                            ui.add(
                                egui::TextEdit::singleline(&mut state.new_fixed_date)
                                    .hint_text("例: 07-07")
                                    .desired_width(f32::INFINITY),
                            );

                            ui.add_space(12.0);
                            ui.label(
                                egui::RichText::new("祝日の名前")
                                    .size(12.0)
                                    .color(pal.text_muted),
                            );
                            ui.add_space(4.0);
                            ui.add(
                                egui::TextEdit::singleline(&mut state.new_fixed_name)
                                    .hint_text("例: 七夕")
                                    .desired_width(f32::INFINITY),
                            );

                            ui.add_space(18.0);
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new("+ リストに追加")
                                            .size(12.5)
                                            .color(egui::Color32::WHITE),
                                    )
                                    .fill(pal.accent)
                                    .corner_radius(egui::CornerRadius::same(8))
                                    .min_size(egui::vec2(ui.available_width(), 32.0)),
                                )
                                .clicked()
                            {
                                state.add_fixed_holiday();
                            }
                        });
                },
            );
        },
    );
}

fn render_diff_tab(ui: &mut egui::Ui, state: &HolidaysManagerState, pal: &Palette) {
    egui::Frame::new()
        .fill(pal.sub_surface)
        .stroke(egui::Stroke::new(1.0, pal.border))
        .corner_radius(egui::CornerRadius::same(12))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_min_size(egui::vec2(ui.available_width(), 300.0));
            egui::ScrollArea::both().max_height(286.0).show(ui, |ui| {
                for part in &state.diff_parts {
                    let (prefix, bg, fg) = match part.diff_type {
                        common_lib::DiffType::Added => (
                            "+",
                            egui::Color32::from_rgba_unmultiplied(34, 197, 94, 28),
                            egui::Color32::from_rgb(74, 222, 128),
                        ),
                        common_lib::DiffType::Removed => (
                            "-",
                            egui::Color32::from_rgba_unmultiplied(239, 68, 68, 28),
                            egui::Color32::from_rgb(248, 113, 113),
                        ),
                        common_lib::DiffType::Unchanged => {
                            (" ", egui::Color32::TRANSPARENT, pal.text_muted)
                        }
                    };
                    egui::Frame::new().fill(bg).show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(format!("{prefix} {}", part.value))
                                .monospace()
                                .size(12.0)
                                .color(fg),
                        );
                    });
                }
            });
        });
}

fn render_raw_tab(ui: &mut egui::Ui, state: &mut HolidaysManagerState, pal: &Palette) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(2, 6, 23))
        .stroke(egui::Stroke::new(1.0, pal.border))
        .corner_radius(egui::CornerRadius::same(12))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_min_size(egui::vec2(ui.available_width(), 300.0));
            egui::ScrollArea::vertical()
                .max_height(286.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut state.raw_json_text)
                            .font(egui::FontId::monospace(12.0))
                            .text_color(egui::Color32::from_rgb(241, 245, 249))
                            .desired_width(f32::INFINITY)
                            .desired_rows(16),
                    );
                });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_mmdd_input_boundaries() {
        assert!(validate_mmdd_input("07-07").is_ok());
        assert!(validate_mmdd_input("02-29").is_ok());
        assert!(validate_mmdd_input("02-30").is_err());
        assert!(validate_mmdd_input("04-31").is_err());
        assert!(validate_mmdd_input("13-01").is_err());
        assert!(validate_mmdd_input("7-7").is_err());
    }
}
