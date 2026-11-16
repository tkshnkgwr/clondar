/**
 * @fileoverview 年間カレンダー表示モーダルコンポーネントモジュール。
 *
 * `React.lazy` による遅延ロードに対応するため `Calendar.jsx` から分離されています。
 * 12ヶ月分（365日）の日付グリッド構築を `useMemo` で年単位キャッシュし、
 * ホバー操作時の再計算やループ内オブジェクト生成を抑制しています。
 */

import React, { useState, useMemo } from 'react';
import { ChevronLeft, ChevronRight, X } from 'lucide-react';
import { useHolidays, formatDateStr } from '../utils/holidays';

const WEEKDAYS = ['日', '月', '火', '水', '木', '金', '土'];

/**
 * 年間カレンダービューのプロパティ型定義。
 * @typedef {Object} YearlyViewProps
 * @property {number} year - 初期表示する年 (西暦, 例: 2026)
 * @property {function(): void} onClose - モーダルを閉じるコールバック関数
 */

/**
 * 年間カレンダー表示コンポーネント（モーダル形式）。
 *
 * @param {YearlyViewProps} props - コンポーネントのプロパティ
 * @returns {JSX.Element} YearlyView コンポーネント
 */
export default function YearlyView({ year: initialYear, onClose }) {
  const [currentYear, setCurrentYear] = useState(initialYear);
  const holidays = useHolidays(currentYear);
  const [hoveredHoliday, setHoveredHoliday] = useState(null);
  const todayStr = formatDateStr(new Date());

  // 12ヶ月分のセル構造を currentYear ごとに1回だけ生成
  const monthsData = useMemo(() => {
    return Array.from({ length: 12 }, (_, m) => {
      const startDay = new Date(currentYear, m, 1).getDay();
      const totalDays = new Date(currentYear, m + 1, 0).getDate();
      const days = [];
      for (let d = 1; d <= totalDays; d++) {
        const date = new Date(currentYear, m, d);
        const dow = date.getDay();
        days.push({
          day: d,
          dateStr: formatDateStr(date),
          isSun: dow === 0,
          isSat: dow === 6,
        });
      }
      return { monthIndex: m, startDay, days };
    });
  }, [currentYear]);

  return (
    <div
      className="animate-modal-in fixed inset-0 z-[100] flex items-center justify-center p-4 bg-slate-900/75"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div className="bg-white dark:bg-slate-900 w-full max-w-6xl max-h-[90vh] rounded-3xl shadow-2xl overflow-hidden flex flex-col border border-white/20 dark:border-slate-800">
        <div className="p-6 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between">
          <div className="flex items-center gap-6">
            <h2 className="text-3xl font-black text-slate-900 dark:text-white">{currentYear}年 年間カレンダー</h2>
            <div className="flex bg-slate-100 dark:bg-slate-800 p-1 rounded-xl shadow-inner">
              <button
                onClick={() => setCurrentYear((y) => y - 1)}
                className="p-2 hover:bg-white dark:hover:bg-slate-700 rounded-lg transition-colors text-slate-600 dark:text-slate-400"
              >
                <ChevronLeft size={20} />
              </button>
              <button
                onClick={() => setCurrentYear(new Date().getFullYear())}
                className="px-3 py-1 text-xs font-bold text-slate-500 hover:text-blue-500 transition-colors"
              >
                今年
              </button>
              <button
                onClick={() => setCurrentYear((y) => y + 1)}
                className="p-2 hover:bg-white dark:hover:bg-slate-700 rounded-lg transition-colors text-slate-600 dark:text-slate-400"
              >
                <ChevronRight size={20} />
              </button>
            </div>
          </div>
          <button onClick={onClose} className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors text-slate-500">
            <X size={24} />
          </button>
        </div>

        <div className="flex-1 overflow-y-auto p-12">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
            {monthsData.map(({ monthIndex, startDay, days }) => (
              <div key={monthIndex} className="flex flex-col gap-3">
                <h3 className="text-lg font-bold text-blue-600 dark:text-blue-400 ml-2">{monthIndex + 1}月</h3>
                <div className="grid grid-cols-7 gap-1">
                  {WEEKDAYS.map((d, i) => (
                    <div key={d} className={`text-[10px] font-bold text-center py-1 ${i === 0 ? 'text-red-500' : i === 6 ? 'text-blue-500' : 'text-slate-400'}`}>
                      {d}
                    </div>
                  ))}
                  {Array.from({ length: startDay }, (_, i) => (
                    <div key={`empty-${i}`} />
                  ))}
                  {days.map(({ day, dateStr, isSun, isSat }) => {
                    const holidayName = holidays[dateStr];
                    const isToday = dateStr === todayStr;

                    return (
                      <div
                        key={dateStr}
                        onMouseEnter={() => holidayName && setHoveredHoliday({ name: holidayName, dateStr })}
                        onMouseLeave={() => hoveredHoliday && setHoveredHoliday(null)}
                        className={`relative flex items-center justify-center text-xs h-7 rounded-md transition-colors ${
                          isToday
                            ? 'bg-blue-500 text-white font-bold'
                            : holidayName || isSun
                            ? 'text-red-500 font-bold'
                            : isSat
                            ? 'text-blue-500'
                            : 'text-slate-600 dark:text-slate-400'
                        } ${holidayName ? 'hover:bg-red-50 dark:hover:bg-red-900/20' : 'hover:bg-slate-100 dark:hover:bg-slate-800'}`}
                      >
                        {day}
                        {holidayName && <div className="absolute bottom-0.5 w-1 h-1 bg-red-400 rounded-full" />}

                        {hoveredHoliday && hoveredHoliday.dateStr === dateStr && (
                          <div className="animate-tooltip-in absolute bottom-full mb-2 z-[150] px-3 py-1.5 bg-slate-900 dark:bg-white text-white dark:text-slate-900 text-[10px] font-bold rounded-lg shadow-2xl whitespace-nowrap pointer-events-none">
                            {holidayName}
                            <div className="absolute top-full left-1/2 -translate-x-1/2 border-8 border-transparent border-t-slate-900 dark:border-t-white" />
                          </div>
                        )}
                      </div>
                    );
                  })}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
