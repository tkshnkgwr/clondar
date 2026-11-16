/**
 * @fileoverview 月間カレンダー表示コンポーネントモジュール。
 *
 * 常に 6週間 (42日間) の枠組みで安定表示される月間カレンダーグリッドを提供します。
 * `React.memo` による再レンダリング分離、事前計算された日付配列キャッシュ、
 * および日替わり時のみ動作する低負荷タイマーにより、常時待機時の CPU・GC 負荷を最小化します。
 */

import React, { useState, useMemo, useEffect } from 'react';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { useHolidays, formatDateStr } from '../utils/holidays';

const WEEKDAYS = ['日', '月', '火', '水', '木', '金', '土'];

/**
 * 次の午前 0時0分0秒 までのミリ秒を算出します。
 * @returns {number}
 */
function msUntilMidnight() {
  const now = new Date();
  const nextMidnight = new Date(now.getFullYear(), now.getMonth(), now.getDate() + 1, 0, 0, 0, 50);
  return Math.max(1000, nextMidnight.getTime() - now.getTime());
}

/**
 * 月間カレンダーコンポーネントのプロパティ型定義。
 * @typedef {Object} CalendarProps
 * @property {Date} currentMonth - 現在表示している月を表す Date オブジェクト
 * @property {function(Date): void} setCurrentMonth - 表示月を更新するステートセッター関数
 * @property {function(): void} onShowYearly - 年間カレンダーモーダルを表示するコールバック
 * @property {function(): void} onShowHolidays - 祝日設定マネージャーモーダルを表示するコールバック
 * @property {boolean} isTransparent - ウィンドウが背景透過モードであるかどうか
 */

/**
 * 月間カレンダー表示コンポーネント。
 *
 * @param {CalendarProps} props - コンポーネントのプロパティ
 * @returns {JSX.Element} Calendar コンポーネント
 */
export const Calendar = React.memo(({ currentMonth, setCurrentMonth, onShowYearly, onShowHolidays, isTransparent }) => {
  const year = currentMonth.getFullYear();
  const month = currentMonth.getMonth();
  const holidays = useHolidays(year);
  const [todayKey, setTodayKey] = useState(() => formatDateStr(new Date()));
  const [hoveredHoliday, setHoveredHoliday] = useState(null);

  // 日付変更（午前0時）にのみ "今日" ハイライトを更新
  useEffect(() => {
    let timerId;
    const scheduleMidnightUpdate = () => {
      timerId = setTimeout(() => {
        setTodayKey(formatDateStr(new Date()));
        scheduleMidnightUpdate();
      }, msUntilMidnight());
    };
    scheduleMidnightUpdate();
    return () => clearTimeout(timerId);
  }, []);

  // 42日分の日付セルメタデータを事前構築（描画ループでの Date・文字列生成を排除）
  const calendarDays = useMemo(() => {
    const monthStart = new Date(year, month, 1);
    const startDay = monthStart.getDay();
    const curr = new Date(year, month, 1 - startDay);

    const days = [];
    for (let i = 0; i < 42; i++) {
      days.push({
        dateStr: formatDateStr(curr),
        dayOfMonth: curr.getDate(),
        dayOfWeek: curr.getDay(),
        isCurrentMonth: curr.getMonth() === month,
      });
      curr.setDate(curr.getDate() + 1);
    }
    return days;
  }, [year, month]);

  return (
    <div
      className={`w-full max-w-xl transition-colors duration-300 rounded-2xl p-6 shadow-xl border ${
        isTransparent
          ? 'bg-white/30 dark:bg-slate-900/35 border-white/20 dark:border-slate-700/40'
          : 'bg-white dark:bg-slate-900 border-slate-200/80 dark:border-slate-800'
      } relative`}
    >
      <div className="flex items-center justify-between mb-6 gap-4">
        <h2 className="text-2xl font-black text-slate-900 dark:text-white tracking-tight whitespace-nowrap">
          {year}年 {month + 1}月
        </h2>
        <div className="flex items-center gap-2 flex-shrink-0">
          <button
            onClick={onShowYearly}
            className="px-3 py-1 text-xs font-bold bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 rounded-lg hover:bg-blue-100 dark:hover:bg-blue-900/50 transition-colors whitespace-nowrap"
          >
            今年
          </button>
          <button
            onClick={() => setCurrentMonth(new Date())}
            className="px-3 py-1 text-xs font-bold bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors whitespace-nowrap"
          >
            今月
          </button>
          <button
            onClick={onShowHolidays}
            className="px-3 py-1 text-xs font-bold bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors whitespace-nowrap"
          >
            祝日設定
          </button>
          <div className="flex gap-1 ml-1 flex-shrink-0">
            <button
              onClick={() => setCurrentMonth(new Date(year, month - 1, 1))}
              className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors"
            >
              <ChevronLeft size={16} className="text-slate-600 dark:text-slate-200" />
            </button>
            <button
              onClick={() => setCurrentMonth(new Date(year, month + 1, 1))}
              className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors"
            >
              <ChevronRight size={16} className="text-slate-600 dark:text-slate-200" />
            </button>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-7 mb-2">
        {WEEKDAYS.map((day, idx) => (
          <div key={day} className={`text-center text-xs font-black uppercase tracking-wider py-2 ${idx === 0 ? 'text-red-500' : idx === 6 ? 'text-blue-500' : 'text-slate-400'}`}>
            {day}
          </div>
        ))}
      </div>

      <div className="grid grid-cols-7 gap-1">
        {calendarDays.map((cell) => {
          const holidayName = holidays[cell.dateStr];
          const isDayToday = cell.dateStr === todayKey;

          return (
            <div
              key={cell.dateStr}
              onMouseEnter={() => holidayName && setHoveredHoliday({ name: holidayName, dateStr: cell.dateStr })}
              onMouseLeave={() => hoveredHoliday && setHoveredHoliday(null)}
              className={`relative h-12 flex items-center justify-center rounded-lg transition-colors cursor-default ${
                !cell.isCurrentMonth ? 'opacity-20' : 'opacity-100'
              } ${
                isDayToday
                  ? 'bg-blue-500 text-white shadow-lg shadow-blue-500/30 font-bold'
                  : 'hover:bg-slate-100 dark:hover:bg-slate-800'
              }`}
            >
              <span
                className={`text-base font-medium ${
                  !isDayToday && holidayName
                    ? 'text-red-500 font-bold'
                    : !isDayToday && cell.dayOfWeek === 0
                    ? 'text-red-500'
                    : !isDayToday && cell.dayOfWeek === 6
                    ? 'text-blue-500'
                    : 'text-slate-700 dark:text-slate-300'
                }`}
              >
                {cell.dayOfMonth}
              </span>
              {holidayName && <div className="absolute bottom-1 w-1 h-1 bg-red-400 rounded-full" />}

              {hoveredHoliday && hoveredHoliday.dateStr === cell.dateStr && (
                <div className="animate-tooltip-in absolute bottom-full mb-2 z-50 px-3 py-1.5 bg-slate-900 dark:bg-white text-white dark:text-slate-900 text-xs font-bold rounded-lg shadow-xl whitespace-nowrap pointer-events-none">
                  {holidayName}
                  <div className="absolute top-full left-1/2 -translate-x-1/2 border-8 border-transparent border-t-slate-900 dark:border-t-white" />
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
});
