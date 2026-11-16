/**
 * @fileoverview 日本の祝日定義のロード、保存、および動的計算エンジン。
 *
 * 内蔵デフォルト定義（`src/config/holidays.json`）および外部 JSON（%LOCALAPPDATA%/com.clondar.pro/holidays.json）に基づき、
 * Tauri 環境では Rust バックエンド（`get_holidays` IPC コマンド）へ計算を一本化し、
 * 非 Tauri 環境（ブラウザプレビュー・Vitest テスト）では内蔵フォールバック計算を実行します。
 */

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import defaultHolidaysConfig from '../config/holidays.json';

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

/** @type {HolidayConfig|null} 非Tauri環境用にキャッシュされた祝日設定オブジェクト */
let config = null;

/** @type {Map<number, Object.<string, string>>} 年ごとの祝日マップのインメモリキャッシュ */
const holidaysCache = new Map();

/** @type {Map<number, Promise<Object.<string, string>>>} 進行中の年別取得 Promise マップ */
const pendingFetches = new Map();

/**
 * 祝日設定のデフォルト（内蔵フォールバック）定義。
 * `src/config/holidays.json` を単一ソースとしてバンドル・共有します。
 * @type {HolidayConfig}
 */
export const fallbackConfig = defaultHolidaysConfig;

/**
 * 実行環境が Tauri ネイティブ環境かを判定します。
 * @returns {boolean}
 */
const isTauriEnv = () => typeof window !== 'undefined' && !!window.__TAURI__;

/**
 * キャッシュ済みの指定年の祝日マップを同期取得します。
 * キャッシュ未登録の場合は `null` を返します（1948年未満または不正入力は空オブジェクト `{}`）。
 *
 * @param {number|string} yearParam - 対象の西暦年
 * @returns {Object.<string, string>|null} キャッシュ済み祝日マップ、未登録時は null
 */
export function getCachedHolidays(yearParam) {
  const year = parseInt(yearParam, 10);
  if (isNaN(year) || year < 1948) return {};
  return holidaysCache.get(year) ?? null;
}

/**
 * 指定された西暦年の日本の祝日マップを非同期に取得します。
 * Tauri 環境下では Rust バックエンドの IPC コマンド `get_holidays` に計算を一本化し、
 * ブラウザ環境または取得失敗時のみ JavaScript 側計算ロジックを実行してキャッシュします。
 *
 * @async
 * @param {number|string} yearParam - 祝日を算出する対象の西暦年 (例: 2026)
 * @returns {Promise<Object.<string, string>>} "YYYY-MM-DD" 形式の日付文字列をキー、祝日名称を値とするマップ
 */
export async function fetchHolidays(yearParam) {
  const year = parseInt(yearParam, 10);
  if (isNaN(year) || year < 1948) return {};

  if (holidaysCache.has(year)) {
    return holidaysCache.get(year);
  }

  if (pendingFetches.has(year)) {
    return pendingFetches.get(year);
  }

  const fetchPromise = (async () => {
    if (isTauriEnv()) {
      try {
        const holidays = await invoke('get_holidays', { year });
        if (holidays && typeof holidays === 'object') {
          holidaysCache.set(year, holidays);
          return holidays;
        }
      } catch (e) {
        console.error(`Rust get_holidays(${year}) failed, fallback to JS:`, e);
      }
    }

    const holidays = computeHolidaysJs(year);
    holidaysCache.set(year, holidays);
    return holidays;
  })();

  pendingFetches.set(year, fetchPromise);
  try {
    return await fetchPromise;
  } finally {
    pendingFetches.delete(year);
  }
}

/**
 * 指定年の祝日マップを取得する React カスタムフック。
 * キャッシュヒット時は即座に返し、未キャッシュ時のみ `fetchHolidays` を非同期実行します。
 *
 * @param {number} year - 対象の西暦年
 * @returns {Object.<string, string>} 日付文字列をキー、祝日名称を値とするマップ
 */
export function useHolidays(year) {
  const [holidays, setHolidays] = useState(() => getCachedHolidays(year) ?? {});

  useEffect(() => {
    const cached = getCachedHolidays(year);
    if (cached !== null) {
      setHolidays(cached);
      return;
    }

    let cancelled = false;
    fetchHolidays(year).then((result) => {
      if (!cancelled) {
        setHolidays(result);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [year]);

  return holidays;
}

/**
 * 祝日設定データを初期化してメモリ上にロードします。
 *
 * ## 動作仕様
 * 1. Tauri 環境下では Rust 側の `get_holidays` により当年分の祝日マップを直接プリフェッチします（二重JSONパース回避）。
 * 2. 非 Tauri 環境下では `localStorage` のモック設定またはバンドル済み `fallbackConfig` をロードします。
 *
 * @async
 * @returns {Promise<HolidayConfig>} ロードされた祝日設定オブジェクト
 */
export async function initHolidays() {
  if (isTauriEnv()) {
    const currentYear = new Date().getFullYear();
    await fetchHolidays(currentYear);
    return config || fallbackConfig;
  }

  if (config) return config;

  try {
    if (typeof window !== 'undefined' && typeof localStorage !== 'undefined') {
      const mockJson = localStorage.getItem('mock_holidays_json');
      if (mockJson) {
        config = JSON.parse(mockJson);
        return config;
      }
    }
  } catch (e) {
    console.error('Error loading mock holidays config, using built-in fallback:', e);
  }

  config = fallbackConfig;
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
  holidaysCache.clear();
  pendingFetches.clear();
  return initHolidays();
}

/**
 * 祝日設定を内蔵の初期デフォルト設定で強制上書き復元（リセット）します。
 *
 * @async
 * @returns {Promise<HolidayConfig>} 復元されたデフォルト祝日設定オブジェクト
 */
export async function resetHolidays() {
  holidaysCache.clear();
  pendingFetches.clear();
  if (isTauriEnv()) {
    try {
      const jsonText = await invoke('reset_holidays_json');
      config = JSON.parse(jsonText);
      const currentYear = new Date().getFullYear();
      await fetchHolidays(currentYear);
      return config;
    } catch (e) {
      console.error('Error invoking reset_holidays_json:', e);
    }
  } else if (typeof window !== 'undefined' && typeof localStorage !== 'undefined') {
    localStorage.removeItem('mock_holidays_json');
  }
  config = fallbackConfig;
  return config;
}

/**
 * Date オブジェクトを "YYYY-MM-DD" 形式にフォーマットします。
 * @param {Date} d
 * @returns {string}
 */
export function formatDateStr(d) {
  return `${d.getFullYear()}-${(d.getMonth() + 1).toString().padStart(2, '0')}-${d.getDate().toString().padStart(2, '0')}`;
}

/**
 * JavaScript による祝日日付計算フォールバックロジック（非 Tauri 環境・単体テスト用）。
 *
 * @param {number} year - 対象の西暦年
 * @returns {Object.<string, string>} 祝日マップ
 */
function computeHolidaysJs(year) {
  const holidays = {};
  if (isNaN(year) || year < 1948) return {};

  const activeConfig = config || fallbackConfig;

  const getNthMonday = (y, m, n) => {
    const firstDay = new Date(y, m - 1, 1);
    const day = firstDay.getDay();
    const date = 1 + (day === 1 ? 0 : (8 - day) % 7) + (n - 1) * 7;
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
        holidays[formatDateStr(getNthMonday(year, rule.month, rule.week))] = rule.name;
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
          holidays[formatDateStr(getNthMonday(year, rule.month, rule.week))] = rule.name;
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
        const substitute = new Date(date);
        substitute.setDate(substitute.getDate() + 1);
        while (holidays[formatDateStr(substitute)]) {
          substitute.setDate(substitute.getDate() + 1);
        }
        holidays[formatDateStr(substitute)] = "振替休日";
      }
    });
  }

  // 7. 国民の休日 (祝日に挟まれた平日, 1986年以降)
  if (year >= 1986) {
    const sortedKeys = Object.keys(holidays).sort();
    for (let i = 0; i < sortedKeys.length - 1; i++) {
      const d1 = new Date(sortedKeys[i]);
      const d2 = new Date(sortedKeys[i + 1]);
      const diff = (d2 - d1) / (1000 * 60 * 60 * 24);
      if (diff === 2) {
        const bridgeDay = new Date(d1);
        bridgeDay.setDate(bridgeDay.getDate() + 1);
        if (bridgeDay.getDay() !== 0) {
          holidays[formatDateStr(bridgeDay)] = "国民の休日";
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

/**
 * 指定された西暦年の日本の祝日マップ（日付文字列 => 祝日名称）を同期的に計算またはキャッシュから返します。
 * 主に非 Tauri 環境および単体テスト向けに使用されます。
 *
 * @param {number|string} yearParam - 祝日を算出する対象の西暦年 (例: 2026, "2026")
 * @returns {Object.<string, string>} "YYYY-MM-DD" 形式の日付文字列をキー、祝日名称を値とするマップ
 */
export function getHolidays(yearParam) {
  const year = parseInt(yearParam, 10);
  if (isNaN(year) || year < 1948) return {};

  if (holidaysCache.has(year)) {
    return holidaysCache.get(year);
  }

  const holidays = computeHolidaysJs(year);
  holidaysCache.set(year, holidays);
  return holidays;
}
