const html = window.htm.bind(window.React.createElement);
const { useState, useMemo } = window.React;
const React = window.React;
const fMotion = window.Motion || window.framerMotion;
const motion = fMotion ? fMotion.motion : new Proxy({}, {
  get: (target, prop) => (props) => React.createElement(prop, props)
});
const AnimatePresence = fMotion ? fMotion.AnimatePresence : ({children}) => React.createElement(React.Fragment, null, children);

import { getHolidays } from '../utils/holidays.js';

export const Icon = ({ name, size = 20, className = "" }) => {
  const icons = {
    clock: html`<path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10zM12 6v6l4 2" />`,
    calendar: html`<rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10" />`,
    'chevron-left': html`<polyline points="15 18 9 12 15 6" />`,
    'chevron-right': html`<polyline points="9 18 15 12 9 6" />`,
    sun: html`<circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />`,
    moon: html`<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />`,
    pin: html`<path d="M12 17v5M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z" />`,
    x: html`<line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18" />`,
    home: html`<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" /><polyline points="9 22 9 12 15 12 15 22" />`,
  };
  return html`
    <svg xmlns="http://www.w3.org/2000/svg" width=${size} height=${size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className=${className}>
      ${icons[name]}
    </svg>
  `;
};

export const Calendar = ({ currentMonth, setCurrentMonth, isDark, onShowYearly, isTransparent }) => {
  const holidays = useMemo(() => getHolidays(currentMonth.getFullYear()), [currentMonth.getFullYear()]);
  const monthStart = new Date(currentMonth.getFullYear(), currentMonth.getMonth(), 1);
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

  return html`
    <div className=${`w-full max-w-xl transition-all duration-300 glass rounded-2xl p-6 shadow-xl border border-white/20 dark:border-slate-700/30 relative ${isTransparent ? 'bg-white/20 dark:bg-slate-900/20' : 'bg-white/80 dark:bg-slate-900/80'}`}>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-black text-slate-900 dark:text-white tracking-tight">${currentMonth.getFullYear()}年 ${currentMonth.getMonth() + 1}月</h2>
        <div className="flex items-center gap-2">
          <button onClick=${onShowYearly} className="px-3 py-1 text-xs font-bold bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 rounded-lg hover:bg-blue-100 dark:hover:bg-blue-900/50 transition-colors">今年</button>
          <button 
            onClick=${() => setCurrentMonth(new Date())} 
            className="p-1.5 bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors flex items-center justify-center"
            title="今月"
          >
            <${Icon} name="home" size=${14} />
          </button>
          <div className="flex gap-1 ml-1">
            <button onClick=${() => setCurrentMonth(new Date(currentMonth.getFullYear(), currentMonth.getMonth() - 1, 1))} className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors"><${Icon} name="chevron-left" className="text-slate-600 dark:text-slate-200" /></button>
            <button onClick=${() => setCurrentMonth(new Date(currentMonth.getFullYear(), currentMonth.getMonth() + 1, 1))} className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors"><${Icon} name="chevron-right" className="text-slate-600 dark:text-slate-200" /></button>
          </div>
        </div>
      </div>
      <div className="grid grid-cols-7 mb-2">
        ${['日', '月', '火', '水', '木', '金', '土'].map((day, idx) => html`
          <div key=${day} className=${`text-center text-xs font-black uppercase tracking-wider py-2 ${idx === 0 ? 'text-red-500' : idx === 6 ? 'text-blue-500' : 'text-slate-400'}`}>${day}</div>
        `)}
      </div>
      <div className="grid grid-cols-7 gap-1">
        ${calendarDays.map((day) => {
          const dateStr = formatDate(day);
          const holidayName = holidays[dateStr];
          const isCurrentMonth = day.getMonth() === currentMonth.getMonth();
          const today = new Date();
          const isDayToday = day.getDate() === today.getDate() && day.getMonth() === today.getMonth() && day.getFullYear() === today.getFullYear();
          return html`
            <div key=${day.toString()} 
                 onMouseEnter=${() => holidayName && setHoveredHoliday({ name: holidayName, dateStr })}
                 onMouseLeave=${() => setHoveredHoliday(null)}
                 className=${`relative h-12 flex items-center justify-center rounded-lg transition-all group cursor-default ${!isCurrentMonth ? 'opacity-20' : 'opacity-100'} ${isDayToday ? 'bg-blue-500 text-white shadow-lg shadow-blue-500/30 font-bold' : 'hover:bg-slate-100 dark:hover:bg-slate-800'}`}>
              <span className=${`text-base font-medium ${!isDayToday && holidayName ? 'text-red-500 font-bold' : !isDayToday && day.getDay() === 0 ? 'text-red-500' : !isDayToday && day.getDay() === 6 ? 'text-blue-500' : 'text-slate-700 dark:text-slate-300'}`}>${day.getDate()}</span>
              ${holidayName && html`<div className="absolute bottom-1 w-1 h-1 bg-red-400 rounded-full" />`}
              
              <${AnimatePresence}>
                ${hoveredHoliday && hoveredHoliday.dateStr === dateStr && html`
                  <${motion.div} 
                    initial=${{ opacity: 0, y: 10 }}
                    animate=${{ opacity: 1, y: 0 }}
                    exit=${{ opacity: 0, y: 10 }}
                    className="absolute bottom-full mb-2 z-50 px-3 py-1.5 bg-slate-900 dark:bg-white text-white dark:text-slate-900 text-xs font-bold rounded-lg shadow-xl whitespace-nowrap pointer-events-none"
                  >
                    ${holidayName}
                    <div className="absolute top-full left-1/2 -translate-x-1/2 border-8 border-transparent border-t-slate-900 dark:border-t-white" />
                  </${motion.div}>
                `}
              </${AnimatePresence}>
            </div>
          `;
        })}
      </div>
    </div>
  `;
};

export const YearlyView = ({ year: initialYear, onClose, isDark }) => {
  const [currentYear, setCurrentYear] = useState(initialYear);
  const months = Array.from({ length: 12 }, (_, i) => i);
  const holidays = useMemo(() => getHolidays(currentYear), [currentYear]);
  const formatDate = (d) => `${d.getFullYear()}-${(d.getMonth() + 1).toString().padStart(2, '0')}-${d.getDate().toString().padStart(2, '0')}`;
  const [hoveredHoliday, setHoveredHoliday] = useState(null);

  return html`
    <${motion.div} 
      initial=${{ opacity: 0, scale: 0.95 }}
      animate=${{ opacity: 1, scale: 1 }}
      exit=${{ opacity: 0, scale: 0.95 }}
      className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-md"
    >
      <div className="bg-white dark:bg-slate-900 w-full max-w-6xl max-h-[90vh] rounded-3xl shadow-2xl overflow-hidden flex flex-col border border-white/20 dark:border-slate-800">
        <div className="p-6 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between">
          <div className="flex items-center gap-6">
            <h2 className="text-3xl font-black text-slate-900 dark:text-white">${currentYear}年 年間カレンダー</h2>
            <div className="flex bg-slate-100 dark:bg-slate-800 p-1 rounded-xl shadow-inner">
              <button onClick=${() => setCurrentYear(currentYear - 1)} className="p-2 hover:bg-white dark:hover:bg-slate-700 rounded-lg transition-all text-slate-600 dark:text-slate-400"><${Icon} name="chevron-left" size=${20} /></button>
              <button onClick=${() => setCurrentYear(new Date().getFullYear())} className="px-3 py-1 text-xs font-bold text-slate-500 hover:text-blue-500 transition-colors">今年</button>
              <button onClick=${() => setCurrentYear(currentYear + 1)} className="p-2 hover:bg-white dark:hover:bg-slate-700 rounded-lg transition-all text-slate-600 dark:text-slate-400"><${Icon} name="chevron-right" size=${20} /></button>
            </div>
          </div>
          <button onClick=${onClose} className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors text-slate-500"><${Icon} name="x" size=${24} /></button>
        </div>
        <div className="flex-1 overflow-y-auto p-12">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
            ${months.map(m => html`
              <div key=${m} className="flex flex-col gap-3">
                <h3 className="text-lg font-bold text-blue-600 dark:text-blue-400 ml-2">${m + 1}月</h3>
                <div className="grid grid-cols-7 gap-1">
                  ${['日', '月', '火', '水', '木', '金', '土'].map((d, i) => html`
                    <div key=${d} className=${`text-[10px] font-bold text-center py-1 ${i === 0 ? 'text-red-500' : i === 6 ? 'text-blue-500' : 'text-slate-400'}`}>${d}</div>
                  `)}
                  ${(() => {
                    const start = new Date(currentYear, m, 1).getDay();
                    const days = new Date(currentYear, m + 1, 0).getDate();
                    const cells = [];
                    for (let i = 0; i < start; i++) cells.push(html`<div key=${`empty-${i}`} />`);
                    for (let d = 1; d <= days; d++) {
                      const date = new Date(currentYear, m, d);
                      const dateStr = formatDate(date);
                      const holidayName = holidays[dateStr];
                      const isSun = date.getDay() === 0;
                      const isSat = date.getDay() === 6;
                      const isToday = new Date().toDateString() === date.toDateString();
                      cells.push(html`
                        <div key=${d} 
                             onMouseEnter=${() => holidayName && setHoveredHoliday({ name: holidayName, month: m, day: d })}
                             onMouseLeave=${() => setHoveredHoliday(null)}
                             className=${`relative flex items-center justify-center text-xs h-7 rounded-md transition-colors ${isToday ? 'bg-blue-500 text-white font-bold' : holidayName || isSun ? 'text-red-500 font-bold' : isSat ? 'text-blue-500' : 'text-slate-600 dark:text-slate-400'} ${holidayName ? 'hover:bg-red-50 dark:hover:bg-red-900/20' : 'hover:bg-slate-100 dark:hover:bg-slate-800'}`}>
                          ${d}
                          ${holidayName && html`<div className="absolute bottom-0.5 w-1 h-1 bg-red-400 rounded-full" />`}
                          
                          <${AnimatePresence}>
                            ${hoveredHoliday && hoveredHoliday.name === holidayName && hoveredHoliday.month === m && hoveredHoliday.day === d && html`
                              <${motion.div} 
                                initial=${{ opacity: 0, y: 10 }}
                                animate=${{ opacity: 1, y: 0 }}
                                exit=${{ opacity: 0, y: 10 }}
                                className="absolute bottom-full mb-2 z-[150] px-3 py-1.5 bg-slate-900 dark:bg-white text-white dark:text-slate-900 text-[10px] font-bold rounded-lg shadow-2xl whitespace-nowrap pointer-events-none"
                              >
                                ${holidayName}
                                <div className="absolute top-full left-1/2 -translate-x-1/2 border-8 border-transparent border-t-slate-900 dark:border-t-white" />
                              </${motion.div}>
                            `}
                          </${AnimatePresence}>
                        </div>
                      `);
                    }
                    return cells;
                  })()}
                </div>
              </div>
            `)}
          </div>
        </div>
      </div>
    </motion.div>
  `;
};
