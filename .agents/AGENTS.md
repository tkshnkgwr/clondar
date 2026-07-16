# Tauri v2 デスクトップウィジェット開発ガイドライン (AGENTS.md)

このプロジェクト（または後継プロジェクト）でデスクトップアプリを開発する際は、以下の「黄金設定」およびドキュメント更新・AI用規約を厳守すること。

---

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

---

## 6. AI 開発用ドキュメント管理ルール

AIがコードの変更、機能追加、リファクタリングなどを行う際は、必ず以下のドキュメントをセットで更新・確認すること。
**※ただし、Markdownのみの修正であり、ソースコード（Rust, HTML, CSS, JS/TS等）の変更や機能的な修正を伴わない場合は、以下のドキュメント更新や `CHANGELOG.md` への追記は行わないこと。**

### 6.1 事前確認ドキュメント
- **`.agents/Instructions.md` (コーディング規約)**:
  命名規則、エラーハンドリング方針、コンポーネント分割基準、AI用出力フォーマットがまとめられています。**コードの変更を行う前に必ずこれを確認・厳守してください**。
- **`docs/Todo.md` (タスク管理)**:
  実装済み機能や直近の課題、バックログがまとめられています。**作業開始前に「現在地」を把握するために確認してください**。

### 6.2 コード修正時に同期更新すべきドキュメント
ソースコードに変更を加えた場合は、作業完了前に必ず以下の該当ドキュメントを整合性がとれるよう更新すること。

- **`CHANGELOG.md`**:
  変更内容や目的を最新セクションに自動追記する。
- **`docs/Todo.md`**:
  着手したタスクや完了したタスクのステータス（`[ ]` ➔ `[x]`）を最新の状態に更新する。新規課題を発見した場合は Todo/Backlog に追記する。
- **`docs/SPECIFICATION.md` (仕様書)**:
  新しい機能の追加や既存のデータ構造・画面仕様の変更があった場合に反映する。
- **`docs/ARCHITECTURE.md` (設計書)**:
  プロセスモデル、IPC通信、永続化設計、またはデータフローに変更があった場合に反映する。
- **`docs/DEVELOPMENT.md` (開発ガイド)**:
  ディレクトリ構成、依存関係、起動手順、外部連携等のルールに変更があった場合に反映する。
- **`docs/RELEASE.md` (リリース手順)**:
  ビルド・パッケージング手順、バージョンバンプフローに変更があった場合に反映する。
- **`docs/USER_GUIDE.md` (ユーザーガイド)**:
  UI仕様や操作方法に変更があった場合に反映する。
- **`README.md` / `README.ja.md`**:
  環境構築や依存関係、ステータスバッジ（Shields.io）に影響する変更時に反映する。
- **`docs/TEST_REPORT.md` (テスト結果報告)**:
  テスト実行時（自動/手動）に、検証手順や結果を記録・更新する。

---

## 7. 開発環境の統一とCI/CD
- **エディタ設定の厳守**:
  プロジェクトルートにある `.editorconfig` と `.vscode/settings.json` の設定を厳守すること。
  特に PowerShell スクリプト（`*.ps1`）は文字化けを防ぐため、必ず「BOM付き UTF-8」かつ「CRLF」で保存すること。その他のファイルは「BOMなし UTF-8」かつ「LF」で保存すること。
- **依存ライブラリの自動アップデート**:
  `.github/dependabot.yml` が設定されており、Cargo 依存関係および GitHub Actions のアップデートが自動で監視・提案される。
- **自動CI（継続的インテグレーション）**:
  リポジトリへのプッシュやプルリクエスト時に、GitHub Actions（`ci.yml`）によって自動的に `cargo fmt`、`cargo clippy`、`cargo test` が走るようになっている。ローカルでのコミット前にこれらが通ることを事前に確認すること。
  なお、無駄なビルドを防ぐため、Markdownファイル（`**/*.md`）のみの変更時はCIが起動しないよう設定（`paths-ignore`）されている。
- **手動リリース・自動デプロイフロー**:
  普段の Push では自動的にバージョンは上がりません。リリースする際は、ローカルで `scripts/bump-version.ps1` を実行してバージョンを更新・コミットし、バージョンタグ（例: `v1.3.5`）を作成してプッシュします。GitHub 側でタグのプッシュを検知すると、自動デプロイフロー（Release App）がトリガーされて、Windows 用の Tauri アプリがビルドされ GitHub Releases にドラフトリリースとしてデプロイされます。

## 8. 共有ライブラリの依存解決ルール
- **Cargo.toml の依存定義**:
  GitHub Actions や Dependabot 解析時にローカル相対パス参照エラー（`path_dependencies_not_reachable`）が発生するのを防ぐため、`common_lib` などの外部依存ライブラリは、必ず Git リポジトリ（`git = "..."`）として記述すること。
- **ローカルパスオーバーライド**:
  ローカル開発時に変更を迅速に反映させるため、`src-tauri/.cargo/config.toml` に `paths = ["../../common_lib"]` を指定してローカルパスを優先させる。この `config.toml` は開発環境依存となるため、必ず `.gitignore` に追加してリポジトリの管理対象外にすること。

## 9. ドキュメントコメント（RustDoc/JSDoc）の作成と同期
コードの追加や仕様変更時には、必ず対応するドキュメントコメント（RustDoc/JSDoc）を作成または更新すること。
- **Rust**: クレート、モジュール、構造体、列挙型、関数、メソッドなどに対して、`///` または `//!` を用いて RustDoc コメントを記述する。
- **JavaScript/TypeScript**: 各コンポーネント、カスタムフック、ユーティリティ関数などに対して、`/** ... */` を用いて JSDoc/TSDoc コメントを記述する。引数には `@param`、戻り値には `@returns` を適切に設定する。
- **言語設定**: コード内のコメントおよびドキュメントコメントは、英語表記を極力避け、日本語で統一すること。既存のコードで英語のコメントが存在する場合は、本プロジェクト開発時および仕様変更のタイミングで日本語へローカライズ（日本語化）すること。
