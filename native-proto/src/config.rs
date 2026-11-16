//! # 設定・状態永続化モジュール (`config.rs`)
//!
//! Phase C プロトタイプ用の表示設定・ウィンドウ位置の保存・復元と、
//! 既存の祝日設定ファイル（`%LOCALAPPDATA%/com.clondar.pro/holidays.json`）の読み込みを担当します。

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 内蔵標準の祝日設定 JSON（`src/config/holidays.json` と単一ソース共有）。
pub const DEFAULT_HOLIDAYS_JSON: &str = include_str!("../../src/config/holidays.json");

/// 時計の表示モード。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClockType {
    Digital,
    Analog,
}

/// プロトタイプのユーザー設定およびウィンドウ状態。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppStateConfig {
    pub is_24_hour: bool,
    pub show_seconds: bool,
    pub clock_type: ClockType,
    pub is_dark_mode: bool,
    pub is_transparent: bool,
    pub is_pinned: bool,
    pub window_pos: Option<[f32; 2]>,
}

impl Default for AppStateConfig {
    fn default() -> Self {
        Self {
            is_24_hour: true,
            show_seconds: true,
            clock_type: ClockType::Digital,
            is_dark_mode: true,
            is_transparent: false,
            is_pinned: true,
            window_pos: None,
        }
    }
}

/// ローカル設定ディレクトリ（`%LOCALAPPDATA%/com.clondar.pro`）を取得します。
fn get_app_data_dir() -> Option<PathBuf> {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let dir = PathBuf::from(local_app_data).join("com.clondar.pro");
        let _ = fs::create_dir_all(&dir);
        return Some(dir);
    }
    None
}

impl AppStateConfig {
    fn config_path() -> Option<PathBuf> {
        get_app_data_dir().map(|d| d.join("native_proto_state.json"))
    }

    /// 保存済みのプロトタイプ設定を読み込みます。存在しない場合はデフォルト値を返します。
    pub fn load_or_default() -> Self {
        if let Some(path) = Self::config_path() {
            if let Ok(text) = fs::read_to_string(path) {
                if let Ok(cfg) = serde_json::from_str::<Self>(&text) {
                    return cfg;
                }
            }
        }
        Self::default()
    }

    /// 現在の設定を JSON ファイルへ保存します。
    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = fs::write(path, json);
            }
        }
    }
}

/// 既存の外部 `holidays.json` があれば読み込み、無ければ内蔵デフォルト JSON を返します。
pub fn load_holidays_json_text() -> String {
    if let Some(dir) = get_app_data_dir() {
        let holidays_path = dir.join("holidays.json");
        if holidays_path.exists() {
            if let Ok(text) = fs::read_to_string(&holidays_path) {
                return text;
            }
        }
    }
    DEFAULT_HOLIDAYS_JSON.to_string()
}

/// 祝日設定の JSON 文字列をローカルデータ領域（`%LOCALAPPDATA%/com.clondar.pro/holidays.json`）へ保存します。
pub fn save_holidays_json_text(json_content: &str) -> Result<(), String> {
    let dir = get_app_data_dir()
        .ok_or_else(|| "LOCALAPPDATA 環境変数が取得できないため保存できません。".to_string())?;
    let path = dir.join("holidays.json");
    fs::write(&path, json_content).map_err(|e| format!("ファイル保存に失敗しました: {e}"))
}

/// 祝日設定を内蔵の初期デフォルト設定（[`DEFAULT_HOLIDAYS_JSON`]）で強制上書き復元します。
pub fn reset_holidays_json_text() -> Result<String, String> {
    save_holidays_json_text(DEFAULT_HOLIDAYS_JSON)?;
    Ok(DEFAULT_HOLIDAYS_JSON.to_string())
}
