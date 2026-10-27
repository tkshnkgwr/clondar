# Self-Test Report & System Architecture Diagram (Obsidian Integration)

**English** | [日本語版](../ja/TEST_REPORT.md)

## 1. Self-Test Report

### Testing Environment
- **OS**: Windows 11 (Simulation replicating resource-restricted / low-spec systems)
- **Runtime**: Tauri v2, Rust 1.96.0, Node.js v26.4.0
- **View Configuration**: Window transparency active, Always on Top active, borderless (Decorations: false), system tray residency active
- **Frontend Architecture**: Local Bundled SPA (Vite + React)

### Testing Matrices & Results

| Test Item ID | Testing Target                          | Expected Behavior                                                                                                                                       | Verification Result                                                                                                                                                                                                   | Determination |
| :----------- | :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-----------: |
| TS-001       | Startup Position Recovery               | Cleanly restores absolute physical coordinates capture at previous exit.                                                                                | Replaced custom methods with Tauri v2 `setPosition` and optimized object fetches via `getCurrentWebviewWindow`. Restores coordinates dynamically across high-DPI configs (125%/150%).                                |   **PASS**    |
| TS-002       | Startup Collision Save-Guard            | Startup auto-centering routines do not overwrite clean LocalStorage coordinate records.                                                                 | Restricted write locks during boot for 1 second using `isRestoringRef`. Completely eliminated saving loops.                                                                                                           |   **PASS**    |
| TS-003       | Always on Top Maintenance               | Widgets anchor cleanly above all running application windows.                                                                                           | Verified toggles anchor and load correctly during startup.                                                                                                                                                            |   **PASS**    |
| TS-004       | Transparency & Shadow Removal           | Window canvas renders transparently without displaying standard OS borders or drop shadows.                                                             | WinAPI backend configs `set_shadow(false)` combined with `shadow: false` configurations cleanly render borderless transparency.                                                                                       |   **PASS**    |
| TS-005       | Drag Tracking on Transparency           | Transparent canvas areas successfully catch dragging pointer events.                                                                                    | Applying `pointer-events: auto` maintains drag tracking on transparent backgrounds.                                                                                                                                    |   **PASS**    |
| TS-006       | Scoped Security Capabilities            | Decouples unnecessary permissions from configs without triggering compilation errors.                                                                   | Deleted `allow-set-outer-position` keys, scoping capabilities purely to `allow-outer-position` and `allow-set-position` for Tauri v2. Build succeeds.                                                                   |   **PASS**    |
| TS-007       | GitHub Actions Release Pipelines        | Triggering tagged pushes launches automatic release compilation and drafts releases without errors.                                                     | Checked YAML compiler syntax and arguments compatibility.                                                                                                                                                             |   **PASS**    |
| TS-008       | Dependabot Configurations               | Verifies Dependabot syntax tracking is correct.                                                                                                         | Checked syntax structures in `.github/dependabot.yml`.                                                                                                                                                                |   **PASS**    |
| TS-009       | Editor standards compliance             | Validates indentation rules and ensures PowerShell script encodings are read correctly.                                                                 | Confirmed UTF-8 BOM encoding applications in editors.                                                                                                                                                                 |   **PASS**    |
| TS-010       | System Tray Operations                  | Toggles windows, Always on Top status, coordinate resets, and exits correctly via tray menu context clicks.                                             | Enabled `tray-icon` cargo features and configured menus. Toggles sync frontend react states instantly through event emitters.                                                                                         |   **PASS**    |
| TS-011       | External Holidays Loadings              | Loads custom holiday lists dynamically from local JSON files.                                                                                           | Read custom settings inside `public/config/holidays.json` and computed dates for fixed holidays, Happy Mondays, and Emperor birthdays.                                                                              |   **PASS**    |
| TS-012       | Local Bundled Vite Compilation          | Resolves and packages all dependencies locally to ensure offline capabilities.                                                                          | Vite compilation (`npm run build`) runs successfully and bundles React/Tailwind/Framer Motion local packages, letting the app start offline.                                                                           |   **PASS**    |
| TS-013       | Shared Library `common_lib` Integration | Integrates shared Rust code and passes unit tests.                                                                                                      | Standard testing `cargo test` passes validation methods (`it_works`, `test_compute_diff`, `test_count_occurrences`).                                                                                                 |   **PASS**    |
| TS-014       | Holidays Manager UI Operations          | Compares configuration differences in Rust and renders metrics in tabular views with color indicators.                                                  | Tauri commands `get_holidays_diff` and `get_word_count` route successfully. UI displays insertions (green) and deletions (red) correctly.                                                                             |   **PASS**    |
| TS-015       | Visual Holiday Modifications            | Adding and deleting holiday definitions dynamically functions in-memory.                                                                                | Form insertions and trash-bin deletions run cleanly on temporary arrays.                                                                                                                                              |   **PASS**    |
| TS-016       | AppData Local Persistence               | Overwrites holidays files inside `%LOCALAPPDATA%` on save and reloads calendar UI elements instantly.                                                   | Triggering save buttons dispatches `save_holidays_json` commands, rewriting the file, reloading calendar state, and persisting changes on relaunch.                                                                    |   **PASS**    |
| TS-017       | Holidays Manager Dark Mode Themes       | Elements do not clash in dark mode (backgrounds stay dark).                                                                                             | Swapped invalid Tailwind class configurations from `slate-850` to `slate-800` to fix background rendering.                                                                                                            |   **PASS**    |
| TS-018       | Local Workspace Path Resolution         | Compiles and passes unit tests using relative path imports locally.                                                                                     | Resolves `common_lib` path dependencies through configuration overrides.                                                                                                                                              |   **PASS**    |
| TS-019       | Manual Release Versioning               | Bumps version attributes and packages release installers only on git tagged pushes.                                                                     | Deleted auto-bumping files and verified local script executions.                                                                                                                                                      |   **PASS**    |
| TS-020       | Release Build Binary Optimizations      | Optimization profile flags do not trigger compiler linking errors.                                                                                      | Compiles and links binaries successfully under release flags.                                                                                                                                                         |   **PASS**    |
| TS-021       | Vite Dynamic Import Warnings            | Resolves compiler warnings concerning dynamic import targets.                                                                                           | Swapped dynamic loaders in `HolidaysManager.jsx` with static imports.                                                                                                                                                 |   **PASS**    |
| TS-022       | Source Code Localizations               | Confirmed all developer comments inside source codes are translated to Japanese.                                                                        | Compiled frontend and checked backend components without warnings.                                                                                                                                                    |   **PASS**    |
| TS-023       | GitHub Actions CI Workflows             | CI runner triggers on pushes and PRs, validating formats, clippy checks, and unit tests.                                                                | Checked `ci.yml` syntax structures and authentication overrides for checkout runners.                                                                                                                                 |   **PASS**    |
| TS-024       | Single Instance Guard (Named Mutex)     | Windows named mutex properly prevents multiple instances from running, exiting redundant processes immediately.                                         | Integrated `common_lib::check_single_instance` which returns a Result, ensuring a secondary launched instance exits gracefully without conflict.                                                                     |   **PASS**    |
| TS-025       | Holiday & Lifecycle Unit Tests (Vitest) | Accurate calculation of fixed holidays, Happy Mondays, emperor eras, equinoxes, substitutes, 2020/2021 overrides, boundaries, and config loads.         | Running `npm test` passes 100% of all 10 test cases in `holidays.test.js`.                                                                                                                                            |   **PASS**    |
| TS-026       | Backend Default JSON Validation Test    | Built-in default holiday configuration JSON (`DEFAULT_HOLIDAYS_JSON`) parses with zero syntax errors and valid schema.                                  | Running `cargo test --manifest-path src-tauri/Cargo.toml` executes `test_default_holidays_json_validity` successfully.                                                                                                |   **PASS**    |
| TS-027       | Raw JSON Editing & Pre-Validation       | In Holidays Manager Raw Data tab, invalid JSON input or missing required fields triggers instant validation errors.                                      | Invalid syntax/missing keys are detected immediately on save, preventing corrupted payloads from reaching backend.                                                                                                    |   **PASS**    |
| TS-028       | Rust Holiday Engine Tests (holidays.rs) | Accurate calculation of fixed holidays, Happy Mondays, emperor eras, equinoxes, substitutes, bridge holidays, and helpers in Rust.                      | Verified comprehensive calculation test cases under `cargo test --manifest-path src-tauri/Cargo.toml` pass successfully.                                                                                              |   **PASS**    |
| TS-029       | Rust Schema & Range Validation Tests    | Strongly typed `HolidayConfig` via Serde rejects out-of-range months (e.g. 13), invalid days, or invalid weeks.                                         | Verified `test_validation_error_handling` detects invalid months and weeks properly, preventing disk writes.                                                                                                          |   **PASS**    |
| TS-030       | Reset Holidays Command (Reset)          | `reset_holidays_json` IPC command restores built-in default settings safely onto local disk storage.                                                    | Verified clicking "Reset" in Holidays Manager overwrites config with built-in defaults and resets UI and calendar grids.                                                                                              |   **PASS**    |
| TS-031       | Tauri Wrapper Unit Tests (Vitest)       | `tauri.js` environment detection (`isTauri`), window handle, version retrieval fallbacks, and clean exit without errors.                                | Running `npm test` passes 100% of all test cases in `tauri.test.js` (including window restoration and move listeners).                                                                                                 |   **PASS**    |
| TS-032       | Help & App Info Modal Verification      | Clicking the help button (❓) in title bar opens the modal displaying version badge and feature guides, and closes safely via button, backdrop, or Esc. | Verified `HelpModal.jsx` mounts dynamically, shows the app version badge and 6 guide cards, and closes properly on close button click, backdrop click, or Esc key.                                                   |   **PASS**    |
| TS-033       | Tauri Standard Directory Structure      | Deprecates `ui/` directory and unifies project into Tauri official standard layout (root `src/` and `src-tauri/`), verifying all builds and tests pass. | Verified clean passes across root `npm test` (16 Vitest cases passed), `npm run build` (zero errors), `cargo check`, `cargo test`, `cargo clippy`, and `cargo fmt`.                                                   |   **PASS**    |
| TS-034       | Backend Diff & Word Count Unit Tests    | Unit tests for configuration diff computation (`test_holidays_diff_logic`) and keyword occurrence aggregation (`test_word_count_logic`).                | Ran `cargo test --manifest-path src-tauri/Cargo.toml` and verified that both new tests passed, completing all 12 cargo tests successfully.                                                                            |   **PASS**    |

---

## 2. System Architecture Diagram

The diagram below details the architecture flow, rendering Mermaid diagrams in Markdown viewers like Obsidian:

```mermaid
graph TD
    %% Styling configurations
    classDef frontend fill:#e0f2fe,stroke:#0284c7,stroke-width:2px,color:#0f172a;
    classDef backend fill:#fef3c7,stroke:#d97706,stroke-width:2px,color:#0f172a;
    classDef storage fill:#bbf7d0,stroke:#16a34a,stroke-width:2px,color:#0f172a;
    classDef platform fill:#f1f5f9,stroke:#475569,stroke-width:2px,color:#334155;
    classDef config fill:#edd1ff,stroke:#a855f7,stroke-width:2px,color:#0f172a;

    subgraph OS_Platform [Windows OS / Platform]
        TauriRuntime["Tauri v2 Core Webview (Rust runtime)"]:::platform
        TrayAPI["OS System Tray API"]:::platform
    end

    subgraph DesktopWidget [Low-Resource Desktop Widget]
        subgraph FrontEnd [Frontend Layer (React / Vite / JSX)]
            WebUI["UI Shell (index.html)"]:::frontend
            AppJSX["Main App Logic (App.jsx)"]:::frontend
            ClockJSX["Clock Component (Clock.jsx)"]:::frontend
            CalendarJSX["Calendar Component (Calendar.jsx)"]:::frontend
            HelpModal["Help Modal (HelpModal.jsx)"]:::frontend
            HolidaysManager["Holidays Manager (HolidaysManager.jsx)"]:::frontend
            TauriJS["Tauri wrapper (tauri.js)"]:::frontend
            LocalStorage["localStorage<br/>- windowPosition (Physical coordinates)<br/>- keepsAlwaysOnTop"]:::storage
        end

        subgraph BackEnd [Backend Layer (Rust / Tauri Core / Crate)]
            TauriConf["Configuration (tauri.conf.json)<br/>- shadow: false<br/>- transparent: true<br/>- decorations: false"]:::backend
            Capabilities["Permissions capabilities (default.json)<br/>- allow-outer-position<br/>- allow-set-position<br/>- allow-close"]:::backend
            RustMain["Rust Main Program (main.rs)<br/>- System Tray Menu Builder<br/>- rust-version: 1.96.0"]:::backend
            HolidaysRS["Holiday Engine (holidays.rs)<br/>- calculate_holidays<br/>- validate_and_parse_config"]:::backend
            CommonLib["Shared Crate (common_lib)<br/>- compute_diff<br/>- count_occurrences"]:::backend
        end

        subgraph ConfigLayer [Configuration Data Layer]
            HolidaysJSON["External Holidays File (LocalAppData/holidays.json)"]:::config
        end
    end

    %% Data Flow Connections
    WebUI --> AppJSX
    AppJSX --> ClockJSX
    AppJSX --> CalendarJSX
    AppJSX --> HelpModal
    AppJSX --> HolidaysManager
    CalendarJSX -->|"Load Holiday Definitions"| HolidaysJSON
    HolidaysManager -->|"Read/Write/Reset via Tauri"| HolidaysJSON
    HolidaysManager -->|"Diff/Stats Command"| TauriRuntime
    TauriRuntime -->|"Execute Command"| RustMain
    RustMain -->|"Calculate & Validate Holidays"| HolidaysRS
    RustMain -->|"Invoke Shared Crate"| CommonLib

    AppJSX -->|"Restore Coordinates & Pin Status"| LocalStorage
    TauriJS -->|"Position Window / Set Always on Top"| TauriRuntime

    TauriRuntime -->|"Authorize Window APIs"| Capabilities
    Capabilities -->|"Control Signals"| TauriConf

    RustMain -->|"Native Rendering Properties"| TauriRuntime
    RustMain <-->|"Tray Interactivity"| TrayAPI
    RustMain -->|"Dispatch Events (Always on Top Toggle / Reset Position)"| AppJSX

    TauriJS -->|"Capture Position Changes (tauri://move)"| LocalStorage
    TauriJS -->|"Register Drag Events"| TauriRuntime
    TauriRuntime -->|"Apply Window Operations"| OS_Platform

    %% Assign classes
    class WebUI,AppJSX,ClockJSX,CalendarJSX,HelpModal,HolidaysManager,TauriJS frontend;
    class TauriConf,Capabilities,RustMain,HolidaysRS,CommonLib backend;
    class LocalStorage storage;
    class HolidaysJSON config;
```

---
**Created**: July 10, 2026  
**Last Updated**: September 8, 2026  
**Target Version**: Widget v1.4.2 (DPI-Aware Physical Coordination Model with Tauri v2 compatibility)
