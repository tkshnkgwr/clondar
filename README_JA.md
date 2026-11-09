# 🕒 Clondar Pro (日本語版)

[English Version](./README.md) | **日本語版**

[![CI Status](https://github.com/tkshnkgwr/clondar/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/clondar/actions)
[![Build Status](https://github.com/tkshnkgwr/clondar/actions/workflows/release.yml/badge.svg)](https://github.com/tkshnkgwr/clondar/actions)
[![Latest Release](https://img.shields.io/github/v/release/tkshnkgwr/clondar)](https://github.com/tkshnkgwr/clondar/releases)
[![Rust Version](https://img.shields.io/badge/rust-1.96.0-orange.svg)](https://www.rust-lang.org/)
[![Node.js Version](https://img.shields.io/badge/node-%3E%3D26.4.0-green.svg)](https://nodejs.org/)
[![Tauri Version](https://img.shields.io/badge/tauri-v2-blue.svg)](https://v2.tauri.app/)
[![License](https://img.shields.io/github/license/tkshnkgwr/clondar)](LICENSE)

**Clondar Pro** は、Windows デスクトップに溶け込む、透過背景・枠なし・影なしの極限までミニマルなデザインを追求した、モダンなデスクトップウィジェット型時計＆カレンダーアプリです。

Tauri v2 を基盤としており、極めて低いリソース消費（RAM 約 30〜55MB）で軽快に常駐動作し、常に最前面（Always on Top）に配置して日々のスケジュールと現在時刻をスマートに確認できます。

![Clondar Showcase](./docs/assets/screenshot.png)

---

## ✨ 主な機能

- **デスクトップ一体化デザイン (Borderless & Shadowless)**:
  - ウィンドウ枠線やタイトルバーを完全排除し、Windows OS 標準の影も物理的に除去。
  - 背景透過トグル（☀️）でグラスモーフィズム調のすりガラス背景へ瞬時に切り替え可能。
  - `data-tauri-drag-region` により、ウィジェット内のほぼ全域をスムーズにドラッグ移動可能。
- **時計機能 (Clock Section)**:
  - **デジタル時計**: 力強い `Impact` 系フォントと `tabular-nums`（等幅数字）を採用し、秒更新による数字のブレを防止。12H/24H 表示切替および秒表示 ON/OFF に対応。
  - **アナログ時計**: 秒針がミリ秒単位でなめらかに動くスイープ運針アニメーション。
  - **テーマ切替**: ワンクリックでライトモードとダークモードを瞬時に切り替え。
- **カレンダー機能 (Calendar Section)**:
  - **6週間固定グリッド**: 月の長さに依存せず常に 42日間を表示し、月切替時のウィジェットサイズ変動を排除。
  - **日本の祝日完全対応**: 固定祝日、ハッピーマンデー、歴代天皇誕生日、天文計算（春分・秋分）、振替休日、国民の休日、オリンピック特例上書きを正確に網羅。
  - **年間カレンダー**: 全画面展開の年間カレンダーモーダル（前年・翌年のナビゲーション対応）。
- **祝日設定マネージャー (Holidays Manager)**:
  - 固定祝日のビジュアル追加・削除編集機能。
  - 共有ライブラリ `common_lib` の LCS Diff アルゴリズムを用いた、内蔵デフォルトと設定ファイル（`%LOCALAPPDATA%/com.clondar.pro/holidays.json`）の行単位差分ビジュアル表示。
  - キーワード（「の日」「天皇誕生日」等）の出現件数統計。
  - ワンクリックでの初期設定復元（リセット機能）。
- **ヘルプ＆アプリ情報モーダル (❓)**:
  - タイトルバーから起動し、動的バージョンバッジや主要な操作ガイドを一覧表示。
- **システムトレイ（タスクトレイ）常駐**:
  - ウィンドウを閉じてもトレイに安全に常駐（非表示化）。
  - トレイ右クリックから「表示/非表示」「最前面表示の切替」「位置をリセット（画面中央へ）」「終了」を操作可能。
- **高DPI対応の位置記憶と復元ガード**:
  - マルチモニターや高DPI環境でも物理絶対座標（`Physical`）を記憶し、次回起動時に正確に復元。
  - 起動後1秒間の書き込み保護ガード（`isRestoringRef`）により座標の誤上書きを完全防止。
- **オフライン完結設計**:
  - 外部CDNやネットワーク接続に一切依存せず、完全なオフライン状態でも軽快に起動・稼働。

---

## 🖥️ 動作要件

| 項目         | 要件                                                                       |
| :----------- | :------------------------------------------------------------------------- |
| 対応OS       | Windows 10 / 11 (64bit)                                                    |
| ランタイム   | Microsoft Edge WebView2 ランタイム (Windows 10/11 標準搭載)                |
| 開発環境推奨 | Node.js v26.4.0 以上, Rust 1.96.0 以上                                     |

---

## 🚀 クイックスタート & 操作方法

### ショートカット & ボタン操作

| 操作                     | アクション                                                           |
| :----------------------- | :------------------------------------------------------------------- |
| **画面ドラッグ**         | ウィジェット内の背景部分をドラッグして画面上の好きな位置へ移動       |
| **最前面ピン留め (📌)**   | 常に最前面表示（Always-On-Top）の有効化 / 解除                       |
| **背景透過切替 (☀️)**     | 壁紙が透けるグラスモーフィズム透過の ON / OFF 切替                   |
| **ヘルプ (❓)**          | バージョン情報および操作ガイドモーダルを開く                         |
| **閉じる (❌) / Escキー** | ウィジェットを閉じる（常駐トレイへ非表示化。モーダル展開時はモーダルのみ閉じる） |

---

## 🛠️ 開発・ビルド手順

### 1. 依存関係のインストール

```powershell
# フロントエンドの依存関係をインストール
npm install
```

### 2. 開発モードでの起動 (ホットリロード対応)

```powershell
cargo tauri dev
```

### 3. テストの実行

```powershell
# フロントエンド単体テスト (Vitest)
npm test

# バックエンド単体テスト (Rust)
cargo test --manifest-path src-tauri/Cargo.toml
```

### 4. プロダクションビルド (MSI / EXE)

```powershell
cargo tauri build
```

---

## 📚 ドキュメント一覧

詳細な技術設計や運用ガイドラインについては、以下のドキュメントを参照してください。

| ドキュメント                                                  | 言語   | 内容                                                       |
| :------------------------------------------------------------ | :----: | :--------------------------------------------------------- |
| [製品仕様書 (SPECIFICATION.md)](./docs/ja/SPECIFICATION.md)   | 日本語 | 機能仕様、UI/UX定義、データ構造、パッケージング仕様        |
| [アーキテクチャ設計書 (ARCHITECTURE.md)](./docs/ja/ARCHITECTURE.md) | 日本語 | プロセス構造、IPC通信、データフロー、状態永続化の仕組み    |
| [ユーザーガイド (USER_GUIDE.md)](./docs/ja/USER_GUIDE.md)     | 日本語 | インストール手順、基本操作、カレンダー・祝日マネージャー操作 |
| [開発者向けガイド (DEVELOPMENT.md)](./docs/ja/DEVELOPMENT.md) | 日本語 | 開発環境の構築、ディレクトリ構成、コーディング規約         |
| [テスト方針書 (TESTING.md)](./docs/ja/TESTING.md)             | 日本語 | 単体テスト方針、検証手順、コード品質チェック               |
| [テストレポート (TEST_REPORT.md)](./docs/ja/TEST_REPORT.md)   | 日本語 | セルフテスト結果（全34項目PASS）およびMermaidシステム構成図 |
| [開発ToDoリスト (TODO.md)](./docs/ja/TODO.md)                 | 日本語 | 実装済み機能、直近のタスク、今後の機能拡張バックログ       |
| [変更履歴 (CHANGELOG.md)](./docs/ja/CHANGELOG.md)             | 日本語 | バージョンごとのリリースノートおよび変更履歴               |
| [リリース手順書 (RELEASE.md)](./docs/ja/RELEASE.md)           | 日本語 | バージョンバンプ手順、タグ打ち、GitHub Actions自動デプロイ |
| [貢献ガイドライン (CONTRIBUTING.md)](./docs/ja/CONTRIBUTING.md) | 日本語 | コミット規約、PR提出プロセス、開発ルール                   |
| [セキュリティポリシー (SECURITY.md)](./docs/ja/SECURITY.md)   | 日本語 | セキュリティ設計基準、サポートバージョン、脆弱性報告手順   |
| [フットプリント記録 (FOOTPRINTS.md)](./docs/ja/FOOTPRINTS.md) | 日本語 | バイナリサイズ、RAM使用量、ビルドパフォーマンス記録        |

---

## 📄 ライセンス

本プロジェクトは GNU General Public License v3.0 or later (GPL-3.0-or-later) のもとで公開されています - Copyright (c) 2026 tkshnkgwr。詳細は [LICENSE](LICENSE) をご覧ください。
