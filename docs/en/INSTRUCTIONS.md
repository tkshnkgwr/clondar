# Clondar Pro AI Development Instructions (INSTRUCTIONS.md)

**English** | [日本語版](../ja/INSTRUCTIONS.md)

This document defines the coding style, error handling policy, module boundaries, and documentation synchronization guidelines for AI assistants maintaining **Clondar Pro** (Pure Rust native codebase under `src/`).

---

## 1. Naming Conventions

- **Files / Modules**: `snake_case` (e.g., `main.rs`, `clock_pane.rs`, `holidays_manager.rs`, `yearly_view.rs`)
- **Functions / Variables**: `snake_case` (e.g., `calculate_holidays`, `load_or_default`, `render_digital_clock`)
- **Structs / Enums / Traits**: `CamelCase` (e.g., `ClondarApp`, `AppStateConfig`, `ClockType`, `HolidayConfig`)
- **Constants**: `UPPER_SNAKE_CASE` (e.g., `DEFAULT_HOLIDAYS_JSON`, `APP_MUTEX_NAME`)
- **Comments & RustDoc**: Write module docs (`//!`), doc comments (`///`), and non-obvious inline rationale in Japanese (`AGENTS.md`).

---

## 2. Error Handling Policy

- Avoid `panic!` or bare `.unwrap()` in production paths; prefer `Result<T, String>` or safe fallbacks.
- If `state.json` or `holidays.json` is absent or malformed, fall back cleanly to `AppStateConfig::default()` or `DEFAULT_HOLIDAYS_JSON` without blocking startup.

---

## 3. Module Boundaries

- **`main.rs`**: Application state management, borderless transparent window setup, tray event handling, calendar pane, and modal dispatch.
- **`clock_pane.rs`**: Digital/Analog clock rendering and bottom toggle buttons.
- **`holidays.rs` / `config.rs`**: Japanese holiday calculation engine, schema validation, and `%LOCALAPPDATA%\com.clondar.pro\` persistence I/O.
- **`icons.rs` / `theme.rs` / `font.rs`**: Vector line icons, Light/Dark/Transparent color palettes, and Windows Japanese font loading.
- **`yearly_view.rs` / `holidays_manager.rs` / `help_modal.rs`**: Full-canvas overlay modals.
- **`tray.rs`**: `tray-icon` wrapper and context menu handling.
- **`common_lib` (Shared Crate)**: LCS text diffing, occurrence counting, and Named Mutex single-instance locking.

---

## 4. Documentation Synchronization Rules

When modifying source code, synchronize affected files under both `docs/ja/` and `docs/en/` and preserve bidirectional header links.  
*(Note: Code verification checks may be skipped when modifying `*.md` documentation files only.)*

| Document File                | Role            | Trigger for Update                                                 |
| :--------------------------- | :-------------- | :----------------------------------------------------------------- |
| `CHANGELOG.md`               | Release notes   | Features added, bugs fixed, optimizations applied, or code removed |
| `TODO.md`                    | Task status     | Tasks completed (Done), started (In Progress), or proposed         |
| `SPECIFICATION.md`           | Specs & UI      | Layouts, data structures, or shortcuts modified                    |
| `ARCHITECTURE.md`            | System design   | Module layout, persistence layer, or process flow altered          |
| `DEVELOPMENT.md`             | Dev guide       | Build workflows, dependencies, or CLI setup changed                |
| `RELEASE.md`                 | Release manual  | Versioning scripts, NSIS setup, or release pipeline changed        |
| `USER_GUIDE.md`              | Operation guide | UI features, user options, or interactions modified                |
| `README.md` / `README_JA.md` | Overview        | Requirements, build commands, or highlights updated                |
| `TEST_REPORT.md`             | Test logs       | Verification results or system architecture diagrams updated       |
| `TESTING.md`                 | Testing policy  | Test strategies, commands, or quality gates modified               |
| `FOOTPRINTS.md`              | Performance     | Binary dimensions, RAM usage, or compile timings updated           |
| `CONTRIBUTING.md`            | Conventions     | Commit formats, PR guidelines, or setup steps changed              |
| `SECURITY.md`                | Security policy | Security architecture or supported versions updated                |

---

## 5. Quality Assurance & Verification Process

*(Skip code checks when editing `*.md` files only.)*
- **File Length Target (1000 lines)**: Refactor and split modules proactively before any single Rust source file exceeds 1,000 lines.
- **Pre-Commit Checklist**: Ensure zero errors and zero warnings across:
  1. `cargo fmt --check`
  2. `cargo clippy --all-targets -- -D warnings`
  3. `cargo test`
  4. `cargo doc --no-deps --document-private-items`
- **Vertically Aligned Tables**: Align every Markdown table pipe (`|`) and header separator (`-`) for plain-text editor readability.
