//! # 日本の祝日算出エンジンおよび設定バリデーションモジュール (holidays.rs)
//!
//! 内蔵デフォルト定義および外部 JSON 設定に基づき、日本の祝日（固定祝日、ハッピーマンデー、
//! 歴代天皇誕生日、天文計算による春分・秋分、振替休日、国民の休日、特定年の特例上書き）を
//! 高速かつ型安全に計算します。

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// 祝日設定全体のデータ構造。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HolidayConfig {
    /// 固定祝日マップ（キー: "MM-DD", 値: 祝日名称）
    #[serde(default)]
    pub fixed: HashMap<String, String>,

    /// 2000年以降のハッピーマンデールール一覧
    #[serde(default)]
    pub happy_mondays: Vec<HappyMondayRule>,

    /// 1999年以前の旧祝日制度ルール一覧
    #[serde(default)]
    pub happy_mondays_legacy: Vec<LegacyHappyMondayRule>,

    /// 歴代天皇誕生日の期間ルール一覧
    #[serde(default)]
    pub emperor_birthdays: Vec<EmperorBirthdayRule>,

    /// 特定年限定の祝日上書きマップ（年 => 日付文字列 => 祝日名またはnull）
    #[serde(default)]
    pub custom_overrides: HashMap<String, HashMap<String, Option<String>>>,
}

/// 2000年以降の新ハッピーマンデールール。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HappyMondayRule {
    /// 対象月 (1〜12)
    pub month: u32,
    /// 第何週の月曜日か (1〜5)
    pub week: u32,
    /// 祝日名称
    pub name: String,
    /// 適用開始年 (西暦)
    pub start_year: u32,
    /// 適用終了年 (西暦、省略時は現在まで有効)
    pub end_year: Option<u32>,
}

/// 1999年以前の旧祝日制度ルール。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegacyHappyMondayRule {
    /// 対象月 (1〜12)
    pub month: u32,
    /// 固定日 (1〜31、固定日指定の場合)
    pub day: Option<u32>,
    /// 週番号 (第何月曜日か、週指定の場合)
    pub week: Option<u32>,
    /// 祝日名称
    pub name: String,
    /// 適用開始年 (西暦)
    pub start_year: u32,
    /// 適用終了年 (西暦)
    pub end_year: u32,
}

/// 歴代天皇誕生日の期間ルール。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmperorBirthdayRule {
    /// 適用開始年 (西暦)
    pub start_year: u32,
    /// 適用終了年 (西暦、省略時は現在まで有効)
    pub end_year: Option<u32>,
    /// 対象月 (1〜12)
    pub month: u32,
    /// 対象日 (1〜31)
    pub day: u32,
}

/// グレゴリオ暦におけるうるう年判定を行います。
#[inline]
pub fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

/// 指定した年・月の日数を取得します。
pub fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// 指定した年月日における曜日を算出します。
///
/// # 戻り値
/// 0: 日曜日, 1: 月曜日, 2: 火曜日, 3: 水曜日, 4: 木曜日, 5: 金曜日, 6: 土曜日
pub fn day_of_week(year: u32, month: u32, day: u32) -> u32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 3 { year - 1 } else { year };
    (y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + day) % 7
}

/// 指定した年・月の「第 N 月曜日」の日付（日）を算出します。
pub fn get_nth_monday(year: u32, month: u32, n: u32) -> u32 {
    let first_day = day_of_week(year, month, 1);
    let first_monday_day = 1 + if first_day == 1 {
        0
    } else {
        (8 - first_day) % 7
    };
    first_monday_day + (n - 1) * 7
}

/// 天文計算（海上保安庁基準式ベースの近似式）により春分の日を算出します。
pub fn calc_spring_equinox(year: u32) -> u32 {
    let y_diff = year as f64 - 1980.0;
    (20.8431 + 0.242194 * y_diff - (y_diff / 4.0).floor()).floor() as u32
}

/// 天文計算（海上保安庁基準式ベースの近似式）により秋分の日を算出します。
pub fn calc_autumn_equinox(year: u32) -> u32 {
    let y_diff = year as f64 - 1980.0;
    (23.2488 + 0.242194 * y_diff - (y_diff / 4.0).floor()).floor() as u32
}

/// 祝日設定 JSON 文字列をパースし、厳格なデータ型・範囲バリデーションを実施します。
///
/// # 引数
/// * `json_content` - 検証対象の JSON 文字列
///
/// # 戻り値
/// * `Ok(HolidayConfig)` - 検証に合格した設定構造体
/// * `Err(String)` - 構文エラーまたは範囲外データのエラーメッセージ
pub fn validate_and_parse_config(json_content: &str) -> Result<HolidayConfig, String> {
    let config: HolidayConfig = serde_json::from_str(json_content)
        .map_err(|e| format!("JSONの構文エラーまたは型が一致しません: {}", e))?;

    // 1. 固定祝日のキーフォーマット検証
    for (mmdd, name) in &config.fixed {
        if name.trim().is_empty() {
            return Err(format!("固定祝日 '{}' の名称が空です。", mmdd));
        }
        let parts: Vec<&str> = mmdd.split('-').collect();
        if parts.len() != 2 {
            return Err(format!(
                "固定祝日キー '{}' は 'MM-DD' 形式である必要があります。",
                mmdd
            ));
        }
        let month: u32 = parts[0]
            .parse()
            .map_err(|_| format!("月 '{}' が無効な数値です。", parts[0]))?;
        let day: u32 = parts[1]
            .parse()
            .map_err(|_| format!("日 '{}' が無効な数値です。", parts[1]))?;

        if !(1..=12).contains(&month) {
            return Err(format!(
                "月 '{}' は 1〜12 の範囲である必要があります。",
                month
            ));
        }
        if !(1..=31).contains(&day) {
            return Err(format!(
                "日 '{}' は 1〜31 の範囲である必要があります。",
                day
            ));
        }
    }

    // 2. ハッピーマンデールールの検証
    for rule in &config.happy_mondays {
        if !(1..=12).contains(&rule.month) {
            return Err(format!(
                "ハッピーマンデー '{}' の月 '{}' が無効です (1〜12)。",
                rule.name, rule.month
            ));
        }
        if !(1..=5).contains(&rule.week) {
            return Err(format!(
                "ハッピーマンデー '{}' の週番号 '{}' が無効です (1〜5)。",
                rule.name, rule.week
            ));
        }
        if let Some(end) = rule.end_year {
            if end < rule.start_year {
                return Err(format!(
                    "ハッピーマンデー '{}' の終了年 ({}) が開始年 ({}) より前です。",
                    rule.name, end, rule.start_year
                ));
            }
        }
    }

    // 3. 旧祝日制度ルールの検証
    for rule in &config.happy_mondays_legacy {
        if !(1..=12).contains(&rule.month) {
            return Err(format!(
                "旧祝日 '{}' の月 '{}' が無効です (1〜12)。",
                rule.name, rule.month
            ));
        }
        if let Some(day) = rule.day {
            if !(1..=31).contains(&day) {
                return Err(format!(
                    "旧祝日 '{}' の日 '{}' が無効です (1〜31)。",
                    rule.name, day
                ));
            }
        }
        if let Some(week) = rule.week {
            if !(1..=5).contains(&week) {
                return Err(format!(
                    "旧祝日 '{}' の週番号 '{}' が無効です (1〜5)。",
                    rule.name, week
                ));
            }
        }
        if rule.day.is_none() && rule.week.is_none() {
            return Err(format!(
                "旧祝日 '{}' には day または week の指定が必要です。",
                rule.name
            ));
        }
    }

    // 4. 天皇誕生日ルールの検証
    for rule in &config.emperor_birthdays {
        if !(1..=12).contains(&rule.month) {
            return Err(format!(
                "天皇誕生日の月 '{}' が無効です (1〜12)。",
                rule.month
            ));
        }
        if !(1..=31).contains(&rule.day) {
            return Err(format!(
                "天皇誕生日の日 '{}' が無効です (1〜31)。",
                rule.day
            ));
        }
    }

    Ok(config)
}

/// 年月日から "YYYY-MM-DD" 形式の日付文字列を生成します。
#[inline]
fn format_date(year: u32, month: u32, day: u32) -> String {
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// 指定した西暦年における日本の祝日一覧（振替休日、国民の休日、ハッピーマンデー、
/// 天皇誕生日、春分/秋分の日、特定年カスタム上書き）を計算して返します。
///
/// # 引数
/// * `year` - 対象の西暦年 (1948年以降)
/// * `config` - 適用する祝日設定
///
/// # 戻り値
/// 日付文字列（"YYYY-MM-DD"）をキー、祝日名称を値とするマップ
pub fn calculate_holidays(year: u32, config: &HolidayConfig) -> HashMap<String, String> {
    let mut holidays: BTreeMap<String, String> = BTreeMap::new();

    if year < 1948 {
        return HashMap::new();
    }

    // 1. 固定祝日
    for (mmdd, name) in &config.fixed {
        holidays.insert(format!("{:04}-{}", year, mmdd), name.clone());
    }

    // 2. ハッピーマンデー (2000年以降)
    for rule in &config.happy_mondays {
        let is_after_start = year >= rule.start_year;
        let is_before_end = rule.end_year.is_none_or(|end| year <= end);
        if is_after_start && is_before_end {
            let day = get_nth_monday(year, rule.month, rule.week);
            holidays.insert(format_date(year, rule.month, day), rule.name.clone());
        }
    }

    // 3. 旧ハッピーマンデー制度 (〜1999年)
    for rule in &config.happy_mondays_legacy {
        if year >= rule.start_year && year <= rule.end_year {
            if let Some(day) = rule.day {
                holidays.insert(format_date(year, rule.month, day), rule.name.clone());
            } else if let Some(week) = rule.week {
                let day = get_nth_monday(year, rule.month, week);
                holidays.insert(format_date(year, rule.month, day), rule.name.clone());
            }
        }
    }

    // 4. 天皇誕生日
    for rule in &config.emperor_birthdays {
        let is_after_start = year >= rule.start_year;
        let is_before_end = rule.end_year.is_none_or(|end| year <= end);
        if is_after_start && is_before_end {
            holidays.insert(
                format_date(year, rule.month, rule.day),
                "天皇誕生日".to_string(),
            );
        }
    }

    // 5. 天文計算による祝日
    let spring_day = calc_spring_equinox(year);
    holidays.insert(format_date(year, 3, spring_day), "春分の日".to_string());

    let autumn_day = calc_autumn_equinox(year);
    holidays.insert(format_date(year, 9, autumn_day), "秋分の日".to_string());

    // 6. 振替休日 (1973年以降)
    // 祝日が日曜日と重なった場合、直後の平日（連休の場合は祝日明け平日）を「振替休日」に設定
    if year >= 1973 {
        let existing_dates: Vec<String> = holidays.keys().cloned().collect();
        for date_str in existing_dates {
            let parts: Vec<&str> = date_str.split('-').collect();
            if parts.len() == 3 {
                let y: u32 = parts[0].parse().unwrap_or(year);
                let m: u32 = parts[1].parse().unwrap_or(1);
                let d: u32 = parts[2].parse().unwrap_or(1);

                if day_of_week(y, m, d) == 0 {
                    // 日曜日
                    let mut curr_y = y;
                    let mut curr_m = m;
                    let mut curr_d = d + 1;

                    if curr_d > days_in_month(curr_y, curr_m) {
                        curr_d = 1;
                        curr_m += 1;
                        if curr_m > 12 {
                            curr_m = 1;
                            curr_y += 1;
                        }
                    }

                    while holidays.contains_key(&format_date(curr_y, curr_m, curr_d)) {
                        curr_d += 1;
                        if curr_d > days_in_month(curr_y, curr_m) {
                            curr_d = 1;
                            curr_m += 1;
                            if curr_m > 12 {
                                curr_m = 1;
                                curr_y += 1;
                            }
                        }
                    }

                    holidays.insert(format_date(curr_y, curr_m, curr_d), "振替休日".to_string());
                }
            }
        }
    }

    // 7. 国民の休日 (祝日に挟まれた平日, 1986年以降)
    if year >= 1986 {
        let sorted_keys: Vec<String> = holidays.keys().cloned().collect();
        for i in 0..sorted_keys.len().saturating_sub(1) {
            let d1_str = &sorted_keys[i];
            let d2_str = &sorted_keys[i + 1];

            let p1: Vec<u32> = d1_str.split('-').filter_map(|s| s.parse().ok()).collect();
            let p2: Vec<u32> = d2_str.split('-').filter_map(|s| s.parse().ok()).collect();

            if p1.len() == 3 && p2.len() == 3 {
                // 同一年・同一月内で2日差（中1日）の場合
                if p1[0] == p2[0] && p1[1] == p2[1] && p2[2] == p1[2] + 2 {
                    let bridge_d = p1[2] + 1;
                    if day_of_week(p1[0], p1[1], bridge_d) != 0 {
                        // 日曜日以外
                        let bridge_key = format_date(p1[0], p1[1], bridge_d);
                        holidays
                            .entry(bridge_key)
                            .or_insert_with(|| "国民の休日".to_string());
                    }
                }
            }
        }
    }

    // 8. 特定年の上書き (カスタムオーバーライド)
    let year_str = year.to_string();
    if let Some(overrides) = config.custom_overrides.get(&year_str) {
        for (date_str, name_opt) in overrides {
            match name_opt {
                Some(name) => {
                    holidays.insert(date_str.clone(), name.clone());
                }
                None => {
                    holidays.remove(date_str);
                }
            }
        }
    }

    holidays.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DEFAULT_JSON: &str = r#"{
      "fixed": {
        "01-01": "元日",
        "02-11": "建国記念の日",
        "04-29": "昭和の日",
        "05-03": "憲法記念日",
        "05-04": "みどりの日",
        "05-05": "こどもの日",
        "08-11": "山の日",
        "11-03": "文化の日",
        "11-23": "勤労感謝の日"
      },
      "happy_mondays": [
        {
          "month": 1,
          "week": 2,
          "name": "成人の日",
          "start_year": 2000
        },
        {
          "month": 7,
          "week": 3,
          "name": "海の日",
          "start_year": 2003
        },
        {
          "month": 9,
          "week": 3,
          "name": "敬老の日",
          "start_year": 2003
        },
        {
          "month": 10,
          "week": 2,
          "name": "スポーツの日",
          "start_year": 2020
        }
      ],
      "happy_mondays_legacy": [
        {
          "month": 1,
          "day": 15,
          "name": "成人の日",
          "start_year": 1949,
          "end_year": 1999
        },
        {
          "month": 7,
          "day": 20,
          "name": "海の日",
          "start_year": 1996,
          "end_year": 2002
        },
        {
          "month": 9,
          "day": 15,
          "name": "敬老の日",
          "start_year": 1966,
          "end_year": 2002
        },
        {
          "month": 10,
          "day": 10,
          "name": "体育の日",
          "start_year": 1966,
          "end_year": 1999
        },
        {
          "month": 10,
          "week": 2,
          "name": "体育の日",
          "start_year": 2000,
          "end_year": 2019
        }
      ],
      "emperor_birthdays": [
        {
          "start_year": 1949,
          "end_year": 1988,
          "month": 4,
          "day": 29
        },
        {
          "start_year": 1989,
          "end_year": 2018,
          "month": 12,
          "day": 23
        },
        {
          "start_year": 2020,
          "month": 2,
          "day": 23
        }
      ],
      "custom_overrides": {
        "2020": {
          "2020-07-23": "海の日",
          "2020-07-24": "スポーツの日",
          "2020-08-10": "山の日",
          "2020-07-20": null,
          "2020-08-11": null,
          "2020-10-12": null
        },
        "2021": {
          "2021-07-22": "海の日",
          "2021-07-23": "スポーツの日",
          "2021-08-08": "山の日",
          "2021-07-19": null,
          "2021-08-11": null,
          "2021-10-11": null
        }
      }
    }"#;

    #[test]
    fn test_calculate_holidays_2026_fixed() {
        let config = validate_and_parse_config(TEST_DEFAULT_JSON).unwrap();
        let holidays = calculate_holidays(2026, &config);

        assert_eq!(holidays.get("2026-01-01").unwrap(), "元日");
        assert_eq!(holidays.get("2026-02-11").unwrap(), "建国記念の日");
        assert_eq!(holidays.get("2026-02-23").unwrap(), "天皇誕生日");
        assert_eq!(holidays.get("2026-04-29").unwrap(), "昭和の日");
        assert_eq!(holidays.get("2026-05-03").unwrap(), "憲法記念日");
        assert_eq!(holidays.get("2026-05-04").unwrap(), "みどりの日");
        assert_eq!(holidays.get("2026-05-05").unwrap(), "こどもの日");
        assert_eq!(holidays.get("2026-08-11").unwrap(), "山の日");
        assert_eq!(holidays.get("2026-11-03").unwrap(), "文化の日");
        assert_eq!(holidays.get("2026-11-23").unwrap(), "勤労感謝の日");
    }

    #[test]
    fn test_calculate_holidays_emperor_eras() {
        let config = validate_and_parse_config(TEST_DEFAULT_JSON).unwrap();

        // 昭和時代 (1988年: 4月29日)
        let holidays1988 = calculate_holidays(1988, &config);
        assert_eq!(holidays1988.get("1988-04-29").unwrap(), "天皇誕生日");

        // 平成時代 (1989年: 12月23日)
        let holidays1989 = calculate_holidays(1989, &config);
        assert_eq!(holidays1989.get("1989-12-23").unwrap(), "天皇誕生日");

        // 平成最終年 (2018年: 12月23日)
        let holidays2018 = calculate_holidays(2018, &config);
        assert_eq!(holidays2018.get("2018-12-23").unwrap(), "天皇誕生日");

        // 即位特例年 (2019年: 天皇誕生日なし)
        let holidays2019 = calculate_holidays(2019, &config);
        assert!(!holidays2019.contains_key("2019-12-23"));
        assert!(!holidays2019.contains_key("2019-02-23"));

        // 令和時代 (2020年: 2月23日)
        let holidays2020 = calculate_holidays(2020, &config);
        assert_eq!(holidays2020.get("2020-02-23").unwrap(), "天皇誕生日");
        assert!(!holidays2020.contains_key("2020-12-23"));
    }

    #[test]
    fn test_calculate_holidays_happy_monday_transitions() {
        let config = validate_and_parse_config(TEST_DEFAULT_JSON).unwrap();

        // 1999年 成人の日 (旧制度 1月15日)
        let holidays1999 = calculate_holidays(1999, &config);
        assert_eq!(holidays1999.get("1999-01-15").unwrap(), "成人の日");
        assert!(!holidays1999.contains_key("1999-01-11"));

        // 2000年 成人の日 (新制度 1月第2月曜 = 1月10日)
        let holidays2000 = calculate_holidays(2000, &config);
        assert_eq!(holidays2000.get("2000-01-10").unwrap(), "成人の日");
        assert!(!holidays2000.contains_key("2000-01-15"));

        // 2002年 海の日 (旧制度 7月20日)
        let holidays2002 = calculate_holidays(2002, &config);
        assert_eq!(holidays2002.get("2002-07-20").unwrap(), "海の日");

        // 2003年 海の日 (新制度 7月第3月曜 = 7月21日)
        let holidays2003 = calculate_holidays(2003, &config);
        assert_eq!(holidays2003.get("2003-07-21").unwrap(), "海の日");
        assert!(!holidays2003.contains_key("2003-07-20"));
    }

    #[test]
    fn test_calculate_holidays_equinox() {
        let config = validate_and_parse_config(TEST_DEFAULT_JSON).unwrap();
        let holidays2026 = calculate_holidays(2026, &config);

        assert_eq!(holidays2026.get("2026-03-20").unwrap(), "春分の日");
        assert_eq!(holidays2026.get("2026-09-23").unwrap(), "秋分の日");
    }

    #[test]
    fn test_calculate_holidays_substitute_and_national() {
        let config = validate_and_parse_config(TEST_DEFAULT_JSON).unwrap();
        let holidays2026 = calculate_holidays(2026, &config);

        // 2026年5月3日(日)憲法記念日 -> 5/4みどりの日、5/5こどもの日のため 5/6(水)が振替休日
        assert_eq!(holidays2026.get("2026-05-06").unwrap(), "振替休日");

        // 2026年9月21日(月)敬老の日、9月23日(水)秋分の日 -> 挟まれた9/22(火)が国民の休日
        assert_eq!(holidays2026.get("2026-09-22").unwrap(), "国民の休日");
    }

    #[test]
    fn test_calculate_holidays_custom_overrides_2020_and_2021() {
        let config = validate_and_parse_config(TEST_DEFAULT_JSON).unwrap();

        // 2020年 オリンピック特例
        let holidays2020 = calculate_holidays(2020, &config);
        assert_eq!(holidays2020.get("2020-07-23").unwrap(), "海の日");
        assert_eq!(holidays2020.get("2020-07-24").unwrap(), "スポーツの日");
        assert_eq!(holidays2020.get("2020-08-10").unwrap(), "山の日");

        assert!(!holidays2020.contains_key("2020-07-20"));
        assert!(!holidays2020.contains_key("2020-08-11"));
        assert!(!holidays2020.contains_key("2020-10-12"));

        // 2021年 オリンピック特例
        let holidays2021 = calculate_holidays(2021, &config);
        assert_eq!(holidays2021.get("2021-07-22").unwrap(), "海の日");
        assert_eq!(holidays2021.get("2021-07-23").unwrap(), "スポーツの日");
        assert_eq!(holidays2021.get("2021-08-08").unwrap(), "山の日");

        assert!(!holidays2021.contains_key("2021-07-19"));
        assert!(!holidays2021.contains_key("2021-08-11"));
        assert!(!holidays2021.contains_key("2021-10-11"));
    }

    #[test]
    fn test_pre_1948_boundary() {
        let config = validate_and_parse_config(TEST_DEFAULT_JSON).unwrap();
        assert!(calculate_holidays(1947, &config).is_empty());
        assert!(calculate_holidays(1900, &config).is_empty());
    }

    #[test]
    fn test_date_algorithm_helpers() {
        // うるう年判定
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2100));
        assert!(!is_leap_year(2023));

        // 月の日数
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2023, 2), 28);
        assert_eq!(days_in_month(2026, 4), 30);
        assert_eq!(days_in_month(2026, 5), 31);

        // サカモトのアルゴリズム（曜日判定: 日=0, 月=1, ..., 土=6）
        assert_eq!(day_of_week(2026, 1, 1), 4); // 2026-01-01 は木曜日(4)
        assert_eq!(day_of_week(2026, 5, 3), 0); // 2026-05-03 は日曜日(0)

        // 第N月曜日判定
        assert_eq!(get_nth_monday(2026, 1, 2), 12); // 2026年1月第2月曜日は1月12日
        assert_eq!(get_nth_monday(2026, 7, 3), 20); // 2026年7月第3月曜日は7月20日

        // 春分・秋分の日の天文計算
        assert_eq!(calc_spring_equinox(2026), 20);
        assert_eq!(calc_autumn_equinox(2026), 23);
    }

    #[test]
    fn test_validation_error_handling() {
        // 不正な月のテスト
        let invalid_month_json = r#"{
            "fixed": { "13-01": "不正な月" }
        }"#;
        assert!(validate_and_parse_config(invalid_month_json).is_err());

        // 不正な日フォーマットのテスト
        let invalid_format_json = r#"{
            "fixed": { "01/01": "スラッシュ区切り" }
        }"#;
        assert!(validate_and_parse_config(invalid_format_json).is_err());

        // 不正な週番号のテスト
        let invalid_week_json = r#"{
            "happy_mondays": [{
                "month": 1,
                "week": 6,
                "name": "第6月曜",
                "start_year": 2000
            }]
        }"#;
        assert!(validate_and_parse_config(invalid_week_json).is_err());

        // 空文字列
        assert!(validate_and_parse_config("").is_err());
    }
}
