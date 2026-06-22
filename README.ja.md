# 🕒 Clondar (日本語版)

[English Version](./README.md) | **日本語版**

<!-- UPDATE 2026-06-21: 多言語化サポートとして README.ja.md を分離新設。 -->

**Clondar** は、Windows デスクトップに溶け込む、透過・枠なし・影なしの極限までミニマルなデザインを追求した、モダンなデスクトップウィジェット型時計＆カレンダーアプリです。

Tauri v2 を基盤としており、低リソース消費で動作し、常に最前面（Always on Top）に配置して日々のスケジュールと時刻をスマートに確認できます。

![Clondar Showcase](https://picsum.photos/seed/clondar/1200/600?blur=1)

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
- **💾 ロバストな状態記憶 (v1.2.0)**:
  - **物理座標復元 (DPI対応)**: マルチモニターや高DPIスケーリング（125%、150%など）によるズレを恒久対策。
  - **復旧レースコンディションガード**: 起動時の自動センタリングとJS座標適用のズレを `isRestoringRef` でガード。起動時の誤上書きを100%防止。
  - 時計の設定（秒針、12H/24H）、透過度設定、ピン留め状態（Always on Top）も同様に次回起動へ自動復帰。
- **🖱️ 自由な配置**: ウィンドウのほぼ全域がドラッグ可能。好きな場所に配置できます。
- **📌 常に最前面**: 他のウィンドウに隠れることなく時刻を確認可能。
- **🌓 ダークモード対応**: システム設定や気分に合わせてテーマを切り替え。

## 🛠️ 技術スタック

- **Backend / Core Engine**: Rust ([Tauri v2](https://v2.tauri.app/))
- **Frontend / Framework**: HTML5 (CDN React 18) / Tailwind CSS
- **Animation**: [Framer Motion](https://www.framer.com/motion/) (CSS Transitions)
- **Styling**: Fluent Design 準拠 of モダンな UI

---

## 🚀 セットアップと開発

ローカル環境でのビルドや開発には **Node.js** と **Rust** が必要です。

### 1. リポジトリのクローン
```bash
git clone https://github.com/tkshnkgwr/clondar.git
cd clondar
```

### 2. 開発モードでの起動
```bash
# ※ フロントエンドの依存関係はCDN駆動のため、npm installは不要です。Rust環境のみで即起動できます。
cargo tauri dev
```

### 3. ビルド（インストーラーの作成）
```bash
cargo tauri build
```
ビルド完了後、`src-tauri/target/release/bundle/msi/` または `src-tauri/target/release/bundle/nsis/` 配下にインストーラー（.msi / .exe）が自動生成されます。

---

## 🎨 アプリケーションアイコンの指定方法

ビルドされたアプリのデスクトップアイコンやタスクバーアイコンは、以下の手順で自由に変更可能です。

### 自動生成コマンドを使用する場合 (推奨)
Tauri はソースとなる 1 枚の正方形画像（512x512px以上推奨）から、Windows/Mac/Linux用に最適な全フォーマットのアイコンを一画で自動生成するツールを提供しています。

1. アイコンにしたい画像（`source_icon.png`など）を用意します。
2. プロジェクトのルートディレクトリで、以下のコマンドを実行します。
```bash
npx tauri icon /path/to/source_icon.png
```
※ コマンドを実行すると、`src-tauri/icons/` 配下のすべてのデフォルトアイコンファイル（`32x32.png`、`128x128.png`、`icon.ico`、`icon.icns` など）が自動的に上書き置換され、`tauri.conf.json` のアイコンパス設定とも自動で紐付けられます。

### 手動で差し替える場合
コマンドを使用しない場合、以下のパスに直接用意したアイコンを同じ名前で上書き保存してください。
- **Windows（実行ファイル・タスクバー）用**: `src-tauri/icons/icon.ico`
- **その他システムUI・トレイ用 PNG**: `src-tauri/icons/` ディレクトリ内の各解像度画像（`128x128.png`, `512x512.png` など）

アイコン再生成後は、一度 `cargo clean` してから `cargo tauri dev` または `cargo tauri build` を再実行すると完全に反映されます。

---

## 📝 デスクトップアプリ設計仕様 (Tauri v2 黄金設定)

このアプリは低リソース環境でのウィジェット体験を最大化するために、以下の特殊なコア設計を実装しています。

- **1. 影の削除 (Shadow Removal)**:
  Rust 側 (`main.rs`) の `set_shadow(false)` 呼び出しと、`tauri.conf.json` の設定（`"shadow": false`）を組み合わせ、Windows11等で発生する透過境界の不要な薄い枠や影を完全に排除。
- **2. 権限管理 (Permissions Guard)**:
  `src-tauri/capabilities/default.json` において、必要なアクション（ドラッグ: `allow-start-dragging`, 終了: `allow-close`, 座標復旧: `allow-outer-position`, `allow-set-position`）のための必要最小限の権限のみに絞り、セキュリティと安全性を両立。
- **3. イベント透過制御 (Pointer Events)**:
  CSS で `html, body { pointer-events: auto; }` を適用し、透明部分であってもドラッグ追従がスムーズに成立する挙動を実現。

## ⚠️ 注意事項

- **Windows SmartScreen**: 自作の未署名インストーラーを実行すると、初回のみWindows SmartScreenの警告ポップアップが表示されます。「詳細情報」をクリックしてから「実行」を選択してください。
- **WebView2**: Windows 10/11 には標準搭載されておりそのままで動作します。古いOSや一部の環境で動作しない場合は、Microsoft WebView2 Runtimeのインストールが必要です。

## 📄 ライセンス

[MIT License](LICENSE)

---
Developed by [lunatic.chariot](mailto:lunatic.chariot@gmail.com)
