# Tauri v2 デスクトップアプリ開発：新規プロジェクト用雛形ガイドライン

Google AI Studio や AntiGravity（Gem）と共同でデスクトップアプリを開発する際、初期設定や文字コード問題で躓かないための「黄金構成」と「AI指示書（AGENTS.md）」のテンプレートです。

次回以降のプロジェクト作成時は、この構成をコピー＆ペーストすることで、安全かつ綺麗な状態でスムーズに開発をスタートできます。

---

## 1. 推奨するフォルダ・ファイル構成 (Directory Structure)

リポジトリルートには余計なファイルを置かず、AIが作業しやすいように分類します。

```text
[project-root]/
│
├── .agents/                 # AI(Gem)向け指示書の格納先
│   └── AGENTS.md            # ★AI用ルールブック（下記テンプレートを使用）
│
├── docs/                    # 各種技術ドキュメント（ルートを汚さないため）
│   ├── SPECIFICATION.md     # ★製品仕様書（バージョン・技術スタック記載）
│   └── TEST_REPORT.md       # ★テストレポート（検証結果・Mermaid構成図記載）
│
├── scripts/                 # 自動化スクリプト
│   └── bump-version.ps1     # ★文字コード対策済・バージョン一括更新スクリプト
│
├── src-tauri/               # Tauri バックエンド（Rust）
│   ├── Cargo.toml           # (※Cargo.lockはGitの管理対象にすること)
│   ├── tauri.conf.json
│   └── capabilities/
│       └── default.json
│
├── ui/                      # フロントエンド（React CDN / HTMLなど）
│   └── index.html           # (※バージョンは直書きせずTauri APIから動的取得する)
│
├── .gitignore               # ★ルート一本化のGit除外設定
├── CHANGELOG.md             # 更新履歴（CHANGELOG）
├── LICENSE                  # ライセンス
├── README.md                # メイン説明（英語推奨、多言語なら上部リンク）
└── README.ja.md             # 日本語説明（README.mdから分離）
```

---

## 2. 新規プロジェクト用 `.gitignore` テンプレート

Tauri / Cargo 関連および OS・エディタ関連の除外設定をルートに一本化したものです。ビルドの再現性を保つため、`Cargo.lock` は除外**しません**。

```gitignore
# Rust (Cargo) 関連の除外
/target/
/src-tauri/target/
/src-tauri/gen/schemas/
**/*.rs.bk

# Node.js (フロントエンド) 関連の除外
/node_modules/
/.pnpm-store/
.env
.env.*
!.env.example
dist/
build/

# OS・エディタ関連の除外
.DS_Store
Thumbs.db
/.vscode/
/.idea/

# エディタの一時ファイル・バックアップファイル
*.un~
*~
*.swp
```

---

## 3. ひな形 `.agents/AGENTS.md` (AI向けカスタムルール)

プロジェクト作成時にこれを `.agents/AGENTS.md` に配置することで、AIがTauri v2の特性や各種ドキュメントの自動更新ルールを最初から理解して動くようになります。

```markdown
# Tauri v2 デスクトップウィジェット開発ガイドライン

このプロジェクトでデスクトップアプリを開発する際は、以下の「黄金設定」を厳守すること。

## 1. ウィンドウの影と枠を完全に消す (Windows)
透過ウィンドウで「薄い枠」や「影」が出るのを防ぐため、以下の2箇所を必ずセットで設定する。

- **src-tauri/tauri.conf.json**:
  `"shadow": false`, `"transparent": true`, `"decorations": false` を指定。
- **src-tauri/src/main.rs**:
  `window.set_shadow(false).unwrap();` を `setup` フック内で呼び出す。

## 2. 権限設定 (Capabilities) の重要ルール
Tauri v2 では JavaScript からの命令（移動・終了）はデフォルトで禁止されている。
- **ファイルパス**: `src-tauri/capabilities/default.json`
- **必須権限**:
  - `core:window:allow-start-dragging`
  - `core:window:allow-close`
  - `core:window:allow-set-always-on-top`
  - `core:app:default` (JavaScript側からアプリの動的バージョンを取得するために必要)

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

## 5. 開発および実行時のルール (Windows/PowerShell)
- **PowerShell 7 (pwsh) の使用**: 
  コマンド実行時は、古い Windows PowerShell (5.1) ではなく、必ず最新の `pwsh` を使用して文字コード問題を回避すること。
- **BOMなし UTF-8 の徹底**:
  ファイルはすべて BOMなし UTF-8 で読み書きすること（TauriのJSONパーサーがBOMを許容しないため）。

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
```

---

## 4. 汎用バージョン更新スクリプト `scripts/bump-version.ps1`

PowerShell 5.1/7 の双方で動作し、絶対に文字化けやBOMエラーを発生させずにバージョンを一括更新するスクリプトです。

新規プロジェクト作成時は、この内容を `scripts/bump-version.ps1` として作成し、**必ず「BOM付き UTF-8」**でエンコード保存してください。（※PowerShell 5.1 が非アスキーの日本語文字列を誤読するのを防ぐためです。）

```powershell
param (
    [Parameter(Mandatory=$true)]
    [string]$NewVersion
)

# セマンティックバージョン (X.Y.Z) のバリデーション
if ($NewVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Error "Error: Version must be in semantic versioning format (e.g. 1.2.3)"
    exit 1
}

$InternalVersion = "$NewVersion.0"

Write-Host "Bumping version to $NewVersion (Internal: $InternalVersion)..."
# BOMなしUTF-8を生成するための .NET エンコーディングオブジェクト
$utf8 = New-Object System.Text.UTF8Encoding($false)

# PowerShell 5.1 のスクリプト文字化けを回避するため、日本語文字列を文字コードから動的生成
$versionLabel  = [char]0x30d0 + [char]0x30fc + [char]0x30b8 + [char]0x30e7 + [char]0x30f3          # "バージョン"
$internalLabel = [char]0x5185 + [char]0x90e8 + $versionLabel                                     # "内部バージョン"
$suitableLabel = [char]0x9069 + [char]0x5408 + $versionLabel                                     # "適合バージョン"

# 1. src-tauri/Cargo.toml の更新 (行配列処理)
$cargoPath = "src-tauri/Cargo.toml"
if (Test-Path $cargoPath) {
    Write-Host "Updating $cargoPath..."
    $lines = [System.IO.File]::ReadAllLines($cargoPath, $utf8)
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match '^version\s*=\s*"[^"]+"') {
            $lines[$i] = "version = `"$NewVersion`""
            break
        }
    }
    [System.IO.File]::WriteAllLines($cargoPath, $lines, $utf8)
} else {
    Write-Warning "$cargoPath not found."
}

# 2. src-tauri/tauri.conf.json の更新
$tauriPath = "src-tauri/tauri.conf.json"
if (Test-Path $tauriPath) {
    Write-Host "Updating $tauriPath..."
    $content = [System.IO.File]::ReadAllText($tauriPath, $utf8)
    $content = $content -replace '"version"\s*:\s*"[^"]+"', "`"version`": `"$NewVersion`""
    [System.IO.File]::WriteAllText($tauriPath, $content, $utf8)
} else {
    Write-Warning "$tauriPath not found."
}

# 3. docs/SPECIFICATION.md の更新 (バージョンおよび内部バージョンの動的置換・挿入)
$specPath = "docs/SPECIFICATION.md"
if (Test-Path $specPath) {
    Write-Host "Updating $specPath..."
    $lines = [System.IO.File]::ReadAllLines($specPath, $utf8)
    $newLines = [System.Collections.Generic.List[string]]::new()
    
    $hasInternal = $false
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match "\*\*$internalLabel\*\*") {
            $hasInternal = $true
            break
        }
    }
    
    for ($i = 0; $i -lt $lines.Length; $i++) {
        $line = $lines[$i]
        if ($line -match "\*\*$versionLabel\*\*") {
            $line = "**$versionLabel**: $NewVersion"
            $newLines.Add($line)
            if (-not $hasInternal) {
                $newLines.Add("**$internalLabel**: $InternalVersion")
            }
        } elseif ($line -match "\*\*$internalLabel\*\*") {
            $line = "**$internalLabel**: $InternalVersion"
            $newLines.Add($line)
        } else {
            $newLines.Add($line)
        }
    }
    [System.IO.File]::WriteAllLines($specPath, $newLines.ToArray(), $utf8)
} else {
    Write-Warning "$specPath not found."
}

# 4. docs/TEST_REPORT.md の更新
$testPath = "docs/TEST_REPORT.md"
if (Test-Path $testPath) {
    Write-Host "Updating $testPath..."
    $lines = [System.IO.File]::ReadAllLines($testPath, $utf8)
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match "\*\*$suitableLabel\*\*") {
            # 適合バージョン行を更新 (文言はプロジェクトに合わせて微調整可能)
            $lines[$i] = "**$suitableLabel**: Widget v$NewVersion (DPI-Aware Physical Coordination Model with Tauri v2 compatibility)"
            break
        }
    }
    [System.IO.File]::WriteAllLines($testPath, $lines, $utf8)
} else {
    Write-Warning "$testPath not found."
}

Write-Host "Version bump completed successfully!"
```
