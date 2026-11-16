//! # フォント設定モジュール (`font.rs`)
//!
//! OS 標準の日本語フォント（メイリオ / 游ゴシック / MS ゴシック等）および
//! デジタル時計用のインパクト系フォントを `egui::Context` へ登録します。

use eframe::egui;

/// Windows 等にインストールされているシステムフォントを読み込み、`egui` に登録します。
pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());

    // 日本語ゴシック系フォント候補（小文字でも滲みにくい游ゴシック Medium を最優先）
    let jp_candidates = [
        format!("{windir}\\Fonts\\YuGothM.ttc"),
        format!("{windir}\\Fonts\\YuGothR.ttc"),
        format!("{windir}\\Fonts\\BIZ-UDGothicR.ttc"),
        format!("{windir}\\Fonts\\meiryo.ttc"),
        format!("{windir}\\Fonts\\msgothic.ttc"),
        format!("{windir}\\Fonts\\msyh.ttc"),
        "/System/Library/Fonts/Hiragino Sans GB.ttc".to_string(),
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc".to_string(),
    ];

    for path in &jp_candidates {
        if let Ok(font_data) = std::fs::read(path) {
            // 游ゴシック系はフォント仕様上ディセント余白が大きく上寄りになりやすいため、
            // 視覚的な上下中央（ボタン・タイトルバー・カレンダー数字）に揃うよう微調整する
            let is_yugothic = path.contains("YuGoth");
            let mut data = egui::FontData::from_owned(font_data);
            if is_yugothic {
                data = data.tweak(egui::FontTweak {
                    y_offset_factor: 0.21,
                    ..Default::default()
                });
            }
            fonts
                .font_data
                .insert("japanese_font".to_owned(), data.into());
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "japanese_font".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("japanese_font".to_owned());
            break;
        }
    }

    // デジタル時計用の太字フォント（Impact）を登録（存在しない場合は標準フォントへフォールバック）
    let impact_path = format!("{windir}\\Fonts\\impact.ttf");
    if let Ok(font_data) = std::fs::read(&impact_path) {
        fonts.font_data.insert(
            "impact_font".to_owned(),
            egui::FontData::from_owned(font_data).into(),
        );
        fonts.families.insert(
            egui::FontFamily::Name("impact".into()),
            vec!["impact_font".to_owned(), "japanese_font".to_owned()],
        );
    } else if fonts.font_data.contains_key("japanese_font") {
        fonts.families.insert(
            egui::FontFamily::Name("impact".into()),
            vec!["japanese_font".to_owned()],
        );
    }

    ctx.set_fonts(fonts);
}
