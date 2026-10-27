# Clondar Pro Release Procedure (GitHub Actions Automation)

**English** | [日本語版](../ja/RELEASE.md)

This document is a guideline explaining how to update versions, build release packages, and manage the automated release pipeline via GitHub Actions for Clondar Pro.

---

## 1. Release Pipeline Overview

Releases of Clondar Pro are managed through a **GitHub Actions automated compilation workflow** triggered by pushing a local version tag (`v*`).

```mermaid
sequenceDiagram
    actor Developer
    participant LocalRepo as Local Repository
    participant GitHub as GitHub Remote
    participant Actions as GitHub Actions (release.yml)
    participant Releases as GitHub Releases (Published)

    Developer->>LocalRepo: scripts/bump-version.ps1 [New Version]
    Note over LocalRepo: Batch version updates in Cargo.toml,<br/>tauri.conf.json, and Markdowns
    Developer->>LocalRepo: git commit & git tag v[New Version]
    Developer->>GitHub: git push origin main --tags
    GitHub->>Actions: Trigger action execution (tags: v*)
    Note over Actions: Start Windows build runner<br/>Package app using Tauri CLI
    Actions->>Releases: Upload MSI/EXE binaries
    Note over Releases: Release is published automatically
```

---

## 2. Preparing Releases Locally

Before packaging a release, all project-wide version designations (`Cargo.toml`, `tauri.conf.json`, technical Markdown files) must be incremented. A PowerShell script is provided to automate this batch update.

### 2.1 Running the Batch Version Update Script

Launch PowerShell and execute the version bump script in the root directory:

```powershell
# Temporarily bypass script execution policy (e.g., updating version to 1.4.1)
powershell -ExecutionPolicy Bypass -File .\scripts\bump-version.ps1 1.4.1
```

- **Script Behavior**:
  - Updates `version = "..."` inside `src-tauri/Cargo.toml`.
  - Updates `version` mappings inside `src-tauri/tauri.conf.json` and `package.json`.
  - Updates version and internal version fields in `docs/ja/SPECIFICATION.md` and `docs/en/SPECIFICATION.md`.
  - Updates the matching version field in `docs/ja/TEST_REPORT.md` and `docs/en/TEST_REPORT.md`.

---

## 3. Creating & Pushing Version Tags

Once files are updated, stage and commit the changes, tag the commit with the target version number, and push them to the GitHub repository:

```bash
# 1. Stage and commit update changes
git add .
git commit -m "chore: release version 1.4.1"

# 2. Tag target commit (following semantic version naming conventions, prefixed with 'v')
git tag v1.4.1

# 3. Push commit and tagging attributes to the remote origin
git push origin main
git push origin v1.4.1
```

---

## 4. Automated Build & Release Publication via GitHub Actions

Once GitHub detects a tagged push, `.github/workflows/release.yml` triggers automatically.

### 4.1 Actions Workflow Processes
1. **Environment Setup**: Provisions Node.js and Rust (MSVC targets) runners and checks out `clondar` alongside the shared crate `common_lib`.
2. **Frontend Dependency Setup**: Runs `npm ci` inside the project root.
3. **Tauri App Compilation**: Runs `tauri build` to package native Windows executable binaries.
4. **Release Publication**: Uses `softprops/action-gh-release@v3` to publish a GitHub Release (`draft: false`) and attach the compiled installation binaries.

### 4.2 Compiled Release Assets
Upon successful compilation, the following binaries are uploaded to the GitHub Release:
- `clondar/src-tauri/target/release/bundle/nsis/*.exe` (NSIS silent executable installer)
- `clondar/src-tauri/target/release/bundle/wix/*.msi` (WiX installer package)

---

## 5. Verifying Releases

1. Open the repository on GitHub and navigate to "**Releases**".
2. Confirm the published release for version `v1.4.1` exists and the MSI / EXE installer assets are attached.
