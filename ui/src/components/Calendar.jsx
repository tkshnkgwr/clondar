import React, { useState, useMemo } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronLeft, ChevronRight, X } from 'lucide-react';
import { getHolidays } from '../utils/holidays';

export const Calendar = ({ currentMonth, setCurrentMonth, onShowYearly, isTransparent }) => {
  const year = currentMonth.getFullYear();
  const holidays = useMemo(() => getHolidays(year), [year]);
  
  const monthStart = new Date(year, currentMonth.getMonth(), 1);
  const startDay = monthStart.getDay();
  const startDate = new Date(monthStart);
  startDate.setDate(startDate.getDate() - startDay);
  
  const calendarDays = [];
  let curr = new Date(startDate);
  for (let i = 0; i < 42; i++) { 
    calendarDays.push(new Date(curr)); 
    curr.setDate(curr.getDate() + 1); 
  }

  const [hoveredHoliday, setHoveredHoliday] = useState(null);

  const formatDate = (d) => `${d.getFullYear()}-${(d.getMonth() + 1).toString().padStart(2, '0')}-${d.getDate().toString().padStart(2, '0')}`;

  return (
    <div className={`w-full max-w-xl transition-all duration-300 glass rounded-2xl p-6 shadow-xl border border-white/20 dark:border-slate-700/30 relative ${isTransparent ? 'bg-white/20 dark:bg-slate-900/20' : 'bg-white/80 dark:bg-slate-900/80'}`}>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-black text-slate-900 dark:text-white tracking-tight">
          {year}年 {currentMonth.getMonth() + 1}月
        </h2>
        <div className="flex items-center gap-2">
          <button onClick={onShowYearly} className="px-3 py-1 text-xs font-bold bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 rounded-lg hover:bg-blue-100 dark:hover:bg-blue-900/50 transition-colors">
            今年
          </button>
          <button onClick={() => setCurrentMonth(new Date())} className="px-3 py-1 text-xs font-bold bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors">
            今月
          </button>
          <div className="flex gap-1 ml-1">
            <button onClick={() => setCurrentMonth(new Date(year, currentMonth.getMonth() - 1, 1))} className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors">
              <ChevronLeft size={16} className="text-slate-600 dark:text-slate-200" />
            </button>
            <button onClick={() => setCurrentMonth(new Date(year, currentMonth.getMonth() + 1, 1))} className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors">
              <ChevronRight size={16} className="text-slate-600 dark:text-slate-200" />
            </button>
          </div>
        </div>
      </div>
      <div className="grid grid-cols-7 mb-2">
        {['日', '月', '火', '水', '木', '金', '土'].map((day, idx) => (
          <div key={day} className={`text-center text-xs font-black uppercase tracking-wider py-2 ${idx === 0 ? 'text-red-500' : idx === 6 ? 'text-blue-500' : 'text-slate-400'}`}>
            {day}
          </div>
        ))}
      </div>
      <div className="grid grid-cols-7 gap-1">
        {calendarDays.map((day) => {
          const dateStr = formatDate(day);
          const holidayName = holidays[dateStr];
          const isCurrentMonth = day.getMonth() === currentMonth.getMonth();
          const today = new Date();
          const isDayToday = day.getDate() === today.getDate() && day.getMonth() === today.getMonth() && day.getFullYear() === today.getFullYear();
          return (
            <div key={day.toString()} 
                 onMouseEnter={() => holidayName && setHoveredHoliday({ name: holidayName, dateStr })}
                 onMouseLeave={() => setHoveredHoliday(null)}
                 className={`relative h-12 flex items-center justify-center rounded-lg transition-all group cursor-default ${!isCurrentMonth ? 'opacity-20' : 'opacity-100'} ${isDayToday ? 'bg-blue-500 text-white shadow-lg shadow-blue-500/30 font-bold' : 'hover:bg-slate-100 dark:hover:bg-slate-800'}`}>
              <span className={`text-base font-medium ${!isDayToday && holidayName ? 'text-red-500 font-bold' : !isDayToday && day.getDay() === 0 ? 'text-red-500' : !isDayToday && day.getDay() === 6 ? 'text-blue-500' : 'text-slate-700 dark:text-slate-300'}`}>
                {day.getDate()}
              </span>
              {holidayName && <div className="absolute bottom-1 w-1 h-1 bg-red-400 rounded-full" />}
              
              <AnimatePresence>
                {hoveredHoliday && hoveredHoliday.dateStr === dateStr && (
                  <motion.div 
                    initial={{ opacity: 0, y: 10 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: 10 }}
                    className="absolute bottom-full mb-2 z-50 px-3 py-1.5 bg-slate-900 dark:bg-white text-white dark:text-slate-900 text-xs font-bold rounded-lg shadow-xl whitespace-nowrap pointer-events-none"
                  >
                    {holidayName}
                    <div className="absolute top-full left-1/2 -translate-x-1/2 border-8 border-transparent border-t-slate-900 dark:border-t-white" />
                  </motion.div>
                )}
              </AnimatePresence>
            </div>
          );
        })}
      </div>
    </div>
  );
};

export const YearlyView = ({ year: initialYear, onClose }) => {
  const [currentYear, setCurrentYear] = useState(initialYear);
  const months = Array.from({ length: 12 }, (_, i) => i);
  const holidays = useMemo(() => getHolidays(currentYear), [currentYear]);
  const formatDate = (d) => `${d.getFullYear()}-${(d.getMonth() + 1).toString().padStart(2, '0')}-${d.getDate().toString().padStart(2, '0')}`;
  const [hoveredHoliday, setHoveredHoliday] = useState(null);

  return (
    <motion.div 
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.95 }}
      className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-md"
    >
      <div className="bg-white dark:bg-slate-900 w-full max-w-6xl max-h-[90vh] rounded-3xl shadow-2xl overflow-hidden flex flex-col border border-white/20 dark:border-slate-800">
        <div className="p-6 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between">
          <div className="flex items-center gap-6">
            <h2 className="text-3xl font-black text-slate-900 dark:text-white">{currentYear}年 年間カレンダー</h2>
            <div className="flex bg-slate-100 dark:bg-slate-800 p-1 rounded-xl shadow-inner">
              <button onClick={() => setCurrentYear(currentYear - 1)} className="p-2 hover:bg-white dark:hover:bg-slate-700 rounded-lg transition-all text-slate-600 dark:text-slate-400">
                <ChevronLeft size={20} />
              </button>
              <button onClick={() => setCurrentYear(new Date().getFullYear())} className="px-3 py-1 text-xs font-bold text-slate-500 hover:text-blue-500 transition-colors">
                今年
              </button>
              <button onClick={() => setCurrentYear(currentYear + 1)} className="p-2 hover:bg-white dark:hover:bg-slate-700 rounded-lg transition-all text-slate-600 dark:text-slate-400">
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
            {months.map(m => (
              <div key={m} className="flex flex-col gap-3">
                <h3 className="text-lg font-bold text-blue-600 dark:text-blue-400 ml-2">{m + 1}月</h3>
                <div className="grid grid-cols-7 gap-1">
                  {['日', '月', '火', '水', '木', '金', '土'].map((d, i) => (
                    <div key={d} className={`text-[10px] font-bold text-center py-1 ${i === 0 ? 'text-red-500' : i === 6 ? 'text-blue-500' : 'text-slate-400'}`}>{d}</div>
                  ))}
                  {(() => {
                    const start = new Date(currentYear, m, 1).getDay();
                    const days = new Date(currentYear, m + 1, 0).getDate();
                    const cells = [];
                    for (let i = 0; i < start; i++) cells.push(<div key={`empty-${i}`} />);
                    for (let d = 1; d <= days; d++) {
                      const date = new Date(currentYear, m, d);
                      const dateStr = formatDate(date);
                      const holidayName = holidays[dateStr];
                      const isSun = date.getDay() === 0;
                      const isSat = date.getDay() === 6;
                      const isToday = new Date().toDateString() === date.toDateString();
                      const key = `${m}-${d}`;
                      cells.push(
                        <div key={key} 
                             onMouseEnter={() => holidayName && setHoveredHoliday({ name: holidayName, month: m, day: d })}
                             onMouseLeave={() => setHoveredHoliday(null)}
                             className={`relative flex items-center justify-center text-xs h-7 rounded-md transition-colors ${isToday ? 'bg-blue-500 text-white font-bold' : holidayName || isSun ? 'text-red-500 font-bold' : isSat ? 'text-blue-500' : 'text-slate-600 dark:text-slate-400'} ${holidayName ? 'hover:bg-red-50 dark:hover:bg-red-900/20' : 'hover:bg-slate-100 dark:hover:bg-slate-800'}`}>
                          {d}
                          {holidayName && <div className="absolute bottom-0.5 w-1 h-1 bg-red-400 rounded-full" />}
                          
                          <AnimatePresence>
                            {hoveredHoliday && hoveredHoliday.name === holidayName && hoveredHoliday.month === m && hoveredHoliday.day === d && (
                              <motion.div 
                                initial={{ opacity: 0, y: 10 }}
                                animate={{ opacity: 1, y: 0 }}
                                exit={{ opacity: 0, y: 10 }}
                                className="absolute bottom-full mb-2 z-[150] px-3 py-1.5 bg-slate-900 dark:bg-white text-white dark:text-slate-900 text-[10px] font-bold rounded-lg shadow-2xl whitespace-nowrap pointer-events-none"
                              >
                                {holidayName}
                                <div className="absolute top-full left-1/2 -translate-x-1/2 border-8 border-transparent border-t-slate-900 dark:border-t-white" />
                              </motion.div>
                            )}
                          </AnimatePresence>
                        </div>
                      );
                    }
                    return cells;
                  })()}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </motion.div>
  );
};
