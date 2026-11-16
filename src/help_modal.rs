//! # ヘルプ＆アプリ情報モーダル (`help_modal.rs`)
//!
//! アプリケーションの概要、バージョン表示、および各種機能の操作ガイドを閲覧するモーダルです。

use crate::icons;
use crate::theme::{show_modal_overlay, Palette};
use eframe::egui;

struct GuideItem {
    title: &'static str,
    description: &'static str,
}

const GUIDE_ITEMS: &[GuideItem] = &[
    GuideItem {
        title: "時計機能",
        description: "デジタル表示（Impact系フォント）とアナログ表示（滑らかなスイープ運針）を切り替え可能。12H/24H表記や秒表示のON/OFFもワンクリックで変更できます。",
    },
    GuideItem {
        title: "カレンダー機能",
        description: "日本の祝日（振替休日・国民の休日）に対応。月の長さに影響されない安定した6週間レイアウトを採用し、年間カレンダー表示や祝日カスタマイズも備えています。",
    },
    GuideItem {
        title: "最前面ピン留め",
        description: "タイトルバー右上のピンアイコンで、他のウィンドウに隠れず常に最前面に表示できます。作業中も常に時刻を確認可能です。",
    },
    GuideItem {
        title: "透過 & ダークモード",
        description: "太陽アイコンで背景透過モードへ切り替え。月/太陽ボタンでライト/ダークテーマを瞬時に切り替えられ、壁紙に美しく調和します。",
    },
    GuideItem {
        title: "自由なウィンドウ配置",
        description: "タイトルバーや余白をドラッグしてデスクトップの好みの位置へ配置できます。マルチモニター環境でも前回の位置を記憶して自動復元します。",
    },
    GuideItem {
        title: "ショートカット & トレイ",
        description: "「Esc」キーや閉じるボタンでトレイへ格納できます。タスクバーのシステムトレイアイコンを右クリックして最前面固定や位置リセットも可能です。",
    },
];

/// ヘルプモーダルを描画します。閉じる要求があった場合は `true` を返します。
pub fn render_help_modal(ctx: &egui::Context, pal: &Palette) -> bool {
    let mut should_close = false;
    let modal_size = egui::vec2(780.0, 504.0);

    let clicked_outside = show_modal_overlay(ctx, "help_modal", modal_size, pal, |ui| {
        // ヘッダー
        egui::Frame::new()
            .fill(pal.sub_surface)
            .inner_margin(egui::Margin::symmetric(22, 14))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (icon_rect, _) =
                        ui.allocate_exact_size(egui::vec2(34.0, 34.0), egui::Sense::hover());
                    ui.painter().rect_filled(
                        icon_rect,
                        egui::CornerRadius::same(10),
                        egui::Color32::from_rgba_unmultiplied(59, 130, 246, 30),
                    );
                    icons::draw_help_icon(ui.painter(), icon_rect, pal.accent);

                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Clondar Pro")
                                    .size(20.0)
                                    .color(pal.text_primary),
                            );
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgba_unmultiplied(59, 130, 246, 28))
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgba_unmultiplied(59, 130, 246, 70),
                                ))
                                .corner_radius(egui::CornerRadius::same(10))
                                .inner_margin(egui::Margin::symmetric(8, 2))
                                .show(ui, |ui| {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "v{}",
                                            env!("CARGO_PKG_VERSION")
                                        ))
                                        .size(11.5)
                                        .color(pal.accent),
                                    );
                                });
                        });
                        ui.label(
                            egui::RichText::new(
                                "デスクトップ常駐型 ミニマル時計 ＆ カレンダーウィジェット",
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

        // コンテンツ領域
        egui::ScrollArea::vertical()
            .max_height(378.0)
            .show(ui, |ui| {
                egui::Frame::new()
                    .inner_margin(egui::Margin::symmetric(22, 12))
                    .show(ui, |ui| {
                        // コンセプト説明カード
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgba_unmultiplied(59, 130, 246, 18))
                            .stroke(egui::Stroke::new(
                                1.0,
                                egui::Color32::from_rgba_unmultiplied(59, 130, 246, 45),
                            ))
                            .corner_radius(egui::CornerRadius::same(12))
                            .inner_margin(egui::Margin::same(11))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("Clondar Pro は、Windows デスクトップに美しく溶け込む、透過・枠なし・影なしのミニマルなデザインを追求したウィジェット型時計＆カレンダーアプリです。純粋な Rust ネイティブ描画（egui + OpenGL）により、低メモリ・軽快なレスポンスで常駐動作します。")
                                        .size(12.0)
                                        .color(pal.text_secondary),
                                );
                            });

                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("主な機能と使い方")
                                .size(12.5)
                                .color(pal.text_muted),
                        );
                        ui.add_space(6.0);

                        // 2列グリッド（全6カードの高さを完全統一）
                        let col_gap = 12.0;
                        let col_w = (ui.available_width() - col_gap) * 0.5;
                        let card_h = 84.0;
                        for pair in GUIDE_ITEMS.chunks(2) {
                            ui.horizontal_top(|ui| {
                                ui.spacing_mut().item_spacing.x = 0.0;
                                for (idx, item) in pair.iter().enumerate() {
                                    if idx > 0 {
                                        ui.add_space(col_gap);
                                    }
                                    ui.allocate_ui_with_layout(
                                        egui::vec2(col_w, card_h),
                                        egui::Layout::top_down(egui::Align::LEFT),
                                        |ui| {
                                            ui.set_min_size(egui::vec2(col_w, card_h));
                                            ui.set_max_size(egui::vec2(col_w, card_h));
                                            egui::Frame::new()
                                                .fill(pal.sub_surface)
                                                .stroke(egui::Stroke::new(1.0, pal.border))
                                                .corner_radius(egui::CornerRadius::same(12))
                                                .inner_margin(egui::Margin::same(10))
                                                .show(ui, |ui| {
                                                    let inner_w = col_w - 22.0;
                                                    let inner_h = card_h - 22.0;
                                                    ui.set_min_size(egui::vec2(inner_w, inner_h));
                                                    ui.set_max_size(egui::vec2(inner_w, inner_h));
                                                    ui.label(
                                                        egui::RichText::new(item.title)
                                                            .size(13.0)
                                                            .color(pal.accent),
                                                    );
                                                    ui.add_space(3.0);
                                                    ui.label(
                                                        egui::RichText::new(item.description)
                                                            .size(11.5)
                                                            .color(pal.text_muted),
                                                    );
                                                });
                                        },
                                    );
                                }
                            });
                            ui.add_space(8.0);
                        }
                    });
            });

        // フッター
        egui::Frame::new()
            .fill(pal.sub_surface)
            .inner_margin(egui::Margin::symmetric(22, 10))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Clondar Pro © 2026")
                            .size(12.0)
                            .color(pal.text_muted),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("閉じる")
                                        .size(12.5)
                                        .color(egui::Color32::WHITE),
                                )
                                .fill(pal.accent)
                                .corner_radius(egui::CornerRadius::same(8))
                                .min_size(egui::vec2(72.0, 28.0)),
                            )
                            .clicked()
                        {
                            should_close = true;
                        }
                    });
                });
            });
    });

    should_close || clicked_outside
}
