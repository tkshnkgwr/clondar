# 🕒 Clondar (日本語版)

[English Version](./README.md) | **日本語版**

[![CI Status](https://github.com/tkshnkgwr/clondar/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/clondar/actions)
[![Build Status](https://github.com/tkshnkgwr/clondar/actions/workflows/release.yml/badge.svg)](https://github.com/tkshnkgwr/clondar/actions)
[![Latest Release](https://img.shields.io/github/v/release/tkshnkgwr/clondar)](https://github.com/tkshnkgwr/clondar/releases)
[![Rust Version](https://img.shields.io/badge/rust-1.96.0-orange.svg)](https://www.rust-lang.org/)
[![Node.js Version](https://img.shields.io/badge/node-%3E%3D26.4.0-green.svg)](https://nodejs.org/)
[![Tauri Version](https://img.shields.io/badge/tauri-v2-blue.svg)](https://v2.tauri.app/)
[![License](https://img.shields.io/github/license/tkshnkgwr/clondar)](LICENSE)

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
- **💾 ロバストな状態記憶 (v1.2.2)**:
  - **物理座標復元 (DPI対応・Tauri v2最適化)**: マルチモニターや高DPIスケーリング環境でのズレを恒久対策。Tauri v2の最新APIである `setPosition` へ完全移行し、動作の安定性を向上。
  - **復旧レースコンディションガード**: 起動時の自動センタリングとJS座標適用のズレを `isRestoringRef` でガード。起動時の誤上書きを100%防止。
  - **終了時の即時位置保存**: アプリ終了時に現在位置を即時保存するロジックを統合し、ドラッグ終了後の終了タイミングに関わらず次回起動時の位置を完璧に保証。
  - 時計の設定（秒針、12H/24H）、透過度設定、ピン留め状態（Always on Top）も同様に次回起動へ自動復帰。
- **🖱️ 自由な配置**: ウィンドウのほぼ全域がドラッグ可能。好きな場所に配置できます。
- **📌 常に最前面**: 他のウィンドウに隠れることなく時刻を確認可能。
- **🌓 ダークモード対応**: システム設定や気分に合わせてテーマを切り替え。

## 🛠️ 技術スタック

- **Backend / Core Engine**: Rust ([Tauri v2](https://v2.tauri.app/))
- **Frontend / Framework**: React 18 (Vite によるローカルバンドル / 完全オフライン動作対応) / Tailwind CSS v3
- **Animation**: [Framer Motion](https://www.framer.com/motion/) (CSS Transitions & Animations)
- **Styling**: Fluent Design 準拠のモダンな UI
- **CI/CD / 自動化**: GitHub Actions (リリースビルド自動化), Dependabot (Cargo & ワークフロー自動アップデート)
- **エディタ設定標準**: EditorConfig, VS Code ワークスペース設定 (.vscode)

---

## 🚀 セットアップと開発

ローカル環境でのビルドや開発には **Node.js** と **Rust** が必要です。

### 1. リポジトリのクローンと依存関係のインストール
```bash
git clone https://github.com/tkshnkgwr/clondar.git
cd clondar

# フロントエンド（Vite）の依存パッケージをインストールします
npm --prefix ui install
```

### 2. 開発モードでの起動
```bash
# フロントエンドのViteサーバーとTauriが同時に起動します
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

---

## 🔍 トラブルシューティング (FAQ)

### Q. ウィジェットが画面外（デスクトップ外）に消えてしまい、ドラッグできなくなりました。
マルチモニターの切断や誤った座標の保存により、ウィジェットが画面外に配置されてしまうことがあります。以下の手順で設定を初期化してください。
1. アプリを一度終了します。
2. Windows の場合、ローカルのデータ保存領域である `%LOCALAPPDATA%\com.clondar.pro` （LocalStorageのデータフォルダ）をクリアします。手っ取り早い方法として、アプリが起動している状態で開発者ツールコンソール（利用可能な場合）を開き、`localStorage.clear();` を実行してアプリを再起動（`Esc`キー等で終了して再起動）すると、初期位置（画面中央）にリセットされます。

### Q. 背景が透過しなかったり、ウィンドウの周りに黒い/白い枠や影が表示されたりします。
1. OS のグラフィックス設定やグラフィックスドライバーが最新であることを確認してください。
2. アプリの構成設定が正しく反映されていない可能性があります。`src-tauri` フォルダに移動し、`cargo clean` を実行してから再度 `cargo tauri dev` または `cargo tauri build` を行ってください。
3. Windows の「パフォーマンスオプション」で「ウィンドウの下に影を表示する」等の設定が干渉している場合があります。

---

## 🛠️ 開発とメンテナンス (アドバンスド)

### 📅 日本の祝日（法改正）への対応手順
日本の祝日が追加・変更された場合、以下の手順で祝日定義を更新します。
1. [ui/public/config/holidays.json](./ui/public/config/holidays.json) を開きます。
2. 法改正に合わせて、該当する日付やルール（固定祝日、ハッピーマンデーなど）を JSON 設定ファイルに追加・編集します。
3. 編集後、アプリ内の「**祝日設定**」ボタンをクリックして「祝日設定マネージャー」モーダルを起動し、内蔵のデフォルト設定（`fallbackConfig`）との変更点（追加/削除）が正しく Diff に反映されているか、また統計情報が意図した通りになっているかを視覚的に検証します。
4. 動作に問題がないことを確認し、[TEST_REPORT.md](./TEST_REPORT.md) などを更新します。

### ⚙️ 開発用エディタ設定
本プロジェクトでは、`.editorconfig` と `.vscode/settings.json` を使用してコーディング規約を統一しています。
* 一般的なコードファイルは **BOMなし UTF-8** および **LF** を強制。
* PowerShell スクリプト（`*.ps1`）は、Windows PowerShell 5.1 での文字化けを防ぐため、**BOM付き UTF-8** および **CRLF** を適用。
* インデント: Rust は 4 スペース、その他は 2 スペース。

### 🤖 CI/CD & 自動リリース
* **Dependabot**: 週次で Cargo の依存関係および GitHub Actions ワークフローをスキャンし、アップデートの Pull Request を自動生成します。
* **GitHub Actions による自動デプロイ**: `v*` 形式（例: `v1.2.3`）の Git タグが GitHub にプッシュされると、Windows 版インストーラー（`.msi` / `.exe`）が自動でビルドされ、GitHub Releases にドラフトリリースとしてデプロイされます。

### 📦 リリース・バージョンアップ手順
新しいバージョンをビルド・リリースする際は、以下のチェックリストを順に実行します。

1. **バージョン表記の一括更新**:
   * PowerShell 7 (`pwsh`) を開き、以下のコマンドを実行します：
     ```powershell
     pwsh -File scripts/bump-version.ps1 -NewVersion "1.2.3"
     ```
     これにより、`Cargo.toml`、`tauri.conf.json`、`docs/SPECIFICATION.md`、`docs/TEST_REPORT.md` 内のバージョン情報が自動的に一括置換されます。
2. **アプリアイコンの更新 (変更時のみ)**:
   * 新しい 512x512px 以上の正方形画像を用意し、`npx tauri icon /path/to/icon.png` を実行。
   * アイコン変更をビルドに反映させるため、`cargo clean` を実行。
3. **CHANGELOG の更新**:
   * [CHANGELOG.md](./CHANGELOG.md) に新規バージョンと変更点を追記。
4. **自動ビルド＆デプロイのトリガー**:
   * バージョンに合わせたタグを作成し、GitHub にプッシュします（例: `git tag v1.2.3` -> `git push origin v1.2.3`）。GitHub Actions が自動でビルドおよびリリースドラフトのアップロードを実行します。

---

## ⚠️ 注意事項

- **Windows SmartScreen**: 自作の未署名インストーラーを実行すると、初回のみWindows SmartScreenの警告ポップアップが表示されます。「詳細情報」をクリックしてから「実行」を選択してください。
- **WebView2**: Windows 10/11 には標準搭載されておりそのままで動作します。古いOSや一部の環境で動作しない場合は、Microsoft WebView2 Runtimeのインストールが必要です。

## 📄 ライセンス

[MIT License](./LICENSE)

---
Developed by [tkshnkgwr](https://github.com/tkshnkgwr)
