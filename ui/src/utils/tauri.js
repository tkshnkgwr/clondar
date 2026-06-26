export const isTauri = () => !!window.__TAURI__;

export const getTauriWindow = () => {
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
  if (isTauri() && window.__TAURI__.app && window.__TAURI__.app.getVersion) {
    try {
      return await window.__TAURI__.app.getVersion();
    } catch (e) {
      console.error("Failed to get version:", e);
    }
  }
  return '1.2.3'; // fallback
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
        // 起動直後の自動センター配置からスムーズに引き継ぐための適切な遅延時間（300ms）を確保
        await new Promise(resolve => setTimeout(resolve, 300));
        
        let positionObj = { x: pos.x, y: pos.y, type: pos.type || 'Physical' };
        if (window.__TAURI__.dpi && window.__TAURI__.dpi.PhysicalPosition) {
          positionObj = new window.__TAURI__.dpi.PhysicalPosition(pos.x, pos.y);
        }

        if (win.setPosition) {
          await win.setPosition(positionObj);
        } else if (win.setOuterPosition) {
          await win.setOuterPosition(positionObj);
        }
      }
    }
  } catch (err) {
    console.error("Restore position failed:", err);
  } finally {
    // 復元アニメーションやOSの座標決定が完全に落ち着いた1秒後に、ユーザーの手動移動の検知を開始させます。
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
