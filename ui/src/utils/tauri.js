import { getCurrentWindow } from '@tauri-apps/api/window';
import { getVersion as getTauriAppVersion } from '@tauri-apps/api/app';
import { PhysicalPosition } from '@tauri-apps/api/dpi';

/**
 * アプリケーションが Tauri 環境で動作しているかどうかを判定します。
 *
 * @returns {boolean} Tauri 環境である場合は true、それ以外は false
 */
export const isTauri = () => !!window.__TAURI__;

/**
 * 現在の Tauri ウィンドウインスタンスを取得します。
 *
 * @returns {Object|null} WebviewWindow インスタンス、または取得に失敗/非Tauri環境の場合は null
 */
export const getTauriWindow = () => {
  if (isTauri()) {
    try {
      return getCurrentWindow();
    } catch (e) {
      console.error("Tauri API access error:", e);
    }
  }
  return null;
};

/**
 * ウィンドウの最前面表示（Always-On-Top）の状態を設定します。
 *
 * @param {boolean} on - 最前面表示を有効にする場合は true、解除する場合は false
 */
export const setAlwaysOnTop = async (on) => {
  const win = getTauriWindow();
  if (win) {
    try {
      await win.setAlwaysOnTop(on);
    } catch (e) {
      console.error("Failed to set always on top:", e);
    }
  }
};

/**
 * アプリケーションのバージョン情報を取得します。
 *
 * @returns {Promise<string>} バージョン文字列 (例: "1.3.7")
 */
export const getAppVersion = async () => {
  if (isTauri()) {
    try {
      return await getTauriAppVersion();
    } catch (e) {
      console.error("Failed to get version:", e);
    }
  }
  return '0.0.0-dev'; // フォールバック
};

/**
 * 現在のウィンドウを閉じます。
 * ウィンドウが閉じられる前に、現在のウィンドウ位置を LocalStorage に保存します。
 */
export const closeWindow = async () => {
  const win = getTauriWindow();
  if (win) {
    try {
      const pos = await win.outerPosition();
      if (pos) {
        localStorage.setItem('windowPosition', JSON.stringify({ x: pos.x, y: pos.y, type: pos.type || 'Physical' }));
      }
    } catch (e) {
      console.error("Failed to save position on close:", e);
    }
    try {
      await win.close();
    } catch (e) {
      console.error("Failed to close window:", e);
    }
  } else {
    console.log("モック終了: ウィンドウが閉じられました。");
  }
};

/**
 * LocalStorage から保存されたウィンドウ位置を読み込み、ウィンドウ位置を復元します。
 *
 * @param {React.MutableRefObject<boolean>} isRestoringRef - 復元処理中フラグの Ref
 */
export const restoreWindowPosition = async (isRestoringRef) => {
  const win = getTauriWindow();
  if (!win) {
    if (isRestoringRef) isRestoringRef.current = false;
    return;
  }
  try {
    const savedPos = localStorage.getItem('windowPosition');
    if (savedPos) {
      const pos = JSON.parse(savedPos);
      if (pos && typeof pos.x === 'number' && typeof pos.y === 'number') {
        await new Promise(resolve => setTimeout(resolve, 300));
        
        const positionObj = new PhysicalPosition(pos.x, pos.y);
        await win.setPosition(positionObj);
      }
    }
  } catch (err) {
    console.error("Restore position failed:", err);
  } finally {
    setTimeout(() => {
      if (isRestoringRef) isRestoringRef.current = false;
    }, 1000);
  }
};

/**
 * ウィンドウの移動イベント (`tauri://move`) を購読し、
 * 移動が完了した時点でのウィンドウ位置を LocalStorage に保存します。
 *
 * @param {React.MutableRefObject<boolean>} isRestoringRef - 復元処理中フラグの Ref
 * @param {Function} [callback] - 移動完了時に呼び出されるコールバック関数（位置情報を引数に取る）
 * @returns {Promise<Function|null>} イベントの購読解除用関数（Unlisten）、非Tauri環境時は null
 */
export const listenToMove = async (isRestoringRef, callback) => {
  const win = getTauriWindow();
  if (win && win.listen) {
    try {
      return await win.listen('tauri://move', async () => {
        if (isRestoringRef.current) return;
        const pos = await win.outerPosition();
        if (pos) {
          localStorage.setItem('windowPosition', JSON.stringify({ x: pos.x, y: pos.y, type: pos.type || 'Physical' }));
          if (callback) callback(pos);
        }
      });
    } catch (e) {
      console.error("Failed to listen to move event:", e);
    }
  }
  return null;
};
