/**
 * @fileoverview 時計表示コンポーネントモジュール。
 *
 * 等幅数字フォントによる揺れのないデジタル時計（12H/24H、秒表示ON/OFF）および
 * スムーズなスイープ運針アニメーションを備えたアナログ時計を提供します。
 * 時刻更新ステートおよびタイマーを各時計コンポーネント内部に局所化し、
 * 親コンポーネントやカレンダーへの不要な再レンダリング波及を完全に遮断します。
 */

import React, { useState, useEffect, useLayoutEffect, useRef } from 'react';

/**
 * キャッシュ済みの Intl.DateTimeFormat インスタンス群（毎フレームのオブジェクト生成を防止）。
 */
const TIME_FORMATTERS = {
  '24-sec': new Intl.DateTimeFormat('en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false }),
  '24-min': new Intl.DateTimeFormat('en-US', { hour: '2-digit', minute: '2-digit', hour12: false }),
  '12-sec': new Intl.DateTimeFormat('en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: true }),
  '12-min': new Intl.DateTimeFormat('en-US', { hour: '2-digit', minute: '2-digit', hour12: true }),
};

/**
 * アナログ時計の12時間分の目盛り定義（毎描画の配列生成を防止）。
 */
const CLOCK_TICKS = Array.from({ length: 12 }, (_, i) => {
  const isMain = i % 3 === 0;
  const h = isMain ? 16 : 12;
  const w = isMain ? 6 : 4;
  return {
    index: i,
    isMain,
    style: {
      width: `${w}px`,
      height: `${h}px`,
      top: '50%',
      left: '50%',
      marginTop: `-${h / 2}px`,
      marginLeft: `-${w / 2}px`,
      transform: `rotate(${i * 30}deg) translateY(-92px)`,
    },
  };
});

/**
 * 現在時刻を指定フォーマット文字列に整形します。
 * @param {Date} date
 * @param {boolean} is24Hour
 * @param {boolean} showSeconds
 * @returns {string}
 */
function formatDisplayTime(date, is24Hour, showSeconds) {
  const key = `${is24Hour ? '24' : '12'}-${showSeconds ? 'sec' : 'min'}`;
  return TIME_FORMATTERS[key].format(date);
}

/**
 * デジタル時計のプロパティ型定義。
 * @typedef {Object} DigitalClockProps
 * @property {boolean} is24Hour - 24時間表記を有効にするか（false 時は 12時間表記 + AM/PM）
 * @property {boolean} showSeconds - 秒表示を有効にするかどうか
 */

/**
 * デジタル時計コンポーネント。
 *
 * ## 特徴
 * - `Impact` 系フォントおよび `tabular-nums`（等幅数字）設定により、秒の更新によるレイアウトの揺れを防止。
 * - 表示文字列が変化したときのみ再描画するため、秒非表示時は 1分に1回のみ更新されます。
 *
 * @param {DigitalClockProps} props - コンポーネントのプロパティ
 * @returns {JSX.Element} DigitalClock コンポーネント
 */
export const DigitalClock = React.memo(({ is24Hour, showSeconds }) => {
  const [timeStr, setTimeStr] = useState(() => formatDisplayTime(new Date(), is24Hour, showSeconds));

  useEffect(() => {
    const update = () => {
      const next = formatDisplayTime(new Date(), is24Hour, showSeconds);
      setTimeStr((prev) => (prev === next ? prev : next));
    };

    update();
    const intervalMs = showSeconds ? 250 : 1000;
    const timer = setInterval(update, intervalMs);
    return () => clearInterval(timer);
  }, [is24Hour, showSeconds]);

  const spaceIdx = timeStr.indexOf(' ');
  const mainTime = spaceIdx === -1 ? timeStr : timeStr.slice(0, spaceIdx);
  const amPm = spaceIdx === -1 ? '' : timeStr.slice(spaceIdx + 1);

  return (
    <div className="flex flex-col items-center justify-center select-none">
      <div className="h-8 flex items-center justify-center">
        {!is24Hour && amPm && (
          <div className="text-xl font-bold text-slate-500 dark:text-slate-400 uppercase tracking-[0.2em]">
            {amPm}
          </div>
        )}
      </div>
      <div
        className="text-8xl font-normal text-blue-600 dark:text-white drop-shadow-md flex items-center justify-center min-w-[450px]"
        style={{ fontFamily: 'Impact, Haettenschweiler, "Arial Narrow Bold", sans-serif', lineHeight: '1', letterSpacing: '0.02em' }}
      >
        {mainTime.split('').map((char, index) => (
          <span key={index} className="inline-block text-center" style={{ width: char === ':' ? '0.25em' : '0.7em', fontVariantNumeric: 'tabular-nums' }}>
            {char}
          </span>
        ))}
      </div>
      <div className="h-8" />
    </div>
  );
});

/**
 * アナログ時計のプロパティ型定義。
 * @typedef {Object} AnalogClockProps
 * @property {boolean} showSeconds - 秒針を表示するかどうか
 */

/**
 * アナログ時計コンポーネント。
 *
 * ## 特徴
 * - `useRef` による直接 CSS `transform` 更新を採用し、100ms スイープ運針時も React の再レンダリングを一切発生させません。
 * - `backdrop-filter: blur` を排した軽量フラット背景デザイン。
 *
 * @param {AnalogClockProps} props - コンポーネントのプロパティ
 * @returns {JSX.Element} AnalogClock コンポーネント
 */
export const AnalogClock = React.memo(({ showSeconds }) => {
  const hourHandRef = useRef(null);
  const minHandRef = useRef(null);
  const secHandRef = useRef(null);

  useLayoutEffect(() => {
    const updateHands = () => {
      const now = new Date();
      const seconds = now.getSeconds() + now.getMilliseconds() / 1000;
      const minutes = now.getMinutes() + seconds / 60;
      const hours = (now.getHours() % 12) + minutes / 60;

      if (hourHandRef.current) {
        hourHandRef.current.style.transform = `rotate(${hours * 30}deg)`;
      }
      if (minHandRef.current) {
        minHandRef.current.style.transform = `rotate(${minutes * 6}deg)`;
      }
      if (secHandRef.current) {
        secHandRef.current.style.transform = `rotate(${seconds * 6}deg)`;
      }
    };

    updateHands();
    const intervalMs = showSeconds ? 100 : 1000;
    const timer = setInterval(updateHands, intervalMs);
    return () => clearInterval(timer);
  }, [showSeconds]);

  return (
    <div className="relative w-56 h-56 rounded-full border-2 border-slate-200 dark:border-slate-700 bg-white/40 dark:bg-slate-800/40 shadow-inner flex items-center justify-center">
      {CLOCK_TICKS.map((tick) => (
        <div
          key={tick.index}
          className={`absolute rounded-full ${tick.isMain ? 'bg-blue-500' : 'bg-slate-400 dark:bg-slate-600'}`}
          style={tick.style}
        />
      ))}
      <div
        ref={hourHandRef}
        className="absolute w-1.5 h-14 bg-slate-800 dark:bg-slate-100 rounded-full origin-bottom z-3"
        style={{ bottom: '50%' }}
      />
      <div
        ref={minHandRef}
        className="absolute w-1 h-20 bg-slate-500 dark:bg-slate-400 rounded-full origin-bottom z-2"
        style={{ bottom: '50%' }}
      />
      {showSeconds && (
        <div
          ref={secHandRef}
          className="absolute w-0.5 h-24 bg-red-500 rounded-full origin-bottom z-4"
          style={{ bottom: '50%' }}
        />
      )}
      <div className="absolute w-3.5 h-3.5 bg-slate-800 dark:bg-slate-100 rounded-full z-10 shadow-sm border-2 border-white dark:border-slate-800" />
    </div>
  );
});
