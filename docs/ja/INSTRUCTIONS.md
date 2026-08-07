# Clondar Pro AI 開発指示書 (INSTRUCTIONS.md)

本ドキュメントは、AIアシスタントが Clondar Pro のソースコード（Rust バックエンド / React フロントエンド）を修正・追加する際に厳守すべきコーディングスタイル、エラーハンドリング方針、コンポーネント設計、および出力フォーマットを定義したガイドラインです。

---

## 1. 命名規則
既存コードのスタイルを一貫して維持するため、以下の規則を厳守してください。

### 1.1 バックエンド (Rust)
- **ファイル名 / モジュール名**: `snake_case` (例: `main.rs`, `lib.rs`)
- **関数名 / 変数名**: `snake_case` (例: `load_holidays_json`, `get_holidays_file_path`, `app_handle`, `win_clone`)
- **構造体 / 列挙型 / 型名**: `CamelCase` (例: `QuittingState`, `DiffPart`)
- **定数**: `UPPER_SNAKE_CASE` (例: `DEFAULT_HOLIDAYS_JSON`)
- **Tauri コマンド名**: Rust側で定義するコマンド関数名は `snake_case` です。呼び出し文字列のミスマッチに注意してください。

### 1.2 フロントエンド (Vite / React / JavaScript)
- **ファイル名**:
  - UIコンポーネントファイル: `PascalCase.jsx` (例: `Calendar.jsx`, `Clock.jsx`, `HolidaysManager.jsx`)
  - ユーティリティ / その他ファイル: `camelCase.js` (例: `holidays.js`, `tauri.js`)
- **コンポーネント関数名**: `PascalCase` (例: `WindowFrame`, `App`, `DigitalClock`)
- **一般関数名 / 変数名**: `camelCase` (例: `initHolidays`, `reloadHolidays`)

---

## 2. エラーハンドリング方針
- **バックエンド (Rust)**: `panic!` や `unwrap()` の使用は極力避け、回復可能なエラーは `Result<T, String>` などの戻り値で処理してください。
- **フロントエンド (React)**: Tauri API や Rust コマンドの呼び出しなどは、必ず `try-catch` ブロックで保護してください。失敗時はデフォルト設定（`fallbackConfig`）を適用します。

---

## 3. コンポーネント・モジュール分割基準
- **`main.rs` (メインプロセスロジック)**: アプリケーションのセットアップ、ウィンドウ装飾（枠・影・透過）の制御、常駐トレイメニューの構築、およびIPCコマンドの登録を行います。
- **`common_lib` (共有クレート)**: カレンダー処理、Diff計算、二重起動チェックなど、共有可能な純粋なロジックをここに集約します。
- **フロントエンドの構造**: `ui/src/components/` (ビュー層)、`ui/src/utils/` (ロジック・API層)、`App.jsx` (オーケストレーター) に明確に分割します。

---

## 4. 自動ドキュメンテーションと更新ルール (モジュール化退避)
コード修正時に同期更新すべきドキュメント (日本語版 `docs/ja/` / 英語版 `docs/en/` の双方を更新):
最上部には相互言語リンク（例: `[English Version](../en/SPECIFICATION.md) | **日本語版**`）を配置する。
※**`.md` ファイルのみの変更時は、自動ドキュメント更新および事前検証プロセスはすべてスキップ可**。

| 対象ドキュメント | 役割 | 更新タイミング |
| :--- | :--- | :--- |
| `CHANGELOG.md` | 変更履歴 | 機能追加・修正・パフォーマンス改善・削除完了時 |
| `TODO.md` | タスクステータス | タスク完了 (Done)、進行中 (In Progress/Todo)、拡張提案 (Backlog) の変更時に更新。 |
| `SPECIFICATION.md` | 機能仕様・UI | 画面レイアウト・データ構造・ショートカット変更時 |
| `ARCHITECTURE.md` | アーキテクチャ | システム境界・IPC通信・プロセス構造の変更時 |
| `DEVELOPMENT.md` | 開発ガイド | ビルド手順・依存関係・開発コマンド変更時 |
| `RELEASE.md` | リリース手順書 | バージョン更新、ビルド手順、タグ打ち手順の変更時に更新。 |
| `USER_GUIDE.md` | 操作・UIガイド | UI仕様・操作方法の変更時。 |
| `README.md` / `README_JA.md` | 概要・実行手順 | 起動オプション、ビルド手順、動作要件等の変更時。 |
| `TEST_REPORT.md` | テスト結果記録 | テスト実行時の検証手順・結果記録。 |
| `TESTING.md` | テスト方針・ガイド | テスト方針・検証環境基準・ローカル検証プロセスの変更時。 |
| `FOOTPRINTS.md` | パフォーマンス計測 | バイナリサイズ・RAM使用量等の変更時。 |
| `CONTRIBUTING.md` | 開発規約 | コミット規約・PR送信プロセス・セットアップ要件の変更時。 |
| `SECURITY.md` | セキュリティ方針 | セキュリティ設計基準・脆弱性報告プロセスの変更時。 |

---

## 5. 品質管理・検証プロセス (モジュール化退避)
- **1000行超えの検知と推奨**: 単一のソースファイルの行数が **1000行を超えた場合**、AIはモジュール分割やコンポーネント抽出によるリファクタリングを積極的に検討・提案する。
- **ローカル事前検証**: タスク完了前に以下をローカル実行し、すべて合格（エラー・警告ゼロ）を確認する：
  1. `cargo test` （テスト合格）
  2. `cargo clippy --all-targets -- -D warnings` （Clippy警告・エラーゼロ）
  3. `cargo fmt --check` （コードフォーマット完全準拠）
  4. `cargo doc --no-deps --document-private-items` （Rustdoc警告・エラーゼロ）
- **ウィンドウの影と枠を消す (Windows)**: `tauri.conf.json` での `"shadow": false` 等と、`main.rs` での `window.set_shadow(false).unwrap();` の呼び出しを維持する。
