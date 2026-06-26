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

- **`CHANGELOG.md` の自動更新**:
  ソースコードに変更を加えた場合は、作業完了前に必ず変更内容や目的を `CHANGELOG.md` の最新セクションに自動追記すること。
- **`docs/SPECIFICATION.md`（仕様書）の更新**:
  新しい機能を追加したり、既存のデータ構造や仕様を変更した場合は、必ず仕様書と整合性をとり、最新の状態に更新すること。
- **`README.md` / `README.ja.md` の更新**:
  環境構築手順や依存ライブラリの追加、起動コマンド、設定手順に変更があった場合は、必ずドキュメントに反映すること。
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
- **自動デプロイフロー**:
  `v*` 形式の Git タグがプッシュされると、GitHub Actions が自動的にトリガーされ、Windows 用の Tauri アプリがビルドされ GitHub Releases にドラフトリリースとしてデプロイされる。タグ作成前に `scripts/bump-version.ps1` を用いてバージョンを一括更新すること。

