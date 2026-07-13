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

## 6. ドキュメント自動更新ルール（AI向け）
AIがコードの変更、機能追加、リファクタリングなどを行う際は、必ず以下のドキュメントをセットで更新または作成すること。
**※ただし、Markdownのみの修正であり、ソースコード（Rust, HTML, CSS, JS/TS等）の変更や機能的な修正を伴わない場合は、以下のドキュメント更新や `CHANGELOG.md` への追記は行わないこと。**


- **`CHANGELOG.md` の自動更新**:
  ソースコードに変更を加えた場合は、作業完了前に必ず変更内容や目的を `CHANGELOG.md` の最新セクションに自動追記すること。
- **`docs/SPECIFICATION.md`（仕様書）の更新**:
  新しい機能を追加したり、既存のデータ構造や仕様を変更した場合は、必ず仕様書と整合性をとり、最新の状態に更新すること。
- **`docs/DEVELOPMENT.md`（開発用ドキュメント）の更新**:
  アーキテクチャの変更や新しいモジュール分割、ビルド・起動手順、外部連携などの開発者向けのルールに変更があった場合は、必ず開発用ドキュメントに反映すること。
- **`docs/ARCHITECTURE.md`（アーキテクチャ設計書）の更新**:
  内部設計、マルチプロセスモデル、IPC通信、永続化に関する設計を変更した場合は、必ず設計書に反映すること。
- **`docs/RELEASE.md`（リリース手順書）の更新**:
  リリース方法、バージョンバンプの手順、自動ビルドの構成などに変更があった場合は、必ず反映すること。
- **`docs/USER_GUIDE.md`（ユーザーガイド）の更新**:
  UI仕様、機能の操作方法、カスタマイズ設定（祝日設定マネージャーなど）に変更があった場合は、必ずユーザーガイドに反映すること。
- **`README.md` / `README.ja.md` の更新**:
  環境構築手順や依存ライブラリの追加、起動コマンド、設定手順に変更があった場合は、必ずドキュメントに反映すること。また、README の上部には最新の開発環境（Rust、Node.js、Tauri）やビルド状況に合わせた「ステータスバッジ（Shields.io）」を正しく配置・更新し、整合性を維持すること。
- **`docs/TEST_REPORT.md` の更新**:
  機能実装時やリファクタリング時にテスト（自動テスト・手動テスト）を実行した場合は、検証手順や結果を `docs/TEST_REPORT.md` に記録・更新すること。
- **ドキュメントの整合性チェック**:
  タスク完了時には、コードと各種Markdownドキュメントの間に情報のズレが残っていないか必ずセルフチェックすること。

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
