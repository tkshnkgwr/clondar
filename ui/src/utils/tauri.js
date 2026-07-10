import { getCurrentWindow } from '@tauri-apps/api/window';
import { getVersion as getTauriAppVersion } from '@tauri-apps/api/app';
import { PhysicalPosition } from '@tauri-apps/api/dpi';

export const isTauri = () => !!window.__TAURI__;

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
    console.log("Mock Close: Window closed.");
  }
};

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
