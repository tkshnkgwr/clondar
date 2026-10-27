/**
 * @fileoverview 日本の祝日定義のロード、保存、および動的計算エンジン。
 *
 * 内蔵デフォルト定義および外部 JSON（%LOCALAPPDATA%/com.clondar.pro/holidays.json）に基づき、
 * 固定祝日、ハッピーマンデー制度（新旧）、歴代天皇誕生日、天文計算による春分・秋分、
 * 振替休日、国民の休日、および特定年の特例上書き（オリンピック特例等）を正確に計算します。
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * 祝日設定データの型定義。
 * @typedef {Object} HolidayConfig
 * @property {Object.<string, string>} fixed - 固定祝日マップ（キー: "MM-DD"形式, 値: 祝日名）
 * @property {HappyMondayRule[]} happy_mondays - 2000年以降の新ハッピーマンデールール配列
 * @property {LegacyHappyMondayRule[]} happy_mondays_legacy - 1999年以前の旧祝日ルール配列
 * @property {EmperorBirthdayRule[]} emperor_birthdays - 天皇誕生日の期間ルール配列
 * @property {Object.<string, Object.<string, string|null>>} [custom_overrides] - 特定年限定の祝日上書きマップ（年 => 日付 => 祝日名またはnull）
 */

/**
 * ハッピーマンデールールの型定義。
 * @typedef {Object} HappyMondayRule
 * @property {number} month - 月 (1〜12)
 * @property {number} week - 第何週の月曜日か (例: 2 = 第2月曜日)
 * @property {string} name - 祝日名称 (例: "成人の日")
 * @property {number} start_year - 適用開始年 (西暦)
 * @property {number} [end_year] - 適用終了年 (西暦、省略時は現在まで有効)
 */

/**
 * 旧祝日制度ルールの型定義。
 * @typedef {Object} LegacyHappyMondayRule
 * @property {number} month - 月 (1〜12)
 * @property {number} [day] - 固定日 (例: 15)
 * @property {number} [week] - 週番号 (第何月曜日か)
 * @property {string} name - 祝日名称
 * @property {number} start_year - 適用開始年 (西暦)
 * @property {number} end_year - 適用終了年 (西暦)
 */

/**
 * 天皇誕生日ルールの型定義。
 * @typedef {Object} EmperorBirthdayRule
 * @property {number} start_year - 在位開始年（または祝日適用開始年）
 * @property {number} [end_year] - 在位終了年
 * @property {number} month - 月 (1〜12)
 * @property {number} day - 日 (1〜31)
 */

/** @type {HolidayConfig|null} キャッシュされた祝日設定オブジェクト */
let config = null;

/**
 * 祝日設定のデフォルト（内蔵フォールバック）定義。
 * 外部ファイルの読み込みに失敗した場合や初期起動時に使用されます。
 * @type {HolidayConfig}
 */
export const fallbackConfig = {
  fixed: {
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
  happy_mondays: [
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
  happy_mondays_legacy: [
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
  emperor_birthdays: [
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
  custom_overrides: {
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
};

/**
 * 祝日設定データを初期化してメモリ上にロードします。
 *
 * ## 動作仕様
 * 1. 既にキャッシュ（`config` 変数）が存在する場合は、それを即座に返します。
 * 2. Tauri 環境下であれば、Rust の IPC コマンド `load_holidays_json` を呼び出してローカルファイルから読み込みます。
 * 3. ブラウザ環境であれば、静的パス `/config/holidays.json` へ `fetch` リクエストを試行します。
 * 4. いずれの読み込みにも失敗した場合は、内蔵の `fallbackConfig` を適用して安全にフォールバックします。
 *
 * @async
 * @returns {Promise<HolidayConfig>} ロードされた祝日設定オブジェクト
 * @example
 * const config = await initHolidays();
 * console.log("固定祝日数:", Object.keys(config.fixed).length);
 */
export async function initHolidays() {
  if (config) return config;
  
  const isTauri = typeof window !== 'undefined' && !!window.__TAURI__;
  if (isTauri) {
    try {
      const jsonText = await invoke('load_holidays_json');
      config = JSON.parse(jsonText);
      return config;
    } catch (e) {
      console.error('Error invoking load_holidays_json, fallback to fetch:', e);
    }
  }

  try {
    if (typeof fetch !== 'undefined' && typeof window !== 'undefined' && window.location && window.location.origin) {
      const res = await fetch('/config/holidays.json');
      if (!res.ok) throw new Error('Failed to fetch holidays config');
      config = await res.json();
    } else {
      config = fallbackConfig;
    }
  } catch (e) {
    console.error('Error loading holidays config, using built-in fallback:', e);
    config = fallbackConfig;
  }
  return config;
}

/**
 * 祝日設定データのインメモリキャッシュを明示的にクリアし、最新設定を再ロードします。
 * 祝日設定マネージャーで新しい祝日を保存した直後などに呼び出されます。
 *
 * @async
 * @returns {Promise<HolidayConfig>} 再ロードされた最新の祝日設定オブジェクト
 */
export function reloadHolidays() {
  config = null;
  return initHolidays();
}

/**
 * 祝日設定を内蔵の初期デフォルト設定で強制上書き復元（リセット）します。
 *
 * @async
 * @returns {Promise<HolidayConfig>} 復元されたデフォルト祝日設定オブジェクト
 */
export async function resetHolidays() {
  const isTauri = typeof window !== 'undefined' && !!window.__TAURI__;
  if (isTauri) {
    try {
      const jsonText = await invoke('reset_holidays_json');
      config = JSON.parse(jsonText);
      return config;
    } catch (e) {
      console.error('Error invoking reset_holidays_json:', e);
    }
  }
  config = fallbackConfig;
  return config;
}

/**
 * 指定された西暦年の日本の祝日マップ（日付文字列 => 祝日名称）を計算して返します。
 *
 * ## 計算アルゴリズムの構成
 * 1. **固定祝日**: `fixed` マップから該当年の "YYYY-MM-DD" を生成。
 * 2. **ハッピーマンデー制度**: 該当年月に応じた「第 N 月曜日」の日付を動的算出。
 * 3. **歴代天皇誕生日**: 在位期間に合致する天皇誕生日を判定・追加。
 * 4. **天文計算（春分の日・秋分の日）**: 簡易天文計算式（海上保安庁基準式ベース）により日付を導出。
 * 5. **振替休日 (1973年〜)**: 祝日が日曜日と重なった場合、直後の平日（連休の場合は祝日明け平日）を「振替休日」に設定。
 * 6. **国民の休日 (1986年〜)**: 2つの祝日に挟まれた平日（日曜日でない日）を「国民の休日」に設定。
 * 7. **特定年上書き**: オリンピック特例等、特定年に特別に移動・削除（null指定）された祝日を反映。
 *
 * @param {number|string} yearParam - 祝日を算出する対象の西暦年 (例: 2026, "2026")
 * @returns {Object.<string, string>} "YYYY-MM-DD" 形式の日付文字列をキー、祝日名称を値とするマップ
 * @example
 * const holidays2026 = getHolidays(2026);
 * console.log(holidays2026["2026-01-01"]); // "元日"
 * console.log(holidays2026["2026-05-06"]); // "振替休日"
 */
export function getHolidays(yearParam) {
  const year = parseInt(yearParam, 10);
  const holidays = {};
  
  if (isNaN(year) || year < 1948) return {};
  
  const activeConfig = config || fallbackConfig;

  const formatDate = (d) => `${d.getFullYear()}-${(d.getMonth() + 1).toString().padStart(2, '0')}-${d.getDate().toString().padStart(2, '0')}`;
  
  const getNthMonday = (y, m, n) => {
    const firstDay = new Date(y, m - 1, 1);
    let day = firstDay.getDay();
    let date = 1 + (day === 1 ? 0 : (8 - day) % 7) + (n - 1) * 7;
    return new Date(y, m - 1, date);
  };

  // 1. 固定祝日
  if (activeConfig.fixed) {
    Object.entries(activeConfig.fixed).forEach(([mmdd, name]) => {
      holidays[`${year}-${mmdd}`] = name;
    });
  }

  // 2. ハッピーマンデー (2000年以降)
  if (activeConfig.happy_mondays) {
    activeConfig.happy_mondays.forEach(rule => {
      if (year >= rule.start_year && (!rule.end_year || year <= rule.end_year)) {
        holidays[formatDate(getNthMonday(year, rule.month, rule.week))] = rule.name;
      }
    });
  }

  // 3. ハッピーマンデー旧制度 (〜1999年) および特定の固定日
  if (activeConfig.happy_mondays_legacy) {
    activeConfig.happy_mondays_legacy.forEach(rule => {
      if (year >= rule.start_year && year <= rule.end_year) {
        if (rule.day) {
          holidays[`${year}-${rule.month.toString().padStart(2, '0')}-${rule.day.toString().padStart(2, '0')}`] = rule.name;
        } else if (rule.week) {
          holidays[formatDate(getNthMonday(year, rule.month, rule.week))] = rule.name;
        }
      }
    });
  }

  // 4. 天皇誕生日
  if (activeConfig.emperor_birthdays) {
    activeConfig.emperor_birthdays.forEach(rule => {
      if (year >= rule.start_year && (!rule.end_year || year <= rule.end_year)) {
        holidays[`${year}-${rule.month.toString().padStart(2, '0')}-${rule.day.toString().padStart(2, '0')}`] = "天皇誕生日";
      }
    });
  }

  // 5. 天文計算による祝日
  const springEquinox = Math.floor(20.8431 + 0.242194 * (year - 1980) - Math.floor((year - 1980) / 4));
  holidays[`${year}-03-${springEquinox.toString().padStart(2, '0')}`] = "春分の日";
  const autumnEquinox = Math.floor(23.2488 + 0.242194 * (year - 1980) - Math.floor((year - 1980) / 4));
  holidays[`${year}-09-${autumnEquinox.toString().padStart(2, '0')}`] = "秋分の日";

  // 6. 振替休日 (1973年以降)
  if (year >= 1973) {
    const baseHolidays = Object.keys(holidays).sort();
    baseHolidays.forEach(dateStr => {
      const date = new Date(dateStr);
      if (date.getDay() === 0) {
        let substitute = new Date(date);
        substitute.setDate(substitute.getDate() + 1);
        while (holidays[formatDate(substitute)]) {
          substitute.setDate(substitute.getDate() + 1);
        }
        holidays[formatDate(substitute)] = "振替休日";
      }
    });
  }
  
  // 7. 国民の休日 (祝日に挟まれた平日, 1986年以降)
  if (year >= 1986) {
    const sortedKeys = Object.keys(holidays).sort();
    for (let i = 0; i < sortedKeys.length - 1; i++) {
      let d1 = new Date(sortedKeys[i]);
      let d2 = new Date(sortedKeys[i+1]);
      let diff = (d2 - d1) / (1000 * 60 * 60 * 24);
      if (diff === 2) {
        let bridgeDay = new Date(d1);
        bridgeDay.setDate(bridgeDay.getDate() + 1);
        if (bridgeDay.getDay() !== 0) {
          holidays[formatDate(bridgeDay)] = "国民の休日";
        }
      }
    }
  }

  // 8. 特定年の上書き (カスタムオーバーライド: オリンピック等)
  if (activeConfig.custom_overrides && activeConfig.custom_overrides[year]) {
    Object.entries(activeConfig.custom_overrides[year]).forEach(([dateStr, name]) => {
      if (name === null) {
        delete holidays[dateStr];
      } else {
        holidays[dateStr] = name;
      }
    });
  }

  return holidays;
}
