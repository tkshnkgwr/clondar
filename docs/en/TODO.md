# Clondar Pro Development Todo List (TODO.md)

**English** | [日本語版](../ja/TODO.md)

This document manages development progress, current tasks, and future feature backlogs for Clondar Pro.

---

## 1. Completed Features (Done)

- [x] **Widget Infrastructure (Tauri v2)**
  - [x] Implemented window transparency, borderless layouts, and disabled OS shadows.
  - [x] Configured window dragging via `data-tauri-drag-region`.
  - [x] Enabled window closing via `Esc` keys or the close (`❌`) button.
  - [x] Configured single-instance locks (Named Mutex).
- [x] **System Tray Residence**
  - [x] Configured tray menu with right-click menu controls (Show/Hide, Always on Top, Reset Position, Exit).
  - [x] Intercepted window close commands to hide window interfaces instead of destroying them.
  - [x] Synced tray actions and React states via Tauri IPC events.
- [x] **Clock Section**
  - [x] Rendered monospace digital clocks (12H/24H format, toggleable seconds display).
  - [x] Built sweep-second analog clocks.
- [x] **Calendar Section**
  - [x] Built locked 6-week (42-day) calendar grids.
  - [x] Implemented holiday date calculation rules parsing external JSON configurations (`holidays.json`). Renders fixed dates, Happy Mondays, Emperors' birthdays, Equinoxes, and overrides.
  - [x] Built full-screen yearly calendar viewer with previous/next year pagination.
- [x] **Holidays Manager**
  - [x] Built UI forms to visually add or delete holiday definitions.
  - [x] Displayed layout comparison diffs dynamically using LCS algorithms in `common_lib`.
  - [x] Built statistics grids tracking keyword occurrences.
- [x] **Preference Persistence & Positioning Restoration**
  - [x] Saved styling configurations (seconds visible, 24H formatting, dark themes, transparent canvases) to `LocalStorage`.
  - [x] Recorded absolute physical coordinates at exit, restoring positions dynamically at relaunch (DPI-aware).
  - [x] Implemented startup coordinate guards (`isRestoringRef`) to avoid window animations overwriting saved coordinates.
- [x] **Compilation & Distribution Packages**
  - [x] Compiled multi-language NSIS EXE installers.
  - [x] Compiled separate WiX MSI packages for Japanese and English system locales.
- [x] **Performance Optimization & Document Standardization (v1.3.8)**
  - [x] Optimized clock update timer interval dynamically based on display mode (reducing idle CPU & power consumption).
  - [x] Memoized calendar date grid array (`calendarDays`) using `useMemo` to eliminate unnecessary recalculations.
  - [x] Standardized all documentation across CommonMark / GFM markdown syntax and vertically aligned tables.
- [x] **Frontend Validation for Holiday Inputs (v1.3.8)**
  - [x] Implemented instant syntax parsing (`JSON.parse`) and schema validation on the "Raw JSON" tab inside Holidays Manager to prevent corrupted payloads from triggering Rust backend panics.
- [x] **Expand Unit Tests (v1.3.8)**
  - [x] Expanded test coverage for holiday calculation algorithms (`holidays.test.js`) and backend default JSON validation (`main.rs`).
- [x] **Help Modal, Rust Holiday Engine & Standard Directory Layout (v1.4.1)**
  - [x] Implemented `HelpModal.jsx` providing application overview, version badge, and operational guide cards.
  - [x] Migrated holiday calculation engine to Rust (`src-tauri/src/holidays.rs`), added strong Serde schema/range validation, and implemented `reset_holidays_json`.
  - [x] Unified directory layout to the official Tauri standard (root `src/` and `src-tauri/`) and expanded unit test suites across frontend and backend.
- [x] **Offline Reliability & Static Configuration Alignment (v1.4.2)**
  - [x] Removed external Google Fonts CDN links from `index.html` to guarantee 100% offline self-containment.
  - [x] Aligned dynamic Emperor's Birthday calculation in `public/config/holidays.json` by eliminating duplicates.
  - [x] Prevented URL resolution errors (`Invalid URL`) in headless Vitest environments with `window.location.origin` safety guards.
  - [x] Added `npm test` step to CI workflows and mapped `npm run lint` in `package.json`.
- [x] **Open Source Licensing Update (v1.4.3)**
  - [x] Transitioned project license from MIT License to GNU General Public License v3.0 or later (GPL-3.0-or-later).
  - [x] Updated license notices across `LICENSE`, `package.json`, `Cargo.toml`, `README.md`, `README_JA.md`, and all documentation.
- [x] **Holiday Engine Optimization, Input Validation & Documentation Complete (v1.4.4)**
  - [x] Introduced annual holiday memory cache in `holidays.js` and automated prefetching via Rust backend's `get_holidays` IPC command in Tauri environments.
  - [x] Implemented month/day boundary and monthly day-limit input validation in `HolidaysManager.jsx`.
  - [x] Synchronized version fallback badge in `HelpModal.jsx` with `tauri.js` (`0.0.0-dev`).
  - [x] Added null-safe unlisten guards in `App.jsx` lifecycle teardowns.
  - [x] Comprehensive overhaul of `README.md` / `README_JA.md` and complete technical documentation alignment.

---

## 2. Current Tasks (Todo)

There are currently no pending immediate tasks. All core features and verification suites are complete.

---

## 3. Future Extensions (Backlog)

### 🎨 UI & Display Customization
- [ ] **Dynamic Localization Toggle**
  - Translate calendar days, holiday titles, and preference labels between English and Japanese on the fly.
- [ ] **Personalization Settings Dashboard**
  - Enable users to adjust window transparency slider values and customize widget theme colors (currently locked to default blue).
- [ ] **Compact Layout Mode (Minimal Widget Mode)**
  - Provide a toggle to collapse the widget into an ultra-compact layout showing only the clock or only the current week calendar.
- [ ] **First Day of the Week Selection (Sunday / Monday)**
  - Allow users to switch between Sunday-first and Monday-first calendar grids according to business workflows.
- [ ] **ISO 8601 Week Number Display Option**
  - Toggle displaying annual week numbers (e.g. W01..W52) on the left margin of calendar rows.
- [ ] **Multi-Timezone / World Clock Sub-Display**
  - Render a secondary compact clock displaying UTC or selected international timezones (e.g., New York, London).
- [ ] **Widget Scaling & Display Scaling Slider**
  - Enable users to smoothly adjust widget zoom scale (80% to 150%) across high-DPI and 4K displays.
- [ ] **Alarm & Countdown Timer (Pomodoro Tracker)**
  - Integrate a customizable countdown timer (e.g., 25-minute work intervals) and scheduled alarms with Windows toast notifications.

### ⚡ Usability & System Integration
- [ ] **Global Hotkey Integration (Instant Show/Hide)**
  - Register global keyboard shortcuts (e.g. `Alt+Shift+C`) to instantly summon, hide, or pin the desktop widget during other desktop tasks.
- [ ] **System Startup Integration**
  - Integrate autostart toggles into the settings panel (e.g., using Tauri's autostart plugin) for direct startup enrollment.
- [ ] **Ghost / Click-Through Transparency Mode**
  - Temporarily increase transparency on mouse hover or allow mouse events to click through to underlying desktop windows.
- [ ] **Automatic Update Checker via GitHub Releases**
  - Check for newer releases in the background at startup or within the Help Modal, notifying users with an unobtrusive update badge.
- [ ] **Complete Frontend Integration of Rust Holiday Engine (`get_holidays`)**
  - Unify dual holiday calculation logic by routing Tauri frontend requests directly through the Rust `get_holidays` command for maximal execution speed and maintainability.

### 📅 Scheduling & Data Interoperability
- [ ] **Monospace Widget Planner & Desktop Reminders**
  - Let users click calendar date cells to register quick memos/schedules, rendering indicator markings on the grid and scheduling native Windows toast notifications.
- [ ] **Automated Holidays Syncing**
  - Auto-fetch the latest `holidays.json` from GitHub in the background at startup to sync dates dynamically upon legislative amendments.
- [ ] **Holiday / Event Import & Export (iCalendar / CSV / JSON)**
  - Provide data export and import tools supporting standard `.ics` (iCalendar), CSV, and JSON formats for seamless synchronization with Google Calendar or Outlook.

---

## 4. Ver.2.0 Lightweight Optimization & Architecture Roadmap

A major optimization roadmap based on actual memory profiling on 2026-11-16 (~424.7 MB total working set / ~197.6 MB private memory: **8.32 MB** Rust core + **189.31 MB** WebView2 processes) and source code diagnostics to eliminate redundant background work and memory overhead.

### 🚀 Phase A: Frontend Redesign & Redundancy Removal
- [x] **Localize Clock State & Isolate Re-renders (`src/App.jsx`, `Calendar.jsx`)**
  - Remove the root-level `setTime(new Date())` timer (100ms–250ms) that forces `WindowFrame` and `Calendar` (42 day cells) to re-render multiple times per second; encapsulate tick updates inside `Clock` (or use direct DOM updates via `useRef`) and apply `React.memo`.
- [x] **Remove Backdrop Blur Effects (`.glass` / `backdrop-filter: blur(16px)`) (`src/index.css`, `Clock.jsx`, `Calendar.jsx`)**
  - Eliminate continuous backdrop-blur compositing (currently applied even when background transparency is off) to reduce GPU compositor load and `gpu-process` private memory usage, replacing it with flat semi-transparent or solid backgrounds.
- [x] **Eliminate `framer-motion` Dependency (`package.json`, `Clock.jsx`, `Calendar.jsx`, `App.jsx`)**
  - Replace 100ms-ticking `<motion.div>` clock hands, hover tooltips, and modal transitions with lightweight CSS transitions and direct CSS `transform` updates to cut bundle size and runtime overhead.
- [x] **Remove 5-Second Window Position IPC Polling (`src/App.jsx`)**
  - Rely solely on debounced `listenToMove` events and pre-close persistence, eliminating periodic `win.outerPosition()` IPC calls and `localStorage` writes while idle.
- [x] **Eliminate Per-Frame & Loop Object Allocations (`src/components/Clock.jsx`, `Calendar.jsx`)**
  - Hoist/cache `Intl.DateTimeFormat` instances outside `DigitalClock`, and move `new Date()` "today" comparisons out of the 42-day (`Calendar`) and 365-day (`YearlyView`) render loops to minimize garbage collection pressure.
- [x] **Deduplicate Holiday Config & Dual Calculation Logic (`main.rs`, `holidays.js`, `public/config/holidays.json`)**
  - Consolidate the triplicated default holiday JSON definitions and eliminate redundant JS-plus-Rust double holiday calculation on initial render.
- [x] **Clean Up Dead Code, Double `show()`, and Minor Redundancies (`lib.rs`, `tauri.conf.json`, `HolidaysManager.jsx`, `tailwind.config.js`)**
  - Remove unused `src-tauri/src/lib.rs` (`run()`) and `tauri-plugin-log` dependency, start with `visible: false` so `restoreWindowPosition` calls `show()` once after positioning, batch the 4 sequential `get_word_count` IPC calls in `HolidaysManager`, and remove leftover Google Font names (`Inter` / `JetBrains Mono`).
- [x] **Lazy-Load Modals (`React.lazy`)**
  - Split `YearlyView`, `HolidaysManager`, and `HelpModal` out of the initial bundle and mount cycle to reduce baseline memory footprint.

### ⚡ Phase B: Tauri / WebView2 Runtime Optimization
- [ ] **Tune WebView2 Launch Arguments (`additional_browser_args`)**
  - Disable browser features unnecessary for a desktop clock/calendar widget (spellchecking, media pipelines, background sync, etc.) to trim WebView2 `utility` process overhead.
- [ ] **Destroy or Suspend WebView When Hidden to Tray (`src-tauri/src/main.rs`)**
  - Instead of only calling `hide()`, destroy and recreate the WebView window (or suspend it) when minimized to the system tray so idle memory stays within the Rust core footprint (~8–15 MB).
- [ ] **Evaluate React Alternatives**
  - Consider migrating from React 18 to Preact, SolidJS, or Vanilla TypeScript to minimize renderer JS heap and runtime overhead.

### 🦀 Phase C: Fundamental Architecture Overhaul (Long-Term Option)
- [ ] **Evaluate Pure Rust Native GUI (No WebView2)**
  - Since WebView2 accounts for ~96% of current private memory usage, explore migrating rendering to **Win32 + Direct2D / DirectComposition** or a lightweight Rust GUI framework (**Slint / iced / egui**) to achieve an ultra-lightweight **10–30 MB** resident widget.
