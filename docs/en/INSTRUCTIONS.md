# Clondar Pro AI Development Instructions (INSTRUCTIONS.md)

**English** | [日本語版](../ja/INSTRUCTIONS.md)

This document defines guidelines for coding styles, error handling conventions, component design principles, and output formatting that AI assistants must adhere to when maintaining or extending Clondar Pro source code (Rust backend / React frontend).

---

## 1. Naming Conventions

Maintain consistency with existing codebase patterns:

### 1.1 Backend (Rust)
- **Files / Modules**: `snake_case` (e.g. `main.rs`, `lib.rs`)
- **Functions / Variables**: `snake_case` (e.g. `load_holidays_json`, `get_holidays_file_path`, `app_handle`, `win_clone`)
- **Structs / Enums / Types**: `CamelCase` (e.g. `QuittingState`, `DiffPart`)
- **Constants**: `UPPER_SNAKE_CASE` (e.g. `DEFAULT_HOLIDAYS_JSON`)
- **Tauri Commands**: Defined as `snake_case` in Rust. Ensure invocations from frontend match exactly.

### 1.2 Frontend (Vite / React / JavaScript)
- **File Names**:
  - UI Component files: `PascalCase.jsx` (e.g. `Calendar.jsx`, `Clock.jsx`, `HolidaysManager.jsx`)
  - Utility / Helper files: `camelCase.js` (e.g. `holidays.js`, `tauri.js`)
- **Component Functions**: `PascalCase` (e.g. `WindowFrame`, `App`, `DigitalClock`)
- **Standard Functions / Variables**: `camelCase` (e.g. `initHolidays`, `reloadHolidays`)

---

## 2. Error Handling Policy
- **Backend (Rust)**: Avoid `panic!` or bare `.unwrap()`. Handle recoverable errors gracefully by returning `Result<T, String>`.
- **Frontend (React)**: Wrap all Tauri API invocations and IPC commands in `try-catch` blocks. Fall back safely to `fallbackConfig` upon failure.

---

## 3. Component & Module Boundaries
- **`main.rs` (Main Process Logic)**: Window decorations (borderless, transparent, shadowless), system tray menu setup, and IPC command registration.
- **`common_lib` (Shared Crate)**: Pure shared algorithms such as LCS text diffing and single-instance named mutex locking.
- **Frontend Architecture**: Segregate into `src/components/` (view layer), `src/utils/` (logic & API layer), and `App.jsx` (orchestrator).

---

## 4. Documentation Synchronization Rules

When modifying source code, keep both Japanese (`docs/ja/`) and English (`docs/en/`) documentation in sync.
Always place bidirectional language links at the top of each document.
*(Note: Verification and automated document updates can be skipped when modifying `*.md` files only.)*

| Document File                | Role            | Trigger for Update                                                 |
| :--------------------------- | :-------------- | :----------------------------------------------------------------- |
| `CHANGELOG.md`               | Release notes   | Features added, bugs fixed, optimizations applied, or code removed |
| `TODO.md`                    | Task status     | Tasks completed (Done), started (In Progress), or proposed (Backlog)|
| `SPECIFICATION.md`           | Specs & UI      | Screen layouts, data structures, or shortcuts changed              |
| `ARCHITECTURE.md`            | System design   | System boundaries, IPC interfaces, or process lifecycles altered   |
| `DEVELOPMENT.md`             | Dev guide       | Build processes, dependencies, or developer commands updated       |
| `RELEASE.md`                 | Release manual  | Version bumping, tagging, or release pipeline changed              |
| `USER_GUIDE.md`              | Operation guide | UI features, user options, or interactions changed                 |
| `README.md` / `README_JA.md` | Overview        | Startup commands, requirements, or key highlights modified         |
| `TEST_REPORT.md`             | Test logs       | Self-test executions, results recorded, or architecture diagrams  |
| `TESTING.md`                 | Testing policy  | Test policies, runner configurations, or check processes changed   |
| `FOOTPRINTS.md`              | Performance     | Binary dimensions, RAM footprint, or compile times updated         |
| `CONTRIBUTING.md`            | Conventions     | Commit rules, PR guidelines, or setup instructions changed         |
| `SECURITY.md`                | Security policy | Security architectures, supported releases, or report channels     |

---

## 5. Quality Assurance & Verification Process
*(Note: When modifying `*.md` files only, skip code verification commands for rapid turnaround.)*
- **File Length Limit (1000 lines)**: When a single source file (`*.rs`, `*.jsx`, `*.js`) exceeds **1000 lines**, proactively refactor into modular components.
- **Local Verification Checklist**: Ensure zero errors and zero warnings across:
  1. `cargo test --manifest-path src-tauri/Cargo.toml`
  2. `npm test`
  3. `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
  4. `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
  5. `npm run lint`
- **Window Decorations on Windows**: Ensure `shadow: false` in `tauri.conf.json` and `window.set_shadow(false).unwrap();` in `main.rs` are strictly preserved.
