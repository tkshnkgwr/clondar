# Clondar Pro (v2.0.0)

**English** | [日本語版](README_JA.md)

**Clondar Pro** is a minimalist, borderless, shadowless desktop clock & calendar widget for Windows—built with **100% Pure Rust (`egui` / `eframe` + `glow`)** for single-process, low-memory, completely offline residency.

![Clondar Pro Preview](docs/assets/screenshot.png)

> [!NOTE]
> Starting with **v2.0.0**, Clondar Pro migrated away from Tauri v2 + WebView2 + React to a single-process Pure Rust native GUI (`egui + glow`). The legacy Tauri v2 (`v1.4.4`) codebase is preserved at the `v1.4.4` Git tag and commit history.

---

## ✨ Key Features

- **Borderless, Shadowless & Wallpaper-Integrated UI**:
  - Eliminates OS window chrome and drop shadows, with optional background transparency mode and Light/Dark palettes.
- **Digital & Analog Clocks**:
  - **Digital Mode**: High-contrast bold tabular digits with zero horizontal jitter, 12H/24H (AM/PM) toggle, and seconds toggle.
  - **Analog Mode**: Enlarged circular dial with smooth sweep second-hand rotation (automatically disables the `24H` toggle while active).
  - Locked bottom control positions across clock modes.
- **6-Week Fixed Calendar & Japanese Holiday Engine (`holidays.rs`)**:
  - Locked 42-day (6-week) monthly layout prevents vertical window jumps across months.
  - Native Rust calculation for Japanese fixed holidays, Happy Mondays, Vernal/Autumnal Equinoxes, Emperors' Birthdays, substitute holidays, bridge national holidays, and special overrides.
  - Built-in **12-Month Yearly Overlay** and **Holidays Manager** (visual editing, Raw JSON editing, LCS diff comparison, keyword occurrence metrics, Reset, and validation before saving).
- **Custom Vector Line Icons & Crisp Japanese Typography**:
  - Draws crisp painter-based vector icons (`icons.rs`) to avoid OS emoji/tofu glyph fallbacks, paired with system Japanese fonts (`font.rs`).
- **System Tray, Pinning & Persistence**:
  - Hide to system tray via `Esc` or close button; restore, pin Always on Top, reset position, or quit cleanly from the tray context menu.
  - Single-instance lock via Windows Named Mutex (`com.clondar.pro.mutex`) and local state/coordinate persistence in `%LOCALAPPDATA%\com.clondar.pro\state.json`.
  - 100% offline with zero external network requests.

---

## 📊 Performance & Footprint Highlights (v2.0.0)

Real Windows 11 simultaneous comparison benchmark (`2026-11-16`):

| Architecture / Version               | Processes | Private Memory |  WorkingSet   | Notes                                                 |
| :----------------------------------- | :-------: | :------------: | :-----------: | :---------------------------------------------------- |
| **v2.0.0 / Phase C (`egui + glow`)** |   **1**   |  **68.45 MB**  | **115.76 MB** | Pure Rust OpenGL backend (`Clondar.exe`: **4.89 MB**) |
| Phase C Proto (`egui + wgpu`)        |     1     |   293.06 MB    |   349.54 MB   | Heavy D3D12/Vulkan/DXC shader DLL overhead            |
| Phase B (`v1.4.4`, Tauri + WebView2) |     7     |   190.74 MB    |   364.81 MB   | Active WebView2 multi-process baseline                |
| Legacy (`v1.4.1`, WebView2)          |     7     |   207.51 MB    |   192.17 MB   | Pre-optimization Tauri + React baseline               |

* **Private memory reduced by 64.1%** (`-122.29 MB`) vs. Phase B WebView2 and **76.6%** (`-224.61 MB`) vs. `wgpu`.
* NSIS bilingual installer (`Clondar_Pro_2.0.0_x64-setup.exe`): **2.59 MB**.

---

## 💻 System Requirements

### End Users
- **OS**: Windows 10 / 11 (64-bit)
- **Runtime Dependencies**: None required (standalone native executable; WebView2 not needed)

### Developers
- **Rust**: `1.96.0+` (`x86_64-pc-windows-msvc`)
- **NSIS (`makensis`)**: Required only for building the Windows setup installer (`installer/clondar.nsi`)
- **Shared Crate**: Clone [`common_lib`](https://github.com/tkshnkgwr/common_lib) side-by-side at `../common_lib`

---

## 🚀 Build, Verification & Packaging Commands

```powershell
# Clone clondar alongside common_lib
git clone https://github.com/tkshnkgwr/clondar.git
git clone https://github.com/tkshnkgwr/common_lib.git
cd clondar

# Run in development mode
cargo run

# Quality & Verification Suite
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps --document-private-items

# Production release binary (outputs target\release\Clondar.exe)
cargo build --release

# Build bilingual NSIS installer (outputs installer\Clondar_Pro_2.0.0_x64-setup.exe)
makensis installer\clondar.nsi
```

---

## 📖 Documentation Index

| Document            | English                                      | 日本語                                       |
| :------------------ | :------------------------------------------- | :------------------------------------------- |
| **Product Spec**    | [SPECIFICATION.md](docs/en/SPECIFICATION.md) | [SPECIFICATION.md](docs/ja/SPECIFICATION.md) |
| **Architecture**    | [ARCHITECTURE.md](docs/en/ARCHITECTURE.md)   | [ARCHITECTURE.md](docs/ja/ARCHITECTURE.md)   |
| **User Guide**      | [USER_GUIDE.md](docs/en/USER_GUIDE.md)       | [USER_GUIDE.md](docs/ja/USER_GUIDE.md)       |
| **Developer Guide** | [DEVELOPMENT.md](docs/en/DEVELOPMENT.md)     | [DEVELOPMENT.md](docs/ja/DEVELOPMENT.md)     |
| **Release Guide**   | [RELEASE.md](docs/en/RELEASE.md)             | [RELEASE.md](docs/ja/RELEASE.md)             |
| **Testing Policy**  | [TESTING.md](docs/en/TESTING.md)             | [TESTING.md](docs/ja/TESTING.md)             |
| **Test Report**     | [TEST_REPORT.md](docs/en/TEST_REPORT.md)     | [TEST_REPORT.md](docs/ja/TEST_REPORT.md)     |
| **Footprints**      | [FOOTPRINTS.md](docs/en/FOOTPRINTS.md)       | [FOOTPRINTS.md](docs/ja/FOOTPRINTS.md)       |
| **Changelog**       | [CHANGELOG.md](docs/en/CHANGELOG.md)         | [CHANGELOG.md](docs/ja/CHANGELOG.md)         |
| **Roadmap & TODO**  | [TODO.md](docs/en/TODO.md)                   | [TODO.md](docs/ja/TODO.md)                   |
| **Contributing**    | [CONTRIBUTING.md](docs/en/CONTRIBUTING.md)   | [CONTRIBUTING.md](docs/ja/CONTRIBUTING.md)   |
| **Security Policy** | [SECURITY.md](docs/en/SECURITY.md)           | [SECURITY.md](docs/ja/SECURITY.md)           |
| **AI Instructions** | [INSTRUCTIONS.md](docs/en/INSTRUCTIONS.md)   | [INSTRUCTIONS.md](docs/ja/INSTRUCTIONS.md)   |

---

## 📜 License

Licensed under the **GNU General Public License v3.0 or later ([GPL-3.0-or-later](LICENSE))**.
