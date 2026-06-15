# Tauri v2 デスクトップウィジェット開発ガイドライン

このプロジェクト（または後継プロジェクト）でデスクトップアプリを開発する際は、以下の「黄金設定」を厳守すること。

## 1. ウィンドウの影と枠を完全に消す (Windows)
透過ウィンドウで「薄い枠」や「影」が出るのを防ぐため、以下の2箇所を必ずセットで設定する。

- **src-tauri/tauri.conf.json**:
  `"shadow": false`, `"transparent": true`, `"decorations": false` を指定。
- **src-tauri/src/main.rs**:
  `window.set_shadow(false).unwrap();` を `setup` フック内で呼び出す。

## 2. 権限設定 (Capabilities) の重要ルール
Tauri v2 では JavaScript からの命令（移動・終了）はデフォルトで禁止されている。
- **ファイルパス**: `src-tauri/capabilities/default.json` （※ `src/` の中ではない！）
- **必須権限**:
  - `core:window:allow-start-dragging`
  - `core:window:allow-close`
  - `core:window:allow-set-always-on-top`

## 3. ドラッグ操作と透明背景の干渉対策
背景が透明だとマウスイベントが突き抜けてドラッグできなくなる。
- **CSS**: `html, body { pointer-events: auto; background: transparent !important; }`
- **HTML**: ドラッグさせたい要素に `data-tauri-drag-region` を付与。

## 4. ウィンドウ終了ロジック (Tauri v2)
Escキーやカスタムボタンで閉じる際は、以下のコードを使用する。
```javascript
if (window.__TAURI__) {
  const { window: tauriWin } = window.__TAURI__;
  const curr = tauriWin.getCurrentWindow ? tauriWin.getCurrentWindow() : tauriWin.getCurrent();
  curr.close();
}
```

## 5. 開発時の注意
- **キャッシュ**: 設定変更が反映されない時は `src-tauri` フォルダで `cargo clean` を実行すること。
- **パス**: `frontendDist` は常に `index.html` が存在する正しいフォルダを指しているか確認すること。
