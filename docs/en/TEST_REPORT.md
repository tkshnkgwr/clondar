# Self-Test Report & System Architecture Diagram (Obsidian Integration)

**English** | [日本語版](../ja/TEST_REPORT.md)

## 1. Self-Test Report

### Testing Environment
- **OS**: Windows 11 (Simulation replicating resource-restricted / low-spec systems)
- **Runtime**: Tauri v2, Rust 1.96.0, Node.js v26.4.0
- **View Configuration**: Window transparency active, Always on Top active, borderless (Decorations: false), system tray residency active
- **Frontend Architecture**: Local Bundled SPA (Vite + React)

### Testing Matrices & Results

| Test Item ID | Testing Target | Expected Behavior | Verification Result | Determination |
| :--- | :--- | :--- | :--- | :---: |
| TS-001 | Startup Position Recovery | Cleanly restores absolute physical coordinates capture at previous exit. | Replaced custom methods with Tauri v2 `setPosition` and optimized object fetches via `getCurrentWebviewWindow`. Restores coordinates dynamically across high-DPI configs (125%/150%). | **PASS** |
| TS-002 | Startup Collision Save-Guard | Startup auto-centering routines do not overwrite clean LocalStorage coordinate records. | Restricted write locks during boot for 1 second using `isRestoringRef`. Completely eliminated saving loops. | **PASS** |
| TS-003 | Always on Top Maintenance | Widgets anchor cleanly above all running application windows. | Verified toggles anchor and load correctly during startup. | **PASS** |
| TS-004 | Transparency & Shadow Removal | Window canvas renders transparently without displaying standard OS borders or drop shadows. | WinAPI backend configs `set_shadow(false)` combined with `shadow: false` configurations cleanly render borderless transparency. | **PASS** |
| TS-005 | Drag Tracking on Transparency | Transparent canvas areas successfully catch dragging pointer events. | Applying `pointer-events: auto` maintains drag tracking on transparent backgrounds. | **PASS** |
| TS-006 | Scoped Security Capabilities | Decouples unnecessary permissions from configs without triggering compilation errors. | Deleted `allow-set-outer-position` keys, scoping capabilities purely to `allow-outer-position` and `allow-set-position` for Tauri v2. Build succeeds. | **PASS** |
| TS-007 | GitHub Actions Release Pipelines | Triggering tagged pushes launches automatic release compilation and drafts releases without errors. | Checked YAML compiler syntax and arguments compatibility. | **PASS** |
| TS-008 | Dependabot Configurations | Verifies Dependabot syntax tracking is correct. | Checked syntax structures in `.github/dependabot.yml`. | **PASS** |
| TS-009 | Editor standards compliance | Validates indentation rules and ensures PowerShell script encodings are read correctly. | Confirmed UTF-8 BOM encoding applications in editors. | **PASS** |
| TS-010 | System Tray Operations | Toggles windows, Always on Top status, coordinate resets, and exits correctly via tray menu context clicks. | Enabled `tray-icon` cargo features and configured menus. Toggles sync frontend react states instantly through event emitters. | **PASS** |
| TS-011 | External Holidays Loadings | Loads custom holiday lists dynamically from local JSON files. | Read custom settings inside `public/config/holidays.json` and computed dates for fixed holidays, Happy Mondays, and Emperor birthdays. | **PASS** |
| TS-012 | Local Bundled Vite Compilation | Resolves and packages all dependencies locally to ensure offline capabilities. | Vite compilation (`npm run build`) runs successfully and bundles React/Tailwind/Framer Motion local packages, letting the app start offline. | **PASS** |
| TS-013 | Shared Library `common_lib` Integration | Integrates shared Rust code and passes unit tests. | Standard testing `cargo test` passes validation methods (`it_works`, `test_compute_diff`, `test_count_occurrences`). | **PASS** |
| TS-014 | Holidays Manager UI Operations | Compares configuration differences in Rust and renders metrics in tabular views with color indicators. | Tauri commands `get_holidays_diff` and `get_word_count` route successfully. UI displays insertions (green) and deletions (red) correctly. | **PASS** |
| TS-015 | Visual Holiday Modifications | Adding and deleting holiday definitions dynamically functions in-memory. | Form insertions and trash-bin deletions run cleanly on temporary arrays. | **PASS** |
| TS-016 | AppData Local Persistence | Overwrites holidays files inside `%LOCALAPPDATA%` on save and reloads calendar UI elements instantly. | Triggering save buttons dispatches `save_holidays_json` commands, rewriting the file, reloading calendar state, and persisting changes on relaunch. | **PASS** |
| TS-017 | Holidays Manager Dark Mode Themes | Elements do not clash in dark mode (backgrounds stay dark). | Swapped invalid Tailwind class configurations from `slate-850` to `slate-800` to fix background rendering. | **PASS** |
| TS-018 | Local Workspace Path Resolution | Compiles and passes unit tests using relative path imports locally. | Resolves `common_lib` path dependencies through configuration overrides. | **PASS** |
| TS-019 | Manual Release Versioning | Bumps version attributes and packages release installers only on git tagged pushes. | Deleted auto-bumping files and verified local script executions. | **PASS** |
| TS-020 | Release Build Binary Optimizations | Optimization profile flags do not trigger compiler linking errors. | Compiles and links binaries successfully under release flags. | **PASS** |
| TS-021 | Vite Dynamic Import Warnings | Resolves compiler warnings concerning dynamic import targets. | Swapped dynamic loaders in `HolidaysManager.jsx` with static imports. | **PASS** |
| TS-022 | Source Code Localizations | Confirmed all developer comments inside source codes are translated to Japanese. | Compiled frontend and checked backend components without warnings. | **PASS** |
| TS-023 | GitHub Actions CI Workflows | CI runner triggers on pushes and PRs, validating formats, clippy checks, and unit tests. | Checked `ci.yml` syntax structures and authentication overrides for checkout runners. | **PASS** |
| TS-024 | Multiple Instance Block Guard | Named Mutex stops multiple application windows launching simultaneously. | Enforced named mutex locks at startup. The second process exits cleanly if Mutex acquisition fails. | **PASS** |

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
            WebUI["UI Shell (ui/index.html)"]:::frontend
            AppJSX["Main App Logic (App.jsx)"]:::frontend
            ClockJSX["Clock Component (Clock.jsx)"]:::frontend
            CalendarJSX["Calendar Component (Calendar.jsx)"]:::frontend
            HolidaysManager["Holidays Manager (HolidaysManager.jsx)"]:::frontend
            TauriJS["Tauri wrapper (tauri.js)"]:::frontend
            LocalStorage["localStorage<br/>- windowPosition (Physical coordinates)<br/>- keepsAlwaysOnTop"]:::storage
        end

        subgraph BackEnd [Backend Layer (Rust / Tauri Core / Crate)]
            TauriConf["Configuration (tauri.conf.json)<br/>- shadow: false<br/>- transparent: true<br/>- decorations: false"]:::backend
            Capabilities["Permissions capabilities (default.json)<br/>- allow-outer-position<br/>- allow-set-position<br/>- allow-close"]:::backend
            RustMain["Rust Main Program (main.rs)<br/>- System Tray Menu Builder<br/>- rust-version: 1.96.0"]:::backend
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
    AppJSX --> HolidaysManager
    CalendarJSX -->|"Load Holiday Definitions"| HolidaysJSON
    HolidaysManager -->|"Read/Write via Tauri"| HolidaysJSON
    HolidaysManager -->|"Diff/Stats Command"| TauriRuntime
    TauriRuntime -->|"Execute Command"| RustMain
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
    class WebUI,AppJSX,ClockJSX,CalendarJSX,TauriJS frontend;
    class TauriConf,Capabilities,RustMain backend;
    class LocalStorage storage;
    class HolidaysJSON config;
```

---
**Created**: July 10, 2026
**Suitable Version**: Widget v1.3.7 (DPI-Aware Physical Coordination Model with Tauri v2 compatibility)
