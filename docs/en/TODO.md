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

---

## 2. Current Tasks (Todo)

- [ ] **Align Crate Reference Paths**
  - **Issue**: Standard developer documentation (`docs/ja/DEVELOPMENT.md` and `.agents/AGENTS.md`) mandates using Git repository paths inside `Cargo.toml` to prevent CI build failures, overriding paths locally using `.cargo/config.toml`. Currently, `src-tauri/Cargo.toml` directly references relative directories `{ path = "../../common_lib" }`.
  - **Resolution**: Revert `Cargo.toml` changes to reference remote repositories, enforcing local overrides via `.cargo/config.toml`.
- [x] **Frontend Validation for Holiday Inputs**
  - **Issue**: Saving corrupted configurations triggers errors on the Rust backend because formatting checks are scoped to `serde_json` serialization.
  - **Resolution**: Perform `JSON.parse` logic on the frontend when clicking save buttons, rendering error warnings.
- [x] **Expand Unit Tests**
  - **Issue**: The GitHub CI runner executes `cargo test`, but unit tests and React component testing suites are sparse.
  - **Resolution**: Write unit tests for date calculation algorithms and coordinate restoring logic.

---

## 3. Future Extensions (Backlog)

- [ ] **Dynamic Localization Toggle**
  - Translate calendar days, holiday titles, and preference labels between English and Japanese on the fly.
- [ ] **Personalization Settings Dashboard**
  - Enable users to adjust window transparency slider values and customize widget theme colors (currently locked to default blue).
- [ ] **Monospace Widget Planner**
  - Let users click calendar date cells to add brief schedules or todo items, rendering indicator markings on the grid.
- [ ] **Automated Holidays Syncing**
  - Auto-fetch the latest `holidays.json` from GitHub in the background at startup to sync dates dynamically.
- [ ] **System Startup Integration**
  - Integrate autostart toggles into the settings panel (e.g., using Tauri's autostart plugin).
