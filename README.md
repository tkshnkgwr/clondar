# 🕒 Clondar Pro

**Clondar Pro** は、Windows デスクトップに溶け込む、透過・枠なし・影なしの極限までミニマルなデザインを追求した、モダンなデスクトップウィジェット型時計＆カレンダーアプリです。

Tauri v2 を基盤としており、低リソース消費で動作し、常に最前面（Always on Top）に配置して日々のスケジュールと時刻をスマートに確認できます。

![Clondar Pro Showcase](https://picsum.photos/seed/clondar/1200/600?blur=2) <!-- ここに実際のスクリーンショットを配置してください -->

## ✨ 主な機能

- **💎 真のボーダレスデザイン**: Windows 標準のタイトルバー、枠線、影を完全に排除。
- **🌑 透過背景 & グラスモーフィズム**: デスクトップ壁紙を活かしつつ、背景ぼかしで視認性を確保。
- **⏰ ハイブリッドクロック**:
  - **デジタル**: Impact系フォントを使用した力強い表示。12H/24H切り替え、秒表示ON/OFF対応。
  - **アナログ**: スムーズなスイープ運針のミニマルな時計。
- **📅 カレンダー機能**:
  - 日本の祝日（振替休日・国民の休日対応）を完全サポート。
  - 常に6週間を表示する安定したレイアウト。
  - 全画面表示の「年間カレンダー」モーダル搭載（前年・翌年の切り替えに対応）。
- **💾 状態の記憶**: 終了時のウィンドウ位置、時計の設定、透過設定、ピン留め状態を次回起動時に自動復元。
- **🖱️ 自由な配置**: ウィンドウのほぼ全域がドラッグ可能。好きな場所に配置できます。
- **📌 常に最前面**: 他のウィンドウに隠れることなく時刻を確認可能。
- **🌓 ダークモード対応**: システム設定や気分に合わせてテーマを切り替え。

## 🛠️ 技術スタック

- **Backend**: Rust / [Tauri v2](https://v2.tauri.app/)
- **Frontend**: React 18 / Tailwind CSS
- **Animation**: [Framer Motion](https://www.framer.com/motion/)
- **Styling**: Fluent Design 準拠のモダンな UI

## 🚀 セットアップと開発

開発には [Node.js](https://nodejs.org/) と [Rust](https://www.rust-lang.org/) の環境が必要です。

### 1. リポジトリのクローン
```bash
git clone https://github.com/your-username/clondar-pro.git
cd clondar-pro
```

### 2. 依存関係のインストール
※ 現在の構成は CDN 経由の React を使用しているため、標準的な `npm install` は不要ですが、Tauri のビルドには Rust のセットアップが必要です。

### 3. 開発モードでの起動
```bash
cargo tauri dev
```

### 4. ビルド（インストーラーの作成）
```bash
cargo tauri build
```
ビルド完了後、`src-tauri/target/release/bundle/msi/` にインストーラーが生成されます。

## 📝 開発ガイドライン (Tauri v2 仕様)

このアプリはウィジェットとしての体験を最大化するために特殊な設定を行っています。

- **影の削除**: Rust 側 (`main.rs`) の `set_shadow(false)` と `tauri.conf.json` の設定により、透過時の薄い影を抑制しています。
- **権限管理**: `src-tauri/capabilities/default.json` にて、ドラッグ (`allow-start-dragging`) と終了 (`allow-close`) の権限を明示的に付与しています。
- **イベント制御**: 透明な部分でもマウスイベントを拾えるよう、CSS で `pointer-events: auto` を設定しています。

## ⚠️ 注意事項

- **Windows SmartScreen**: 未署名のインストーラーを実行すると警告が表示される場合があります。「詳細情報」をクリックして「実行」を選択してください。
- **WebView2**: Windows 10/11 には標準搭載されていますが、古い環境では自動的にダウンロードが必要になる場合があります。

## 📄 ライセンス

[MIT License](LICENSE)

---
Developed by [lunatic.chariot](mailto:lunatic.chariot@gmail.com)
