/**
 * @fileoverview 日本の祝日算出エンジンのユニットテストスイート (Vitest)。
 *
 * 以下の境界条件および制度変更ケースを包括的に検証します：
 * - 固定祝日の一覧取得
 * - 歴代天皇誕生日（昭和・平成・令和）の在位期間切り替え
 * - 新旧ハッピーマンデー制度の切り替え境界（1999年/2000年成人の日、2002年/2003年海の日）
 * - 春分・秋分の日の天文計算式
 * - 振替休日（5月連休など祝日が連続する場合の平日繰り下げ判定）
 * - 国民の休日（祝日に挟まれた平日の判定）
 * - 特定年のカスタムオーバーライド（2020年/2021年東京オリンピック特例措置等）
 * - 境界値・エッジケース（1948年未満、不正入力）
 * - ライフサイクル関数（initHolidays, reloadHolidays, resetHolidays）
 */

import { describe, test, expect, beforeEach } from 'vitest';
import { getHolidays, initHolidays, reloadHolidays, resetHolidays, fallbackConfig } from './holidays';

describe('日本の祝日計算ロジック (getHolidays)', () => {

  /**
   * 固定祝日の検証。
   * 指定年の固定祝日（元日、建国記念の日、こどもの日、山の日等）が正しく計算されるかをテストします。
   */
  test('固定祝日が正しく取得できること', () => {
    // 2026年の固定祝日
    const holidays = getHolidays(2026);
    
    expect(holidays['2026-01-01']).toBe('元日');
    expect(holidays['2026-02-11']).toBe('建国記念の日');
    expect(holidays['2026-02-23']).toBe('天皇誕生日');
    expect(holidays['2026-04-29']).toBe('昭和の日');
    expect(holidays['2026-05-03']).toBe('憲法記念日');
    expect(holidays['2026-05-04']).toBe('みどりの日');
    expect(holidays['2026-05-05']).toBe('こどもの日');
    expect(holidays['2026-08-11']).toBe('山の日');
    expect(holidays['2026-11-03']).toBe('文化の日');
    expect(holidays['2026-11-23']).toBe('勤労感謝の日');
  });

  test('歴代天皇誕生日が在位期間に応じて正しく切り替わること', () => {
    // 昭和時代 (1988年以前: 4月29日)
    const holidays1988 = getHolidays(1988);
    expect(holidays1988['1988-04-29']).toBe('天皇誕生日');

    // 平成時代 (1989年〜2018年: 12月23日)
    const holidays1989 = getHolidays(1989);
    expect(holidays1989['1989-12-23']).toBe('天皇誕生日');

    const holidays2018 = getHolidays(2018);
    expect(holidays2018['2018-12-23']).toBe('天皇誕生日');

    // 2019年 (退位・即位特例年: 天皇誕生日なし)
    const holidays2019 = getHolidays(2019);
    expect(holidays2019['2019-12-23']).toBeUndefined();
    expect(holidays2019['2019-02-23']).toBeUndefined();

    // 令和時代 (2020年以降: 2月23日)
    const holidays2020 = getHolidays(2020);
    expect(holidays2020['2020-02-23']).toBe('天皇誕生日');
    expect(holidays2020['2020-12-23']).toBeUndefined();

    const holidays2026 = getHolidays(2026);
    expect(holidays2026['2026-02-23']).toBe('天皇誕生日');
  });

  test('ハッピーマンデー制度（新旧）が正しく切り替わること', () => {
    // --- 成人の日 (旧: 1月15日, 新: 1月第2月曜日) ---
    // 1999年（旧制度）：1月15日
    const holidays1999 = getHolidays(1999);
    expect(holidays1999['1999-01-15']).toBe('成人の日');
    expect(holidays1999['1999-01-11']).toBeUndefined();

    // 2000年（新制度）：1月第2月曜日 (1月10日)
    const holidays2000 = getHolidays(2000);
    expect(holidays2000['2000-01-10']).toBe('成人の日');
    expect(holidays2000['2000-01-15']).toBeUndefined();

    // --- 海の日 (旧: 7月20日, 新: 7月第3月曜日) ---
    // 2002年（旧制度）：7月20日
    const holidays2002 = getHolidays(2002);
    expect(holidays2002['2002-07-20']).toBe('海の日');

    // 2003年（新制度）：7月第3月曜日 (7月21日)
    const holidays2003 = getHolidays(2003);
    expect(holidays2003['2003-07-21']).toBe('海の日');
    expect(holidays2003['2003-07-20']).toBeUndefined();
  });

  test('天文計算（春分の日・秋分の日）が正しく計算されること', () => {
    // 2026年：春分の日（3月20日）、秋分の日（9月23日）
    const holidays2026 = getHolidays(2026);
    expect(holidays2026['2026-03-20']).toBe('春分の日');
    expect(holidays2026['2026-09-23']).toBe('秋分の日');

    // 2020年：春分の日（3月20日）、秋分の日（9月22日）
    const holidays2020 = getHolidays(2020);
    expect(holidays2020['2020-03-20']).toBe('春分の日');
    expect(holidays2020['2020-09-22']).toBe('秋分の日');
  });

  test('振替休日が正しく計算されること', () => {
    // 2026年のゴールデンウィーク：
    // 5月3日（憲法記念日）が日曜日。
    // 5月4日（みどりの日：月曜日）、5月5日（こどもの日：火曜日）が祝日のため、
    // 連休明けの 5月6日（水曜日）が「振替休日」になる必要がある。
    const holidays2026 = getHolidays(2026);
    expect(holidays2026['2026-05-03']).toBe('憲法記念日');
    expect(holidays2026['2026-05-04']).toBe('みどりの日');
    expect(holidays2026['2026-05-05']).toBe('こどもの日');
    expect(holidays2026['2026-05-06']).toBe('振替休日');
  });

  test('国民の休日が正しく計算されること', () => {
    // 2026年9月：
    // 9月21日（敬老の日：月曜日）、9月23日（秋分の日：水曜日）
    // 祝日に挟まれた 9月22日（火曜日）が「国民の休日」になる。
    const holidays2026 = getHolidays(2026);
    expect(holidays2026['2026-09-21']).toBe('敬老の日');
    expect(holidays2026['2026-09-23']).toBe('秋分の日');
    expect(holidays2026['2026-09-22']).toBe('国民の休日');
  });

  test('特定年のカスタムオーバーライド（2020年/2021年オリンピック特例等）が適用されること', () => {
    // 2020年のオリンピック特例
    const holidays2020 = getHolidays(2020);
    expect(holidays2020['2020-07-23']).toBe('海の日');
    expect(holidays2020['2020-07-24']).toBe('スポーツの日');
    expect(holidays2020['2020-08-10']).toBe('山の日');
    expect(holidays2020['2020-07-20']).toBeUndefined();
    expect(holidays2020['2020-08-11']).toBeUndefined();
    expect(holidays2020['2020-10-12']).toBeUndefined();

    // 2021年のオリンピック特例
    const holidays2021 = getHolidays(2021);
    expect(holidays2021['2021-07-22']).toBe('海の日');
    expect(holidays2021['2021-07-23']).toBe('スポーツの日');
    expect(holidays2021['2021-08-08']).toBe('山の日');
    expect(holidays2021['2021-07-19']).toBeUndefined();
    expect(holidays2021['2021-08-11']).toBeUndefined();
    expect(holidays2021['2021-10-11']).toBeUndefined();
  });

  test('祝日法施行前（1948年未満）や不正な入力に対して安全に空オブジェクトを返すこと', () => {
    expect(getHolidays(1947)).toEqual({});
    expect(getHolidays(1900)).toEqual({});
    expect(getHolidays('invalid')).toEqual({});
  });
});

describe('祝日設定ライフサイクル (initHolidays, reloadHolidays, resetHolidays)', () => {
  beforeEach(() => {
    // キャッシュをリセット
    reloadHolidays();
  });

  test('initHolidays が設定オブジェクトを返し、fallbackConfig のキー構造を持つこと', async () => {
    const config = await initHolidays();
    expect(config).toBeDefined();
    expect(config.fixed).toBeDefined();
    expect(config.happy_mondays).toBeInstanceOf(Array);
    expect(config.emperor_birthdays).toBeInstanceOf(Array);
    expect(config.custom_overrides).toBeDefined();
  });

  test('resetHolidays がデフォルト設定に復元すること', async () => {
    const config = await resetHolidays();
    expect(config).toBeDefined();
    expect(config.fixed['01-01']).toBe('元日');
    expect(config.fixed['05-05']).toBe('こどもの日');
  });
});
