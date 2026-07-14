import React from 'react';
import { motion } from 'framer-motion';

/**
 * デジタル時計コンポーネント。
 * 指定された日時オブジェクトに基づいて、時間（AM/PMを含む）をデジタル表示します。
 *
 * @param {Object} props - コンポーネントのプロパティ
 * @param {Date} props.time - 表示する現在時刻の Date オブジェクト
 * @param {boolean} props.is24Hour - 24時間表記にするかどうか（false の場合は 12時間表記）
 * @param {boolean} props.showSeconds - 秒表示を有効にするかどうか
 * @returns {JSX.Element} DigitalClock コンポーネント
 */
export const DigitalClock = ({ time, is24Hour, showSeconds }) => {
  const options = { 
    hour: '2-digit', 
    minute: '2-digit', 
    second: showSeconds ? '2-digit' : undefined, 
    hour12: !is24Hour 
  };
  const timeStr = new Intl.DateTimeFormat('en-US', options).format(time);
  const parts = timeStr.split(' ');
  const mainTime = parts[0];
  const amPm = parts[1];

  return (
    <div className="flex flex-col items-center justify-center select-none">
      <div className="h-8 flex items-center justify-center">
        {!is24Hour && amPm && (
          <div className="text-xl font-bold text-slate-500 dark:text-slate-400 uppercase tracking-[0.2em]">
            {amPm}
          </div>
        )}
      </div>
      <div className="text-8xl font-normal text-blue-600 dark:text-white drop-shadow-md flex items-center justify-center min-w-[450px]" 
           style={{ fontFamily: 'Impact, Haettenschweiler, "Arial Narrow Bold", sans-serif', lineHeight: '1', letterSpacing: '0.02em' }}>
        {mainTime.split('').map((char, index) => (
          <span key={index} className="inline-block text-center" style={{ width: char === ':' ? '0.25em' : '0.7em', fontVariantNumeric: 'tabular-nums' }}>
            {char}
          </span>
        ))}
      </div>
      <div className="h-8" />
    </div>
  );
};

/**
 * アナログ時計コンポーネント。
 * 指定された日時オブジェクトに基づいて、時針・分針・秒針をアニメーション表示します。
 *
 * @param {Object} props - コンポーネントのプロパティ
 * @param {Date} props.time - 表示する現在時刻の Date オブジェクト
 * @param {boolean} props.showSeconds - 秒針を表示するかどうか
 * @returns {JSX.Element} AnalogClock コンポーネント
 */
export const AnalogClock = ({ time, showSeconds }) => {
  const seconds = time.getSeconds() + time.getMilliseconds() / 1000;
  const minutes = time.getMinutes() + seconds / 60;
  const hours = (time.getHours() % 12) + minutes / 60;

  return (
    <div className="relative w-56 h-56 rounded-full border-2 border-slate-200 dark:border-slate-700 bg-white/30 dark:bg-slate-800/30 glass shadow-inner flex items-center justify-center">
      {[...Array(12)].map((_, i) => {
        const isMain = i % 3 === 0;
        const h = isMain ? 16 : 12;
        const w = isMain ? 6 : 4;
        return (
          <div key={i} className={`absolute rounded-full ${isMain ? 'bg-blue-500' : 'bg-slate-400 dark:bg-slate-600'}`}
            style={{
              width: `${w}px`, 
              height: `${h}px`, 
              top: '50%', 
              left: '50%',
              marginTop: `-${h / 2}px`, 
              marginLeft: `-${w / 2}px`,
              transform: `rotate(${i * 30}deg) translateY(-92px)`,
            }}
          />
        );
      })}
      <motion.div className="absolute w-1.5 h-14 bg-slate-800 dark:bg-slate-100 rounded-full origin-bottom z-3" style={{ bottom: '50%', rotate: `${hours * 30}deg` }} />
      <motion.div className="absolute w-1 h-20 bg-slate-500 dark:bg-slate-400 rounded-full origin-bottom z-2" style={{ bottom: '50%', rotate: `${minutes * 6}deg` }} />
      {showSeconds && <motion.div className="absolute w-0.5 h-24 bg-red-500 rounded-full origin-bottom z-4" style={{ bottom: '50%', rotate: `${seconds * 6}deg` }} />}
      <div className="absolute w-3.5 h-3.5 bg-slate-800 dark:bg-slate-100 rounded-full z-10 shadow-sm border-2 border-white dark:border-slate-800" />
    </div>
  );
};
