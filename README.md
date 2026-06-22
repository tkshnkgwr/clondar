# 🕒 Clondar

**English** | [日本語版](./README.ja.md)

<!-- UPDATE 2026-06-21: Translated main README to English and linked to README.ja.md under multi-language structure. -->

**Clondar** is a modern desktop widget-type clock & calendar application that strikes an ultra-minimal look with a transparent, borderless, and shadowless design blending natively into your Windows desktop.

Powered by Tauri v2, it operates with extremely low resource consumption, stays "Always on Top" if desired, and lets you check your schedules and time in a beautiful, streamlined view.

![Clondar Showcase](https://picsum.photos/seed/clondar/1200/600?blur=1)

## ✨ Core Features

- **💎 Genuine Borderless Design**: Windows default titlebars, borders, and window drop shadows are completely eliminated.
- **🌑 Transparent Canvas & Glassmorphism**: Preserves your beautiful desktop wallpaper underneath while utilizing soft background-blur filters for peak readability.
- **⏰ Hybrid Clock**:
  - **Digital**: Bold, muscular display based on Impact-inspired typography. Supports 12H/24H formats and show/hide seconds sub-clock.
  - **Analog**: Super-minimal sweep second hand movement.
- **📅 Smart Calendar**:
  - Full support for Japanese national holidays (including substitute and national press holidays).
  - Built-in visual stability with a locked 6-week grid layout.
  - Full-screen "Yearly Calendar" modal (with quick toggling to previous/next years).
- **💾 Robust State Persistence (v1.2.0)**:
  - **Physical Coordinate Restoring (DPI-Aware)**: Built to handle multi-monitor or high-DPI scaling (125%, 150%, etc.) to eliminate layout drift on start.
  - **Startup Race-Condition Guard**: Incorporates an `isRestoringRef` locking gate preventing the window's centering animation from accidentally overwriting clean coordinate saves in LocalStorage.
  - Restores clock visual preferences (seconds visible, 24-hour style), opacity level, and Always on Top toggle automatically upon relaunch.
- **🖱️ Drag-Anywhere Canvas**: Fully interactive drag region mapped across the widget profile page.
- **📌 Stay on Top**: Freeze the widget window over other app views to monitor calendars at a glance.
- **🌓 Dynamic Dark Mode**: Synchronizes beautifully with system styling preferences.

## 🛠️ Tech Stack

- **Backend / Core Engine**: Rust ([Tauri v2](https://v2.tauri.app/))
- **Frontend / Framework**: HTML5 (CDN React 18) / Tailwind CSS
- **Animation**: [Framer Motion](https://www.framer.com/motion/) (CSS Transitions)
- **Styling**: Modern UI matching Fluent Design guidelines

---

## 🚀 Getting Started & Local Development

To run or compile this widget locally, you will need **Node.js** and **Rust** installed on your system.

### 1. Clone the Repository
```bash
git clone https://github.com/tkshnkgwr/clondar.git
cd clondar
```

### 2. Launch Dev server
```bash
# Frontend is served using zero-dependency CDN delivery, so 'npm install' is NOT required.
cargo tauri dev
```

### 3. Build Production Installer
```bash
cargo tauri build
```
Once build finishes, you will find compilation assets (.msi or .exe installer) generated under `src-tauri/target/release/bundle/msi/` or `src-tauri/target/release/bundle/nsis/`.

---

## 🎨 Changing Your Application Icons

You can easily customize desktop and system tray icons by following these steps:

### Method A: Automated Asset Slicing (Recommended)
Tauri supports instant icon resizing through a single high-resolution square image source (512x512px or higher is recommended).

1. Place your target image as `source_icon.png` in the project root.
2. Open terminal and run:
```bash
npx tauri icon /path/to/source_icon.png
```
This utility automatically replaces all standard files inside `src-tauri/icons/` (e.g. `icon.ico`, `icon.icns`, and scaling PNG copies) and wires them to `tauri.conf.json`.

### Method B: Manual Replacements
If you prefer manual placement, overwrite the files below with identical file extensions and names:
- **Windows Executable & Taskbar**: Replace `src-tauri/icons/icon.ico`
- **Other OS Tray & Platform UI PNG assets**: Replace scaling assets under `src-tauri/icons/`

After replacing, trigger a `cargo clean` and execute `cargo tauri dev` or `cargo tauri build` to render new icon skins onto your system environment.

---

## 📝 Desktop Widget Design Directives (Tauri v2 Golden Setup)

This widget is specialized for Windows and low-spec environments using specific design parameters:

- **1. Shadow Removal**:
  Binds Rust-side `set_shadow(false)` commands with `tauri.conf.json` properties (`"shadow": false`) to fully eliminate default OS borders/glow margins.
- **2. Scoped Permissions**:
  Strict access keys (`allow-start-dragging`, `allow-close`, `allow-outer-position`, `allow-set-position`) are configured in `capabilities/default.json` to keep system memory footprints lightweight and sandboxed.
- **3. Pointer Events**:
  Configures `html, body { pointer-events: auto; }` so you can click and drag anywhere even on transparent backgrounds.

## ⚠️ OS Warnings

- **Windows SmartScreen**: Custom, un-signed installers will warrant a standard diagnostic security screen. Click on "More Info" and select "Run Anyway" during your initial boot.
- **WebView2**: Standard in Windows 10 & 11 base machines. Older environments or stripped operating environments might require installing Microsoft WebView2 Runtime.

## 📄 License

[MIT License](LICENSE)

---
Developed by [lunatic.chariot](mailto:lunatic.chariot@gmail.com)
