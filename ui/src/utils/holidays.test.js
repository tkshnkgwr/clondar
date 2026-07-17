import { describe, test, expect } from 'vitest';
import { getHolidays } from './holidays';

describe('日本の祝日計算ロジック (getHolidays)', () => {
  
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

  test('ハッピーマンデー制度（新旧）が正しく切り替わること', () => {
    // --- 成人の日 (旧: 1月15日, 新: 1月第2月曜日) ---
    // 1999年（旧制度）：1月15日
    const holidays1999 = getHolidays(1999);
    expect(holidays1999['1999-01-15']).toBe('成人の日');
    expect(holidays1999['1999-01-11']).toBeUndefined(); // 1999年1月第2月曜日は祝日ではない

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

  test('特定年のカスタムオーバーライド（オリンピック特例等）が適用されること', () => {
    // 2020年のオリンピック特例：
    // 海の日が 7月23日 (本来は7月第3月曜日の7月20日)
    // スポーツの日が 7月24日 (本来は10月第2月曜日の10月12日)
    // 山の日が 8月10日 (本来は8月11日)
    const holidays2020 = getHolidays(2020);
    
    expect(holidays2020['2020-07-23']).toBe('海の日');
    expect(holidays2020['2020-07-24']).toBe('スポーツの日');
    expect(holidays2020['2020-08-10']).toBe('山の日');
    
    // 元の予定日が null で上書き（削除）されていることの確認
    expect(holidays2020['2020-07-20']).toBeUndefined();
    expect(holidays2020['2020-08-11']).toBeUndefined();
    expect(holidays2020['2020-10-12']).toBeUndefined();
  });
});
