# Footprints (FOOTPRINTS.md) - Clondar

**English** | [日本語版](../ja/FOOTPRINTS.md)

This document records binary sizes, memory footprint resources, and compiler performance indicators during release builds of `Clondar Pro`.

---

## 1. Measurement Environment

- **OS**: Windows 11 64-bit
- **Toolchain**: Rust stable (`x86_64-pc-windows-msvc`), Node.js v26.4.0
- **Target Assets**:
  - Application Executable: `src-tauri/target/release/clondar.exe`
  - Installer Packages: `src-tauri/target/release/bundle/nsis/*.exe` / `wix/*.msi`

---

## 2. Footprint Records

### 2.1 Binary & Package Sizes

| Build Type / Asset Name | Size | Notes |
| :--- | :---: | :--- |
| `clondar.exe` (Debug) | ~ 45 - 60 MB | Contains debug symbols and verbose log pipelines |
| `clondar.exe` (Release) | ~ 10 - 15 MB | LTO and symbol stripping configurations active |
| NSIS Installer (`.exe`) | ~ 5.0 - 7.5 MB | Compressed single silent installer executable |
| WiX Installer (`.msi`) | ~ 4.5 - 6.5 MB | Isolated regional localized MSI packages |

> **Highlights**: By utilizing OS-native WebView2 runtimes as shared platform dependencies rather than embedding whole chromium platforms (like Electron shells which exceed 100MB+ raw binary size), Clondar Pro achieves an incredibly lightweight package footprint.

### 2.2 Memory Footprint (RAM)

- **Main Process (Rust Core)**: ~ 3.0 MB - 5.0 MB
- **Renderer Process (WebView2 / Edge)**: ~ 25 MB - 45 MB
- **Total Resident Execution Memory (Total Peak)**: ~ 30 MB - 55 MB

> **Highlights**: Optimized local asset loaders, regulated polling intervals, and regulated layout repaint scopes ensure that this resident clock & calendar widget runs in the background without affecting CPU or RAM performance.

### 2.3 Compilation Performance (Local Estimation)

- **Clean compilation (`cargo tauri build`)**: ~ 60s - 120s
- **Incremental reloading (`cargo tauri dev`)**: ~ 1.0s - 3.5s

---

## 3. Release Compilation Optimization Configuration

To maximize binary size reductions and regulate runtime resource footprint overheads, the following optimization matrices are applied in `src-tauri/Cargo.toml`:

```toml
[profile.release]
opt-level = 3        # Optimize for speed
lto = true           # Enable Link-Time Optimization
codegen-units = 1    # Reduce compilation units for better optimizations
panic = "abort"      # Omit panic call stack tracing to trim binary size
strip = true         # Strip symbol tables and debug entries
```
