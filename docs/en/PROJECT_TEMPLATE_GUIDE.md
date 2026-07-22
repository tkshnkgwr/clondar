# Tauri v2 Desktop App Development: New Project Template Guide

**English** | [日本語版](../ja/PROJECT_TEMPLATE_GUIDE.md)

This document serves as a "Golden Setup" template and "AI developer instructions (AGENTS.md)" guide for establishing new desktop app projects in collaboration with Google AI Studio or AntiGravity (Gem).

For future projects, copy-pasting this layout ensures a clean, secure, and standardized starting point.

---

## 1. Recommended Directory & File Layout

Organize files cleanly to help AI developers navigate and modify code efficiently.

```text
[project-root]/
│
├── .agents/                 # AI (Gem) instructions and guidelines
│   └── AGENTS.md            # ★ AI Rules (using the template below)
│
├── .github/                 # GitHub repository configurations
│   ├── dependabot.yml       # ★ Dependabot config (weekly dependency tracking)
│   └── workflows/
│       └── release.yml      # ★ Automated release workflow
│
├── .vscode/                 # VS Code editor settings
│   └── settings.json        # ★ Shared editor preferences
│
├── docs/                    # Technical documentation
│   ├── ja/                  # Japanese Documentation
│   │   ├── SPECIFICATION.md # ★ Product Specifications (versions, stack details)
│   │   ├── TEST_REPORT.md   # ★ Test Report (results, Mermaid flowcharts)
│   │   └── ...
│   └── en/                  # English Documentation
│       ├── SPECIFICATION.md # ★ Product Specifications (English Version)
│       ├── TEST_REPORT.md   # ★ Test Report (English Version)
│       └── ...
│
├── scripts/                 # Automation scripts
│   └── bump-version.ps1     # ★ Version bumping utility (encoding-safe)
│
├── src-tauri/               # Tauri Backend (Rust)
│   ├── Cargo.toml           # (Note: Commit Cargo.lock to Git for build replication)
│   ├── tauri.conf.json
│   └── capabilities/
│       └── default.json
│
├── ui/                      # Frontend Application (React / HTML)
│   └── index.html           # (Note: Fetch versions dynamically from Tauri APIs)
│
├── .editorconfig            # ★ Shared editor settings (spacing, line endings)
├── .gitignore               # ★ Consolidated Git exclusion filters
├── CHANGELOG.md             # Change history routing (Links to localized changelogs)
├── LICENSE                  # MIT License
├── README.md                # Main documentation (English, links to README_JA.md)
└── README_JA.md             # Japanese documentation
```

---

## 2. Shared `.gitignore` Template for Root Configuration

Consolidates all backend, frontend, OS, and editor exclusions into a single root file. `Cargo.lock` is **not** ignored to ensure build stability.

```gitignore
# Rust (Cargo) Exclusions
/target/
/src-tauri/target/
/src-tauri/gen/schemas/
**/*.rs.bk

# Node.js (Frontend) Exclusions
/node_modules/
/.pnpm-store/
.env
.env.*
!.env.example
dist/
build/

# OS & Editor Exclusions
.DS_Store
Thumbs.db
/.vscode/
/.idea/

# Temp & Backup Files
*.un~
*~
*.swp
```

---

## 3. Template `.agents/AGENTS.md` (Custom AI Rules)

Placing this file under `.agents/AGENTS.md` ensures that AI developers respect Tauri v2 architectures, physical scaling rules, and localized document maintenance.

```markdown
# Tauri v2 Desktop Widget Development Guidelines

Always strictly adhere to these "Golden Settings" when building applications in this project.

## 1. Eliminate Window Borders and Shadows (Windows)
To prevent subtle borders or glow shadows under transparent windows, configure the following settings:

- **src-tauri/tauri.conf.json**:
  Set `"shadow": false`, `"transparent": true`, and `"decorations": false`.
- **src-tauri/src/main.rs**:
  Call `window.set_shadow(false).unwrap();` inside the `setup` hook.

## 2. Permission Capabilities Configuration Rules
In Tauri v2, window manipulation commands (moving, closing) are blocked by default on the frontend.
- **Path**: `src-tauri/capabilities/default.json`
- **Required permissions**:
  - `core:window:allow-start-dragging`
  - `core:window:allow-close`
  - `core:window:allow-set-always-on-top`
  - `core:app:default` (Required to fetch versions dynamically on the frontend)

## 3. Interaction Handling on Transparent Canvasses
Transparent elements pass pointer clicks down to the desktop.
- **CSS**: Set `html, body { pointer-events: auto; background: transparent !important; }`.
- **HTML**: Attach `data-tauri-drag-region` to drag triggers.

## 4. Window Close Hook Patterns (Tauri v2)
When terminating or hiding the window via Escape key or custom buttons, use:
```javascript
if (window.__TAURI__) {
  const { window: tauriWin } = window.__TAURI__;
  const curr = tauriWin.getCurrentWindow ? tauriWin.getCurrentWindow() : tauriWin.getCurrent();
  curr.close();
}
```

## 5. Script & Encoding Protocols (Windows/PowerShell)
- **Use PowerShell 7 (pwsh)**:
  Run deployment scripts using modern `pwsh` instead of Windows PowerShell 5.1 to prevent character encoding issues.
- **UTF-8 without BOM**:
  Save all files using UTF-8 without BOM (Tauri JSON parsers fail if BOM exists in tauri.conf.json).

## 6. Document Update Rules for AI
Update corresponding technical documents before completing any code modifications:

- **`CHANGELOG.md` updates**:
  When altering source files, document changes under `docs/ja/CHANGELOG.md` and `docs/en/CHANGELOG.md` (the root `CHANGELOG.md` acts as a routing gateway).
- **`docs/ja/SPECIFICATION.md` / `docs/en/SPECIFICATION.md` updates**:
  Keep structural specifications synchronized with new features or modified schemas.
- **`README.md` / `README_JA.md` updates**:
  Update setup commands, environment variables, or build dependencies inside the README.
- **`docs/ja/TEST_REPORT.md` / `docs/en/TEST_REPORT.md` updates**:
  Record validation procedures and testing parameters inside the test report.
- **Consistency Auditing**:
  Verify there are no structural mismatches between final source code states and Markdown documents before concluding tasks.

## 7. Editor Standards & CI/CD
- **Respect Editor Configs**:
  Enforce settings in `.editorconfig` and `.vscode/settings.json`.
  Save PowerShell scripts (`*.ps1`) in **UTF-8 with BOM** and **CRLF** line endings. Other files must use **UTF-8 without BOM** and **LF** line endings.
- **Automated Crate Updates**:
  `.github/dependabot.yml` scans Cargo crates and GitHub Action modules weekly for updates.
- **Automated Deployment**:
  Pushing a release tag (`v*`) triggers GitHub Actions to build Windows installation assets and draft GitHub Releases. Run `scripts/bump-version.ps1` before tagging.
```

---

## 4. Version Bump Template Script `scripts/bump-version.ps1`

A cross-compatible PowerShell script that updates version references across files without causing encoding corruptions.

Save this script under `scripts/bump-version.ps1` using **UTF-8 with BOM** to prevent PowerShell 5.1 from misinterpreting characters.

```powershell
param (
    [Parameter(Mandatory=$true)]
    [string]$NewVersion
)

# Validate semantic version formatting (X.Y.Z)
if ($NewVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Error "Error: Version must be in semantic versioning format (e.g. 1.2.3)"
    exit 1
}

$InternalVersion = "$NewVersion.0"

Write-Host "Bumping version to $NewVersion (Internal: $InternalVersion)..."
# UTF-8 without BOM writer object
$utf8 = New-Object System.Text.UTF8Encoding($false)

# Create localized characters dynamically to prevent encoding errors on PowerShell 5.1
$versionLabel  = [char]0x30d0 + [char]0x30fc + [char]0x30b8 + [char]0x30e7 + [char]0x30f3          # "バージョン"
$internalLabel = [char]0x5185 + [char]0x90e8 + $versionLabel                                     # "内部バージョン"
$suitableLabel = [char]0x9069 + [char]0x5408 + $versionLabel                                     # "適合バージョン"

# 1. Update src-tauri/Cargo.toml
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

# 2. Update src-tauri/tauri.conf.json
$tauriPath = "src-tauri/tauri.conf.json"
if (Test-Path $tauriPath) {
    Write-Host "Updating $tauriPath..."
    $content = [System.IO.File]::ReadAllText($tauriPath, $utf8)
    $content = $content -replace '"version"\s*:\s*"[^"]+"', "`"version`": `"$NewVersion`""
    [System.IO.File]::WriteAllText($tauriPath, $content, $utf8)
} else {
    Write-Warning "$tauriPath not found."
}

# 3. Update docs/ja/SPECIFICATION.md
$specPath = "docs/ja/SPECIFICATION.md"
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

# 4. Update docs/ja/TEST_REPORT.md
$testPath = "docs/ja/TEST_REPORT.md"
if (Test-Path $testPath) {
    Write-Host "Updating $testPath..."
    $lines = [System.IO.File]::ReadAllLines($testPath, $utf8)
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match "\*\*$suitableLabel\*\*") {
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

---

## 5. EditorConfig Configuration Template

Coordinates styling rules across various code editors. Save as `.editorconfig` in the root:

```ini
root = true

[*]
indent_style = space
indent_size = 2
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true

[*.rs]
indent_size = 4

[*.ps1]
end_of_line = crlf
charset = utf-8-bom
```

---

## 6. VS Code settings.json Template

Configures linting, default formatting, and paths for `rust-analyzer` integrations. Save under `.vscode/settings.json`:

```json
{
  "files.encoding": "utf8",
  "files.autoGuessEncoding": true,
  "files.insertFinalNewline": true,
  "files.trimTrailingWhitespace": true,
  "editor.tabSize": 2,
  "editor.insertSpaces": true,
  "[rust]": {
    "editor.tabSize": 4,
    "editor.defaultFormatter": "rust-analyzer"
  },
  "[powershell]": {
    "files.encoding": "utf8bom"
  },
  "rust-analyzer.linkedProjects": [
    "src-tauri/Cargo.toml"
  ]
}
```

---

## 7. Dependabot Configuration Template

Enables automated dependency scans. Save as `.github/dependabot.yml` in the root:

```yaml
version: 2
updates:
  # Maintain dependencies for GitHub Actions
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"

  # Maintain dependencies for Cargo (Rust)
  - package-ecosystem: "cargo"
    directory: "/src-tauri"
    schedule:
      interval: "weekly"
```

---

## 8. GitHub Actions release.yml Workflow Template

Drafts GitHub Releases automatically on pushing semantic tags (`v*`). Save under `.github/workflows/release.yml`:

```yaml
name: Release App

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    permissions:
      contents: write
    strategy:
      fail-fast: false
      matrix:
        platform: [windows-latest]

    runs-on: ${{ matrix.platform }}
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: lts/*

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-pc-windows-msvc

      - name: Rust cache
        uses: swatinem/rust-cache@v2
        with:
          workspaces: './src-tauri -> target'

      - name: Install Tauri CLI
        run: npm install -g @tauri-apps/cli@latest

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v2
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'Clondar Pro ${{ github.ref_name }}'
          releaseBody: 'Please refer to CHANGELOG.md for details of this release.'
          releaseDraft: true
          prerelease: false
```
