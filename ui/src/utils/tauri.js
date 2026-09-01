/**
 * @fileoverview Tauri v2 API ラッパーユーティリティ。
 *
 * ウィンドウの物理座標の取得・復元、最前面ピン留め設定、安全な終了処理、
 * および移動イベント購読などのネイティブ OS 連携処理を安全に抽象化します。
 * 非 Tauri 環境（Web ブラウザでの開発・テスト時）でのフォールバック動作も担保します。
 */

import { getCurrentWindow } from '@tauri-apps/api/window';
import { getVersion as getTauriAppVersion } from '@tauri-apps/api/app';
import { PhysicalPosition } from '@tauri-apps/api/dpi';

/**
 * 物理座標オブジェクトの型定義。
 * @typedef {Object} WindowPosition
 * @property {number} x - X座標（物理ピクセル）
 * @property {number} y - Y座標（物理ピクセル）
 * @property {'Physical'|'Logical'} [type] - 座標の種類
 */

/**
 * アプリケーションが Tauri ネイティブ実行環境で動作しているかを判定します。
 *
 * @returns {boolean} Tauri 環境（`window.__TAURI__` が存在）である場合は true、Webブラウザ環境時は false
 */
export const isTauri = () => typeof window !== 'undefined' && !!window.__TAURI__;

/**
 * 現在の Tauri ウィンドウインスタンスを取得します。
 *
 * @returns {import('@tauri-apps/api/window').Window|null} 現在の Window インスタンス。非Tauri環境または取得失敗時は null
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
 * @async
 * @param {boolean} on - 最前面表示を有効にする場合は true、解除する場合は false
 * @returns {Promise<void>}
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
 * アプリケーションのバージョン文字列を取得します。
 *
 * ## 動作仕様
 * 1. Tauri 環境では `@tauri-apps/api/app` の `getVersion` を呼び出し、`tauri.conf.json` で定義された動的バージョンを取得します。
 * 2. ブラウザ環境やテスト環境ではフォールバック値 `'0.0.0-dev'` を返します。
 *
 * @async
 * @returns {Promise<string>} セマンティックバージョン文字列 (例: "1.3.8")
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
 * 現在のウィンドウを安全に閉じます。
 *
 * ## 動作仕様
 * 1. ウィンドウを閉じる直前に、現在の物理絶対座標（`outerPosition`）を取得して `localStorage` に即時保存します。
 * 2. `win.close()` を呼び出します（Rust 側の常駐フックにより、明示的終了フラグが立っていない場合は非表示になります）。
 *
 * @async
 * @returns {Promise<void>}
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
 * LocalStorage から保存されたウィンドウ物理座標を読み込み、ウィンドウ位置を復元します。
 *
 * ## 画面チラつき防止と競合ガード
 * 1. 座標復元処理中、引数で渡された `isRestoringRef.current` を `true` に維持し、移動検知リスナーによる誤上書きをブロックします。
 * 2. 保存された座標が存在する場合は `PhysicalPosition` オブジェクトを用いて指定位置へ移動させ、存在しない場合は画面中央へ配置（`center()`）します。
 * 3. 位置移動が完了した後にウィンドウを表示（`win.show()`）することで、起動時の画面ジャンプやチラつきを排除します。
 * 4. 起動後 1 秒経過した時点で `isRestoringRef.current` を `false` にリセットし、通常の位置記録を再開します。
 *
 * @async
 * @param {React.MutableRefObject<boolean>} isRestoringRef - 位置復元ガード用 Ref オブジェクト
 * @returns {Promise<void>}
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
        const positionObj = new PhysicalPosition(pos.x, pos.y);
        await win.setPosition(positionObj);
      } else {
        await win.center();
      }
    } else {
      await win.center();
    }
  } catch (err) {
    console.error("Restore position failed:", err);
    try {
      await win.center();
    } catch (_) {}
  } finally {
    try {
      await win.show();
    } catch (e) {
      console.error("Failed to show window:", e);
    }
    setTimeout(() => {
      if (isRestoringRef) isRestoringRef.current = false;
    }, 1000);
  }
};

/**
 * ウィンドウの移動イベント (`tauri://move`) を購読し、
 * 移動が完了した時点での最新物理絶対座標を LocalStorage に保存します。
 *
 * @async
 * @param {React.MutableRefObject<boolean>} isRestoringRef - 復元処理中フラグの Ref（true の間は保存をスキップ）
 * @param {function(WindowPosition): void} [callback] - 移動完了時に最新の座標情報を受け取るコールバック関数
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

