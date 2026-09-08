import { describe, test, expect, beforeEach, afterEach } from 'vitest';
import { 
  isTauri, 
  getTauriWindow, 
  getAppVersion, 
  setAlwaysOnTop, 
  closeWindow, 
  restoreWindowPosition, 
  listenToMove 
} from './tauri';

/**
 * @fileoverview Tauri ユーティリティ関数のユニットテストスイート。
 * 非Tauri環境（Webブラウザ/CI環境）でのフォールバック動作および例外安全性を検証します。
 */
describe('Tauri ユーティリティ (tauri.js)', () => {
  beforeEach(() => {
    if (typeof globalThis.window === 'undefined') {
      globalThis.window = {};
    }
    delete globalThis.window.__TAURI__;
  });

  afterEach(() => {
    if (globalThis.window) {
      delete globalThis.window.__TAURI__;
    }
  });

  test('isTauri() は window.__TAURI__ の有無を正しく判定すること', () => {
    expect(isTauri()).toBe(false);

    globalThis.window.__TAURI__ = {};
    expect(isTauri()).toBe(true);
  });

  test('非Tauri環境では getTauriWindow() が null を返すこと', () => {
    expect(getTauriWindow()).toBeNull();
  });

  test('非Tauri環境では getAppVersion() がフォールバック値 "0.0.0-dev" を返すこと', async () => {
    const version = await getAppVersion();
    expect(version).toBe('0.0.0-dev');
  });

  test('非Tauri環境での setAlwaysOnTop() および closeWindow() が例外をスローせず安全に終了すること', async () => {
    await expect(setAlwaysOnTop(true)).resolves.toBeUndefined();
    await expect(closeWindow()).resolves.toBeUndefined();
  });

  test('非Tauri環境での restoreWindowPosition() が安全に完了し isRestoringRef を解除すること', async () => {
    const isRestoringRef = { current: true };
    await expect(restoreWindowPosition(isRestoringRef)).resolves.toBeUndefined();
    expect(isRestoringRef.current).toBe(false);
  });

  test('非Tauri環境での listenToMove() が null を返すこと', async () => {
    const isRestoringRef = { current: false };
    const unlisten = await listenToMove(isRestoringRef);
    expect(unlisten).toBeNull();
  });
});
