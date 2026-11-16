# Clondar Pro AI開発指示書 (INSTRUCTIONS.md)

[English Version](../en/INSTRUCTIONS.md) | **日本語版**

本ドキュメントは、AIアシスタントが Clondar Pro のソースコード（Pure Rust ネイティブアプリ：`src/`）を保守・拡張する際に出力フォーマット、コーディング規約、エラーハンドリング、およびドキュメント連携について遵守すべきルールを定めます。

---

## 1. コーディング・命名規則

既存のコードベースとスタイルを統一するため、以下の命名規則に従ってください。

- **ファイル名・モジュール名**: `snake_case`（例: `main.rs`, `clock_pane.rs`, `holidays_manager.rs`, `yearly_view.rs`）
- **関数名・変数名**: `snake_case`（例: `calculate_holidays`, `load_or_default`, `render_digital_clock`）
- **構造体・列挙型・トレイト名**: `CamelCase`（例: `ClondarApp`, `AppStateConfig`, `ClockType`, `HolidayConfig`）
- **定数**: `UPPER_SNAKE_CASE`（例: `DEFAULT_HOLIDAYS_JSON`, `APP_MUTEX_NAME`）
- **RustDoc / コメント**: 日本語で明確なロール・背景（「なぜそうするか」）を記述する

---

## 2. エラーハンドリング方針

- `panic!` や無防備な `.unwrap()` の使用は避け、回復可能なエラーは `Result<T, String>` またはフォールバック処理で安全にハンドリングする。
- 設定ファイル（`state.json`, `holidays.json`）が欠損・破損している場合も、内蔵デフォルト設定（`AppStateConfig::default()` / `DEFAULT_HOLIDAYS_JSON`）へ安全にフォールバックして起動を継続させる。

---

## 3. モジュール分割とアーキテクチャ境界

- **`main.rs`**: アプリ全体の状態保持、枠なし透過ウィンドウ設定、トレイイベント処理、カレンダーペインおよびモーダルディスパッチ。
- **`clock_pane.rs`**: 左ペインのデジタル／アナログ時計描画および表示切替ボタン群。
- **`holidays.rs` / `config.rs`**: 日本の祝日計算エンジン、JSON スキーマ検証、および `%LOCALAPPDATA%\com.clondar.pro\` 永続化 I/O。
- **`icons.rs` / `theme.rs` / `font.rs`**: ベクターアイコン描画、ダーク/ライト/透過パレット定義、日本語システムフォント読み込み。
- **`yearly_view.rs` / `holidays_manager.rs` / `help_modal.rs`**: 各種フルスクリーン・オーバーレイモーダル UI。
- **`tray.rs`**: `tray-icon` 制御およびコンテキストメニュー管理。
- **`common_lib`（外部共有クレート）**: テキスト差分計算（LCS）・出現回数集計・Named Mutex 多重起動防止。

---

## 4. ドキュメント同期ルール

ソースコードを変更した際は、変更内容に応じて `docs/ja/` および `docs/en/` の両方の該当ドキュメントを同期して更新してください。
すべてのドキュメントの先頭には日英の相互リンクを配置します。
*(※なお、Markdown ファイル `*.md` のみの軽微な修正時は事前テストコマンドの実行は省略可能です)*

| ドキュメント名               | 役割         | 更新が必要なタイミング                                     |
| :--------------------------- | :----------- | :--------------------------------------------------------- |
| `CHANGELOG.md`               | 変更履歴     | 機能の追加、不具合修正、最適化、不要コード削除を実施時     |
| `TODO.md`                    | タスク管理   | タスクの完了（Done）、新規着手（In Progress）、追加提案時  |
| `SPECIFICATION.md`           | 製品仕様書   | 画面構成、データ構造、ショートカットキー等に変更を加えた時 |
| `ARCHITECTURE.md`            | 構造設計書   | モジュール分割、永続化層、処理フローを変更した時           |
| `DEVELOPMENT.md`             | 開発者ガイド | ビルド手順、依存クレート、開発環境設定を変更した時         |
| `RELEASE.md`                 | リリース手順 | リリースフロー、NSIS 手順、バージョンスクリプト変更時      |
| `USER_GUIDE.md`              | 操作ガイド   | UI 変更、ユーザー設定項目、操作方法を変更・追加した時      |
| `README.md` / `README_JA.md` | 総合概要     | 起動方法、システム要件、主要機能ハイライトに変更がある時   |
| `TEST_REPORT.md`             | テスト報告   | セルフテスト実施、テスト結果記録、システム構成図更新時     |
| `TESTING.md`                 | テスト指針   | テスト実行ポリシー、コマンド、チェック項目を変更した時     |
| `FOOTPRINTS.md`              | リソース計測 | バイナリサイズ、メモリ使用量、ビルド時間を計測・更新した時 |
| `CONTRIBUTING.md`            | 貢献規約     | コミット規約、PR 手順、環境構築ルールを更新した時          |
| `SECURITY.md`                | 安全性方針   | セキュリティ設計、サポートバージョン範囲を更新した時       |

---

## 5. 品質保証と検証プロセス

*(※ドキュメント `*.md` のみの修正時は実行不要)*
- **ファイル規模上限 (1000行目安)**: 単一ファイルが 1000行を超えた場合は適切にモジュール分割を行う。
- **事前検証**: ソースコードの変更を伴うタスクの完了報告前には、必ず以下のコマンドを実行し、エラー・警告がゼロであることを確認する（詳細: [.agents/skills/tauri-verification/SKILL.md](../../.agents/skills/tauri-verification/SKILL.md)）：
  1. `cargo fmt --check`
  2. `cargo clippy --all-targets -- -D warnings`
  3. `cargo test`
  4. `cargo doc --no-deps --document-private-items`
- **ドキュメント上のテーブル整列**: Markdown 内の表組みは、生テキスト（gVim 等）での視認性を保つためパイプ `|` とハイフン `-` の位置を必ず垂直に整列させる。
