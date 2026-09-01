import { describe, test, expect, beforeEach, afterEach } from 'vitest';
import { isTauri, getTauriWindow, getAppVersion, setAlwaysOnTop, closeWindow } from './tauri';

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
});
