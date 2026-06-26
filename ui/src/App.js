const html = window.htm.bind(window.React.createElement);
const { useState, useEffect, useRef } = window.React;
const React = window.React;
const fMotion = window.Motion || window.framerMotion;
const motion = fMotion ? fMotion.motion : new Proxy({}, {
  get: (target, prop) => (props) => React.createElement(prop, props)
});
const AnimatePresence = fMotion ? fMotion.AnimatePresence : ({children}) => React.createElement(React.Fragment, null, children);

import { DigitalClock, AnalogClock } from './components/Clock.js';
import { Calendar, YearlyView, Icon } from './components/Calendar.js';
import { initHolidays } from './utils/holidays.js';
import { 
  isTauri, 
  setAlwaysOnTop, 
  closeWindow, 
  getAppVersion, 
  restoreWindowPosition, 
  listenToMove 
} from './utils/tauri.js';

const WindowFrame = ({ children, title, isPinned, setIsPinned, isTransparent, setIsTransparent, version }) => {
  return html`
    <div 
      className=${`flex flex-col rounded-2xl overflow-hidden transition-all duration-300 w-[1060px] h-[560px] mx-auto ${isTransparent ? 'bg-white/30 dark:bg-slate-900/30 glass' : 'bg-slate-50 dark:bg-slate-950'}`}
      data-tauri-drag-region
    >
      <div className="flex items-center justify-between px-4 py-2 bg-white/50 dark:bg-slate-900/50 border-b border-slate-200 dark:border-slate-800 select-none" data-tauri-drag-region>
        <div className="flex items-center gap-2" data-tauri-drag-region>
          <div className="w-3 h-3 rounded-full bg-blue-500" />
          <div className="flex items-baseline gap-1.5" data-tauri-drag-region>
            <span className="text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase tracking-widest">${title}</span>
            <span className="text-[10px] font-medium text-slate-400 dark:text-slate-500 tracking-wider">${version ? `v${version}` : ''}</span>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <button 
            onClick=${() => { setIsPinned(!isPinned); setAlwaysOnTop(!isPinned); }} 
            className=${`p-1 rounded hover:bg-slate-200 dark:hover:bg-slate-800 transition-colors ${isPinned ? 'text-blue-500' : 'text-slate-400'}`} 
            title="最前面表示"
          >
            <${Icon} name="pin" size=${14} />
          </button>
          <button 
            onClick=${() => setIsTransparent(!isTransparent)} 
            className=${`p-1 rounded hover:bg-slate-200 dark:hover:bg-slate-800 transition-colors ${isTransparent ? 'text-blue-500' : 'text-slate-400'}`} 
            title="背景透過"
          >
            <${Icon} name="sun" size=${14} />
          </button>
          <div className="w-px h-4 bg-slate-200 dark:bg-slate-800 mx-1" />
          <button 
            onClick=${closeWindow} 
            className="p-1 rounded hover:bg-red-500 hover:text-white text-slate-400 transition-colors"
            title="閉じる"
          >
            <${Icon} name="x" size=${14} />
          </button>
        </div>
      </div>
      <div className="flex-1 p-8">${children}</div>
    </div>
  `;
};

export default function App() {
  const isRestoringRef = useRef(true);
  const [loading, setLoading] = useState(true);
  const [time, setTime] = useState(new Date());
  const [is24Hour, setIs24Hour] = useState(true);
  const [isDarkMode, setIsDarkMode] = useState(true);
  const [clockType, setClockType] = useState('digital');
  const [showSeconds, setShowSeconds] = useState(true);
  const [currentMonth, setCurrentMonth] = useState(new Date());
  const [isPinned, setIsPinned] = useState(true);
  const [showYearly, setShowYearly] = useState(false);
  const [isTransparent, setIsTransparent] = useState(false);
  const [version, setVersion] = useState('1.2.3');

  // 設定の読み込みと外部データの初期化
  useEffect(() => {
    const loadSettings = async () => {
      // 1. 外部祝日設定 JSON のロード
      await initHolidays();
      setLoading(false);

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

      // 4. ウィンドウ位置の復元
      await restoreWindowPosition(isRestoringRef);
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

  // イベント購読、タイマーなどのライフサイクル
  useEffect(() => {
    if (loading) return;

    // 時計のアップデート
    const timer = setInterval(() => setTime(new Date()), 100);
    
    // ウィンドウの移動イベント購読
    let unlistenMovePromise = listenToMove(isRestoringRef);

    // フォールバック用の座標保存ポーリング
    const posTimer = setInterval(async () => {
      if (isRestoringRef.current || !isTauri()) return;
      const win = getTauriWindow();
      if (win) {
        try {
          const pos = await win.outerPosition();
          if (pos) {
            localStorage.setItem('windowPosition', JSON.stringify({ x: pos.x, y: pos.y, type: pos.type || 'Physical' }));
          }
        } catch (e) {
          console.error("Failed to poll position:", e);
        }
      }
    }, 5000);

    // Rust トレイメニューからの最前面切り替えイベントハンドラ
    let unlistenAlwaysOnTopPromise;
    if (isTauri() && window.__TAURI__.event && window.__TAURI__.event.listen) {
      unlistenAlwaysOnTopPromise = window.__TAURI__.event.listen('always-on-top-toggled', (event) => {
        const value = event.payload;
        setIsPinned(value);
      });
    }

    // Rust トレイメニューからの位置リセットイベントハンドラ
    let unlistenPositionResetPromise;
    if (isTauri() && window.__TAURI__.event && window.__TAURI__.event.listen) {
      unlistenPositionResetPromise = window.__TAURI__.event.listen('position-reset', (event) => {
        const [x, y] = event.payload;
        localStorage.setItem('windowPosition', JSON.stringify({ x, y, type: 'Physical' }));
      });
    }
    
    // コンテキストメニュー禁止
    window.oncontextmenu = (e) => { e.preventDefault(); return false; };

    // Escキーで終了
    const handleKeyDown = (e) => {
      if (e.key === 'Escape') {
        closeWindow();
      }
    };
    window.addEventListener('keydown', handleKeyDown);

    return () => {
      clearInterval(timer);
      clearInterval(posTimer);
      unlistenMovePromise.then(unlisten => unlisten && unlisten());
      if (unlistenAlwaysOnTopPromise) unlistenAlwaysOnTopPromise.then(unlisten => unlisten());
      if (unlistenPositionResetPromise) unlistenPositionResetPromise.then(unlisten => unlisten());
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [loading]);

  if (loading) {
    return html`
      <div className="flex h-screen w-screen items-center justify-center bg-slate-900 text-slate-400 font-sans">
        <div className="flex flex-col items-center gap-4">
          <div className="w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
          <div>Loading Clondar Pro...</div>
        </div>
      </div>
    `;
  }

  const getTauriWindow = () => {
    try {
      if (isTauri()) {
        if (window.__TAURI__.webviewWindow && window.__TAURI__.webviewWindow.getCurrentWebviewWindow) {
          return window.__TAURI__.webviewWindow.getCurrentWebviewWindow();
        }
        if (window.__TAURI__.window && window.__TAURI__.window.getCurrentWindow) {
          return window.__TAURI__.window.getCurrentWindow();
        }
        if (window.__TAURI__.getCurrentWindow) {
          return window.__TAURI__.getCurrentWindow();
        }
      }
    } catch (e) {
      console.error("Tauri API access error:", e);
    }
    return null;
  };

  return html`
    <div className="h-full w-full flex items-center justify-center p-4 transition-colors duration-500 overflow-hidden relative" data-tauri-drag-region>
      <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-blue-400/20 blur-[120px] rounded-full pointer-events-none" />
      <div className="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-purple-400/20 blur-[120px] rounded-full pointer-events-none" />
      
      <${WindowFrame} 
        title="Clondar Pro" 
        isPinned=${isPinned} 
        setIsPinned=${setIsPinned} 
        isTransparent=${isTransparent} 
        setIsTransparent=${setIsTransparent} 
        version=${version}
      >
        <div className="flex flex-row gap-12 items-start">
          <div className="flex flex-col items-center gap-8 w-[500px] flex-shrink-0 z-10">
            <div className="flex bg-slate-100 dark:bg-slate-800 p-1 rounded-xl shadow-inner relative z-20">
              <button 
                onClick=${() => setClockType('digital')} 
                className=${`px-4 py-2 rounded-lg text-sm font-bold transition-all flex items-center gap-2 ${clockType === 'digital' ? 'bg-white dark:bg-slate-700 shadow-sm text-blue-600 dark:text-blue-400' : 'text-slate-500 hover:text-slate-700 dark:hover:text-slate-300'}`}
              >
                <${Icon} name="clock" size=${16} />DIGITAL
              </button>
              <button 
                onClick=${() => setClockType('analog')} 
                className=${`px-4 py-2 rounded-lg text-sm font-bold transition-all flex items-center gap-2 ${clockType === 'analog' ? 'bg-white dark:bg-slate-700 shadow-sm text-blue-600 dark:text-blue-400' : 'text-slate-500 hover:text-slate-700 dark:hover:text-slate-300'}`}
              >
                <${Icon} name="clock" size=${16} />ANALOG
              </button>
            </div>

            <div className="flex-1 flex items-center justify-center min-h-[300px]">
              ${clockType === 'digital' ? html`
                <${DigitalClock} time=${time} is24Hour=${is24Hour} showSeconds=${showSeconds} />
              ` : html`
                <${AnalogClock} time=${time} showSeconds=${showSeconds} />
              `}
            </div>

            <div className="flex items-center justify-center gap-3 mt-auto">
              <button 
                onClick=${() => setIs24Hour(!is24Hour)} 
                className=${`px-4 py-1.5 rounded-lg text-xs font-black transition-all shadow-sm ${is24Hour ? 'bg-blue-500 text-white shadow-blue-500/20' : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400'}`}
              >
                ${is24Hour ? '24H' : '12H'}
              </button>
              <button 
                onClick=${() => setShowSeconds(!showSeconds)} 
                className=${`px-4 py-1.5 rounded-lg text-xs font-black transition-all shadow-sm ${showSeconds ? 'bg-blue-500 text-white shadow-blue-500/20' : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400'}`}
              >
                秒表示: ${showSeconds ? 'ON' : 'OFF'}
              </button>
              <button 
                onClick=${() => setIsDarkMode(!isDarkMode)} 
                className="p-2 rounded-lg bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors shadow-sm animate-none"
              >
                ${isDarkMode ? html`<${Icon} name="sun" size=${18} />` : html`<${Icon} name="moon" size=${18} />`}
              </button>
            </div>
          </div>

          <div className="w-px bg-slate-200 dark:bg-slate-800 self-stretch" />

          <div className="flex-1 w-full flex flex-col gap-4">
            <div className="flex items-center gap-2 text-slate-400 mb-2">
              <${Icon} name="calendar" size=${16} />
              <span className="text-xs font-bold uppercase tracking-widest">Calendar</span>
            </div>
            <${Calendar} 
              currentMonth=${currentMonth} 
              setCurrentMonth=${setCurrentMonth} 
              onShowYearly=${() => setShowYearly(true)} 
              isTransparent=${isTransparent} 
            />
          </div>
        </div>
      </${WindowFrame}>

      <${AnimatePresence}>
        ${showYearly && html`
          <${YearlyView} 
            year=${currentMonth.getFullYear()} 
            onClose=${() => setShowYearly(false)} 
          />
        `}
      </${AnimatePresence}>
    </div>
  `;
}
