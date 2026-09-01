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

---

## 2. Current Tasks (Todo)

- [ ] **Align Crate Reference Paths**
  - **Issue**: Standard developer documentation (`docs/ja/DEVELOPMENT.md` and `.agents/AGENTS.md`) mandates using Git repository paths inside `Cargo.toml` to prevent CI build failures, overriding paths locally using `.cargo/config.toml`. Currently, `src-tauri/Cargo.toml` directly references relative directories `{ path = "../../common_lib" }`.
  - **Resolution**: Revert `Cargo.toml` changes to reference remote repositories, enforcing local overrides via `.cargo/config.toml`.

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

### ⚡ Usability & System Integration
- [ ] **Global Hotkey Integration (Instant Show/Hide)**
  - Register global keyboard shortcuts (e.g. `Alt+Shift+C`) to instantly summon, hide, or pin the desktop widget during other desktop tasks.
- [ ] **System Startup Integration**
  - Integrate autostart toggles into the settings panel (e.g., using Tauri's autostart plugin) for direct startup enrollment.
- [ ] **Ghost / Click-Through Transparency Mode**
  - Temporarily increase transparency on mouse hover or allow mouse events to click through to underlying desktop windows.

### 📅 Scheduling & Data Interoperability
- [ ] **Monospace Widget Planner & Desktop Reminders**
  - Let users click calendar date cells to register quick memos/schedules, rendering indicator markings on the grid and scheduling native Windows toast notifications.
- [ ] **Automated Holidays Syncing**
  - Auto-fetch the latest `holidays.json` from GitHub in the background at startup to sync dates dynamically upon legislative amendments.
- [ ] **Holiday / Event Import & Export (iCalendar / CSV / JSON)**
  - Provide data export and import tools supporting standard `.ics` (iCalendar), CSV, and JSON formats for seamless synchronization with Google Calendar or Outlook.
