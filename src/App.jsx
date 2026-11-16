/**
 * @fileoverview Clondar Pro メインアプリケーションオーケストレーター。
 *
 * 全体の状態管理（時計モード、透過状態、最前面固定、ダークモード、カレンダー月管理）、
 * 起動時の物理座標復元および競合ガード、システムトレイとのイベント双方向連携、
 * および遅延ロード（React.lazy）によるモーダル制御を行います。
 */

import React, { useState, useEffect, useRef, useCallback, lazy, Suspense } from 'react';
import { Clock as ClockIcon, Calendar as CalendarIcon, Sun, Moon, Pin, X, HelpCircle } from 'lucide-react';
import { DigitalClock, AnalogClock } from './components/Clock';
import { Calendar } from './components/Calendar';
import { initHolidays } from './utils/holidays';
import {
  isTauri,
  setAlwaysOnTop,
  closeWindow,
  getAppVersion,
  restoreWindowPosition,
  listenToMove
} from './utils/tauri';

// モーダル群の完全遅延ロード（初期バンドルおよび初期 DOM から切り離し）
const YearlyView = lazy(() => import('./components/YearlyView'));
const HolidaysManager = lazy(() => import('./components/HolidaysManager'));
const HelpModal = lazy(() => import('./components/HelpModal'));

/**
 * カスタムウィンドウフレームのプロパティ型定義。
 * @typedef {Object} WindowFrameProps
 * @property {React.ReactNode} children - フレーム内にレンダリングするメインコンテンツ
 * @property {string} title - ウィンドウのタイトルバー文字列
 * @property {boolean} isPinned - 最前面表示（Always-On-Top）が有効かどうか
 * @property {function(boolean): void} setIsPinned - 最前面表示状態を切り替えるステートセッター
 * @property {boolean} isTransparent - 背景透過モードが有効かどうか
 * @property {function(boolean): void} setIsTransparent - 背景透過状態を切り替えるステートセッター
 * @property {string} version - アプリケーションのバージョン文字列
 * @property {function(): void} [onOpenHelp] - ヘルプモーダルを開くコールバック関数
 */

/**
 * カスタムウィンドウフレームコンポーネント。
 *
 * @param {WindowFrameProps} props - コンポーネントのプロパティ
 * @returns {JSX.Element} WindowFrame コンポーネント
 */
const WindowFrame = React.memo(({ children, title, isPinned, setIsPinned, isTransparent, setIsTransparent, version, onOpenHelp }) => {
  return (
    <div
      className={`flex flex-col rounded-2xl overflow-hidden transition-colors duration-300 w-full h-full ${isTransparent ? 'bg-white/40 dark:bg-slate-900/45' : 'bg-slate-50 dark:bg-slate-950'}`}
      data-tauri-drag-region
    >
      <div className="flex items-center justify-between px-4 py-2 bg-white/50 dark:bg-slate-900/50 border-b border-slate-200 dark:border-slate-800 select-none" data-tauri-drag-region>
        <div className="flex items-center gap-2" data-tauri-drag-region>
          <div className="w-3 h-3 rounded-full bg-blue-500" />
          <div className="flex items-baseline gap-1.5" data-tauri-drag-region>
            <span className="text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase tracking-widest">{title}</span>
            <span className="text-[10px] font-medium text-slate-400 dark:text-slate-500 tracking-wider">{version ? `v${version}` : ''}</span>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={onOpenHelp}
            className="p-1 rounded hover:bg-slate-200 dark:hover:bg-slate-800 text-slate-400 hover:text-blue-500 transition-colors"
            title="ヘルプ・アプリ情報"
            aria-label="ヘルプ・アプリ情報"
          >
            <HelpCircle size={14} />
          </button>
          <button
            onClick={() => { setIsPinned(!isPinned); setAlwaysOnTop(!isPinned); }}
            className={`p-1 rounded hover:bg-slate-200 dark:hover:bg-slate-800 transition-colors ${isPinned ? 'text-blue-500' : 'text-slate-400'}`}
            title="最前面表示"
          >
            <Pin size={14} />
          </button>
          <button
            onClick={() => setIsTransparent(!isTransparent)}
            className={`p-1 rounded hover:bg-slate-200 dark:hover:bg-slate-800 transition-colors ${isTransparent ? 'text-blue-500' : 'text-slate-400'}`}
            title="背景透過"
          >
            <Sun size={14} />
          </button>
          <div className="w-px h-4 bg-slate-200 dark:bg-slate-800 mx-1" />
          <button
            onClick={closeWindow}
            className="p-1 rounded hover:bg-red-500 hover:text-white text-slate-400 transition-colors"
            title="閉じる"
          >
            <X size={14} />
          </button>
        </div>
      </div>
      <div className="flex-1 p-8">{children}</div>
    </div>
  );
});

/**
 * メインのアプリケーションルートコンポーネント。
 *
 * ## 主要なアーキテクチャ設計
 * 1. **設定の永続化と復元**: `localStorage` から各種表示設定（12H/24H、秒表示、ダークモード等）を自動読み書き。
 * 2. **DPI対応の物理座標復元**: `restoreWindowPosition` により前回終了時の物理絶対位置へウィンドウを正確に復元した後に1回だけ表示。
 * 3. **レンダリング分離**: 時刻更新を `DigitalClock` / `AnalogClock` 内部に局所化し、ルートやカレンダーの再描画を完全に排除。
 * 4. **イベント駆動座標保存**: デバウンス付き `listenToMove` および終了前保存に一本化し、常時 IPC ポーリングを撤廃。
 * 5. **イベントバス連携**: Rust トレイメニューからの最前面トグルや位置リセットイベントをリアルタイム購読。
 *
 * @returns {JSX.Element} App コンポーネント
 */
export default function App() {
  const isRestoringRef = useRef(true);
  const [loading, setLoading] = useState(true);
  const [is24Hour, setIs24Hour] = useState(true);
  const [isDarkMode, setIsDarkMode] = useState(true);
  const [clockType, setClockType] = useState('digital');
  const [showSeconds, setShowSeconds] = useState(true);
  const [currentMonth, setCurrentMonth] = useState(() => new Date());
  const [isPinned, setIsPinned] = useState(true);
  const [showYearly, setShowYearly] = useState(false);
  const [showHolidaysManager, setShowHolidaysManager] = useState(false);
  const [showHelp, setShowHelp] = useState(false);
  const [holidaysVersion, setHolidaysVersion] = useState(0);
  const [isTransparent, setIsTransparent] = useState(false);
  const [version, setVersion] = useState('');

  // Escキー制御用のモーダル表示状態Ref
  const modalsRef = useRef({ showHelp: false, showYearly: false, showHolidaysManager: false });
  modalsRef.current = { showHelp, showYearly, showHolidaysManager };

  const handleOpenHelp = useCallback(() => setShowHelp(true), []);
  const handleCloseHelp = useCallback(() => setShowHelp(false), []);
  const handleShowYearly = useCallback(() => setShowYearly(true), []);
  const handleCloseYearly = useCallback(() => setShowYearly(false), []);
  const handleShowHolidays = useCallback(() => setShowHolidaysManager(true), []);
  const handleCloseHolidays = useCallback(() => setShowHolidaysManager(false), []);
  const handleSavedHolidays = useCallback(() => setHolidaysVersion((v) => v + 1), []);

  // 設定の読み込みと外部データの初期化
  useEffect(() => {
    const loadSettings = async () => {
      try {
        // 1. 外部祝日設定・当年祝日のロード
        await initHolidays();

        // 2. LocalStorage 設定の読み込み
        const saved24H = localStorage.getItem('is24Hour');
        if (saved24H !== null) setIs24Hour(saved24H === 'true');

        const savedSeconds = localStorage.getItem('showSeconds');
        if (savedSeconds !== null) setShowSeconds(savedSeconds === 'true');

        const savedClockType = localStorage.getItem('clockType');
        if (savedClockType) setClockType(savedClockType);

        const savedDarkMode = localStorage.getItem('isDarkMode');
        if (savedDarkMode !== null) setIsDarkMode(savedDarkMode === 'true');

        const savedTransparent = localStorage.getItem('isTransparent');
        if (savedTransparent !== null) setIsTransparent(savedTransparent === 'true');

        const savedPinned = localStorage.getItem('isPinned');
        if (savedPinned !== null) {
          setIsPinned(savedPinned === 'true');
          setAlwaysOnTop(savedPinned === 'true');
        }

        // 3. アプリケーションバージョンの取得
        const appVersion = await getAppVersion();
        setVersion(appVersion);
      } finally {
        setLoading(false);
        // 4. ウィンドウ位置の復元と1回限りの表示
        await restoreWindowPosition(isRestoringRef);
      }
    };

    loadSettings();
  }, []);

  // 設定の保存
  useEffect(() => {
    if (loading) return;
    localStorage.setItem('is24Hour', is24Hour);
    localStorage.setItem('showSeconds', showSeconds);
    localStorage.setItem('clockType', clockType);
    localStorage.setItem('isDarkMode', isDarkMode);
    localStorage.setItem('isTransparent', isTransparent);
    localStorage.setItem('isPinned', isPinned);
  }, [is24Hour, showSeconds, clockType, isDarkMode, isTransparent, isPinned, loading]);

  // ダークモードのクラス適用
  useEffect(() => {
    if (isDarkMode) document.documentElement.classList.add('dark');
    else document.documentElement.classList.remove('dark');
  }, [isDarkMode]);

  // イベント購読、リスナーなどのライフサイクル
  useEffect(() => {
    if (loading) return;

    // ウィンドウの移動イベント購読（デバウンス付き座標保存）
    const unlistenMovePromise = listenToMove(isRestoringRef);

    // Rust トレイメニューからの最前面切り替えイベントハンドラ
    let unlistenAlwaysOnTopPromise;
    if (isTauri() && window.__TAURI__?.event?.listen) {
      unlistenAlwaysOnTopPromise = window.__TAURI__.event.listen('always-on-top-toggled', (event) => {
        const value = event.payload;
        setIsPinned(value);
      });
    }

    // Rust トレイメニューからの位置リセットイベントハンドラ
    let unlistenPositionResetPromise;
    if (isTauri() && window.__TAURI__?.event?.listen) {
      unlistenPositionResetPromise = window.__TAURI__.event.listen('position-reset', (event) => {
        const [x, y] = event.payload;
        localStorage.setItem('windowPosition', JSON.stringify({ x, y, type: 'Physical' }));
      });
    }

    // コンテキストメニュー禁止
    window.oncontextmenu = (e) => { e.preventDefault(); return false; };

    // Escキーでモーダル閉じる、またはアプリ終了
    const handleKeyDown = (e) => {
      if (e.key === 'Escape') {
        if (modalsRef.current.showHelp) {
          setShowHelp(false);
          return;
        }
        if (modalsRef.current.showYearly) {
          setShowYearly(false);
          return;
        }
        if (modalsRef.current.showHolidaysManager) {
          setShowHolidaysManager(false);
          return;
        }
        closeWindow();
      }
    };
    window.addEventListener('keydown', handleKeyDown);

    return () => {
      unlistenMovePromise.then(unlisten => unlisten && unlisten());
      if (unlistenAlwaysOnTopPromise) unlistenAlwaysOnTopPromise.then(unlisten => unlisten && unlisten());
      if (unlistenPositionResetPromise) unlistenPositionResetPromise.then(unlisten => unlisten && unlisten());
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [loading]);

  if (loading) {
    return (
      <div className="flex h-screen w-screen items-center justify-center bg-slate-900 text-slate-400 font-sans">
        <div className="flex flex-col items-center gap-4">
          <div className="w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
          <div>Loading Clondar Pro...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full w-full overflow-hidden relative select-none" data-tauri-drag-region>
      <WindowFrame
        title="Clondar Pro"
        isPinned={isPinned}
        setIsPinned={setIsPinned}
        isTransparent={isTransparent}
        setIsTransparent={setIsTransparent}
        version={version}
        onOpenHelp={handleOpenHelp}
      >
        <div className="flex flex-row gap-12 items-start">
          <div className="flex flex-col items-center gap-8 w-[500px] flex-shrink-0 z-10">
            <div className="flex bg-slate-100 dark:bg-slate-800 p-1 rounded-xl shadow-inner relative z-20">
              <button
                onClick={() => setClockType('digital')}
                className={`px-4 py-2 rounded-lg text-sm font-bold transition-all flex items-center gap-2 ${clockType === 'digital' ? 'bg-white dark:bg-slate-700 shadow-sm text-blue-600 dark:text-blue-400' : 'text-slate-500 hover:text-slate-700 dark:hover:text-slate-300'}`}
              >
                <ClockIcon size={16} />DIGITAL
              </button>
              <button
                onClick={() => setClockType('analog')}
                className={`px-4 py-2 rounded-lg text-sm font-bold transition-all flex items-center gap-2 ${clockType === 'analog' ? 'bg-white dark:bg-slate-700 shadow-sm text-blue-600 dark:text-blue-400' : 'text-slate-500 hover:text-slate-700 dark:hover:text-slate-300'}`}
              >
                <ClockIcon size={16} />ANALOG
              </button>
            </div>

            <div className="flex-1 flex items-center justify-center min-h-[300px]">
              {clockType === 'digital' ? (
                <DigitalClock is24Hour={is24Hour} showSeconds={showSeconds} />
              ) : (
                <AnalogClock showSeconds={showSeconds} />
              )}
            </div>

            <div className="flex items-center justify-center gap-3 mt-auto">
              <button
                onClick={() => setIs24Hour(!is24Hour)}
                className={`px-4 py-1.5 rounded-lg text-xs font-black transition-all shadow-sm ${is24Hour ? 'bg-blue-500 text-white shadow-blue-500/20' : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400'}`}
              >
                {is24Hour ? '24H' : '12H'}
              </button>
              <button
                onClick={() => setShowSeconds(!showSeconds)}
                className={`px-4 py-1.5 rounded-lg text-xs font-black transition-all shadow-sm ${showSeconds ? 'bg-blue-500 text-white shadow-blue-500/20' : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400'}`}
              >
                秒表示: {showSeconds ? 'ON' : 'OFF'}
              </button>
              <button
                onClick={() => setIsDarkMode(!isDarkMode)}
                className="p-2 rounded-lg bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors shadow-sm animate-none"
              >
                {isDarkMode ? <Sun size={18} /> : <Moon size={18} />}
              </button>
            </div>
          </div>

          <div className="w-px bg-slate-200 dark:bg-slate-800 self-stretch" />

          <div className="flex-1 w-full flex flex-col gap-4">
            <div className="flex items-center gap-2 text-slate-400 mb-2">
              <CalendarIcon size={16} />
              <span className="text-xs font-bold uppercase tracking-widest">Calendar</span>
            </div>
            <Calendar
              key={`calendar-${holidaysVersion}`}
              currentMonth={currentMonth}
              setCurrentMonth={setCurrentMonth}
              onShowYearly={handleShowYearly}
              onShowHolidays={handleShowHolidays}
              isTransparent={isTransparent}
            />
          </div>
        </div>
      </WindowFrame>

      <Suspense fallback={null}>
        {showHelp && (
          <HelpModal
            version={version}
            onClose={handleCloseHelp}
          />
        )}
        {showYearly && (
          <YearlyView
            year={currentMonth.getFullYear()}
            onClose={handleCloseYearly}
          />
        )}
        {showHolidaysManager && (
          <HolidaysManager
            onClose={handleCloseHolidays}
            onSaved={handleSavedHolidays}
          />
        )}
      </Suspense>
    </div>
  );
}
