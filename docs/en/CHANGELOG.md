# Changelog

**English** | [日本語版](../ja/CHANGELOG.md)

## [Unreleased]
### Added
- **Unit tests for holiday calculation logic in the frontend (Vitest)**: Created unit tests in `ui/src/utils/holidays.test.js` to verify calculations for fixed holidays, old/new Happy Mondays, astronomical calculations (Vernal/Autumnal Equinox), substitute holidays, national press holidays, and custom overrides for specific years (e.g., Olympic specials).
- **Validation test for default JSON in the backend**: Implemented a unit test in `src-tauri/src/main.rs` to ensure the built-in default holidays JSON (`DEFAULT_HOLIDAYS_JSON`) parses correctly and contains all mandatory fields.
- **Added document comments (RustDoc/JSDoc) across the codebase**:
  - Added RustDoc comments to the Rust backend (`lib.rs` / `main.rs`).
  - Added JSDoc comments to the React frontend (`main.jsx` / `App.jsx`, components, and utilities) to document arguments (`@param`) and return values (`@returns`).
- **Added documentation synchronization rules to developer guidelines**:
  - Added "9. Creation and Synchronization of Document Comments (RustDoc/JSDoc)" to `.agents/AGENTS.md`. This mandates updating document comments in sync with future specification changes, setting Japanese as the default language for comments.

### Changed
- **Implemented visual editing for "Raw Config Data" and frontend JSON validation**: Changed the "Raw Config Data" tab in `HolidaysManager.jsx` from a read-only (`pre`) block to an editable textarea. Configured validation checks (JSON syntax and basic schema checks for objects and arrays) on save and tab switches to prevent corruption.
- **Localized error messages and log outputs to Japanese**:
  - Translated `expect` error messages in `lib.rs` / `main.rs` and debug logs in `tauri.js` to Japanese.

### Fixed
- **Resolved dynamic import warnings during Vite builds**:
  - Replaced dynamic imports (`await import(...)`) for `@tauri-apps/api/core` and `reloadHolidays` in `HolidaysManager.jsx` and `holidays.js` with static imports (`import`) at the top of the file to fix chunk division warnings.

### Changed
- **Unified version management and fixed the bump script**:
  - Set `src-tauri/tauri.conf.json` as the Single Source of Truth (SSOT) for the application version. The frontend (`App.jsx`) now fetches this dynamically at startup using Tauri APIs.
  - Reduced the files modified during version updates to only `tauri.conf.json` and `Cargo.toml`.
  - Refactored `scripts/bump-version.ps1` to cleanly modify only these two files, avoiding replacement errors.
- **Replaced README placeholder images with local screenshots**: Substituted the dummy placeholder image in the README with an actual UI capture of the running app (`docs/assets/screenshot.png`).
- **Localized inline source code comments to Japanese**:
  - Translated comments in `src-tauri/src/main.rs`, `ui/src/utils/holidays.js`, and `ui/src/utils/tauri.js` from English to Japanese.
- **Migrated to manual release workflow and deprecated auto-bump**:
  - Deleted the `bump-version.yml` workflow which automatically bumped version numbers on push.
  - Standardized on running `scripts/bump-version.ps1` locally, committing/tagging (`v*`), and pushing to trigger the automated release build (`release.yml`).

## [1.3.1] - 2026-07-06
### Added
- **Automated release & version bump workflow**: Introduced the `bump-version.yml` workflow to automatically increment patch versions and push tags upon merging into the `main` branch.
- **Release build optimizations**: Added profile settings (LTO, stripping symbols) in `src-tauri/Cargo.toml` to reduce output binary sizes.

### Changed
- **Shared library `common_lib` reference configuration**: Swapped `common_lib` inside `src-tauri/Cargo.toml` from a Git dependency back to a local relative path. Modified CI/CD (`ci.yml`, `release.yml`) to checkout `common_lib` alongside the app. Removed the temporary override configuration file `src-tauri/.cargo/config.toml`.

### Fixed
- **Fixed colors on the holidays manager screen in dark mode**:
  - Corrected an invalid Tailwind class `slate-850` to `slate-800` in the stat count cards and lists, fixing an issue where they stayed white/light grey in dark mode.
  - Corrected hover backgrounds in the list from `dark:hover:bg-slate-850/50` to `dark:hover:bg-slate-800/50`.
- **Fixed compilation errors in CI workflows**:
  - Added a frontend build step (`npm run build`) before running `cargo clippy` in `ci.yml`. This fixes a `proc macro panicked` error caused by Tauri's `tauri::generate_context!` macro requiring `frontendDist` (`ui/dist`) to exist.

## [1.3.0] - 2026-07-03
### Added
- **Visual holiday editing and persistence**: Added forms to add or delete fixed holidays on the "Holidays Config" screen. Changes are saved to OS-specific user folders via Tauri commands and applied instantly.
- **Integrated `common_lib` shared crate and added Holidays Manager**: Leveraged `compute_diff` (LCS diff algorithm) and `count_occurrences` from `common_lib` to display deviations from the default holiday set in a new "Holidays Manager" screen.
- **Updated GitHub Actions build workflow**: Configured checking out and referencing the shared crate `common_lib` in GitHub workflows to ensure CI builds succeed.
- **Updated development guidelines**: Standardized updating `docs/DEVELOPMENT.md` whenever core architecture changes occur, documenting this rule in `.agents/AGENTS.md`.
- **System tray resident menu**: Implemented system tray menu on the Rust backend allowing show/hide, Always on Top toggle, position reset, and quitting the app via right-click.

### Fixed
- **Layout and Modal UI optimizations**:
  - Expanded window width from `1100px` to `1180px` (WindowFrame to `1140px`) to prevent calendar header elements (year/month and buttons) from breaking into two lines. Added `whitespace-nowrap` and `flex-shrink-0`.
  - Optimized the Holidays Manager modal size to `460px` height and `5xl` max width. Added `overflow-y-auto` to the sidebar to make all statistics scrollable and readable.
- **Fixed window close panic**: Fixed a `FailedToReceiveMessage` panic when opening the app from the system tray after closing the window. Hooked the window close event to hide the window instead of destroying it.
- **Resolved WebView2 exit warning**: Introduced a `QuittingState` to distinguish between window closing and application termination, resolving a Win32 exit warning (Error 1412) in WebView2.
- **Offline operation support (Vite + React)**: Bundled all frontend assets locally with Vite, cutting off all external CDN dependencies and allowing the app to run completely offline.
- **External holidays configuration (`holidays.json`)**: Moved holiday data to `ui/public/config/holidays.json` to allow updating definitions without recompiling source code.
- **Created developer guide (`docs/DEVELOPMENT.md`)**: Wrote guide detailing Vite+React setup, component structure, building, tray implementation, and holiday rules.
- **Status badges in README**: Added status badges indicating build success, release versions, and platform versions.
- **Updated guidelines**: Added requirements for maintaining README badges to `.agents/AGENTS.md`.
- **Japanese installer localization**: Enabled multi-language NSIS installers (with language prompts) and parallel WiX (MSI) builds for Japanese (`ja-JP`) and English (`en-US`).

### Changed
- **Integrated single instance lock (Named Mutex)**:
  - Integrated `common_lib::check_single_instance` at startup. If another instance is running, exit immediately (`std::process::exit(0)`).
- **CI and Dependabot configuration fixes**:
  - Changed `common_lib` reference to a Git dependency in `Cargo.toml` to fix Dependabot check failures.
  - Added `.cargo/config.toml` (git-ignored) locally to override Git path with a relative directory for local development.
  - Refactored release workflow paths and updated deprecated Actions.
  - Introduced code verification workflow (`ci.yml`) triggering on pushes/PRs.
- **React Component refactoring**: Refactored the frontend UI into `Clock.jsx`, `Calendar.jsx`, `App.jsx`, and `main.jsx`.
- **Automatic frontend build compilation**: Re-wired Vite build triggers to Tauri `beforeDevCommand` and `beforeBuildCommand`.
- **Updated requirements**: Synchronized Rust version requirement to `1.96.0` and Tauri versions to `2.11.3`.
- **Calendar UI tweak**: Changed the "Current Month" button text to a home icon.

### Fixed
- **Fixed `node_modules` rule in `.gitignore`**: Removed leading slash from `/node_modules/` to ensure subfolder node_modules are ignored.
- **Fixed VS Code schema warnings**: Fixed Rust formatter schema warnings in `.vscode/settings.json`.
- **Fixed CDN framer-motion (UMD) crashes**: Fixed global namespace binding errors when loading framer-motion (Motion) from CDNs.
- **Cleaned JSX comments in htm templates**: Removed leftover `{/* ... */}` comments inside htm tags which caused parser errors.

## [1.2.3] - 2026-06-26
### Added
- **Dependabot configuration**: Added `.github/dependabot.yml` to automatically track Cargo and Actions dependency updates.
- **Automated release workflow**: Configured `.github/workflows/release.yml` to compile and draft GitHub Releases on tag push.
- **Editor Standards**: Standardized spacing, indentation, line endings, and file encodings using `.editorconfig` and `.vscode/settings.json`.

### Changed
- **Updated project templates guide**: Added documentation on new EditorConfig rules and workflows to `docs/PROJECT_TEMPLATE_GUIDE.md`.
- **Updated guidelines**: Added rules for maintaining editor and release pipelines to `.agents/AGENTS.md`.

## [1.2.2] - 2026-06-24
### Added
- **Version bump script**: Introduced `scripts/bump-version.ps1` to automate version increment operations.

### Changed
- **Unified versioning**: Standardized version references to `1.2.2` across all project files.
- **Organized file structure**: Moved `SPECIFICATION.md` and `TEST_REPORT.md` to `docs/` folder.
- **Moved configuration files**: Moved `AGENTS.md` to `.agents/AGENTS.md`.
- **Consolidated `.gitignore`**: Tracked `Cargo.lock` in git and consolidated backend/frontend gitignore rules.
- **Dynamic version string**: Fetched application version from Tauri APIs rather than hardcoding it.

### Fixed
- **Fixed Tauri v2 position APIs**:
  - Updated window object retrieval to match Tauri v2's global structure (`window.__TAURI__.webviewWindow.getCurrentWebviewWindow`).
  - Swapped obsolete `setOuterPosition` call with `setPosition`.
  - Added on-close coordinate capturing to ensure positions save right before exit.

## [1.2.1] - 2026-06-21
### Added
- **Multi-language README**: Separated the main README to English (`README.md`) and Japanese (`README.ja.md`).
- **Icon customization instructions**: Documented how to set custom application icons in the README.
- **Self-test report and diagrams**: Created `TEST_REPORT.md` including self-test results and Mermaid architecture diagrams.

## [1.2.0] - 2026-06-15
### Fixed
- **Fixed coordinate persistence races**: Resolved a bug where the window's center animation race-overwrote correct coordinates in LocalStorage on startup.
- **Implemented coordinate save locking**: Introduced `isRestoringRef` to prevent coordinate saves within 1 second after startup.
- **DPI-aware positioning**: Enforced using Tauri's physical coordinates instead of logical scaling positions to eliminate DPI coordinate drift.
- **Permissions settings**: Configured window dragging and position permission keys in `capabilities/default.json`.

## [1.1.0] - 2026-04-16
### Added
- **Tauri v2 Migration**: Migrated the application core to Tauri v2.
- **Window transparency**: Implemented borderless, transparent, and shadowless views.
- **Permissions config**: Set up `capabilities/default.json` for dragging and closing.
- **Developer marker logs**: Log markers added for file sync checks (removed later).

### Fixed
- **Eliminated drop shadows**: Resolved Windows default drop shadow remaining on transparent canvasses using Rust `set_shadow(false)`.
- **Stabilized transparent dragging**: Enforced `pointer-events: auto` to allow clicking and dragging transparent regions.
- **Esc / Close key hooks**: Ensured window exit shortcuts function correctly via Tauri v2.
- **Disabled mobile scaling styles**: Removed responsive classes to prioritize widget stability.

### Changed
- **Refreshed visual style**: Removed borders and shadow elements for a modern flat widget experience.
- **Changed clock font**: Changed the digital clock font to Impact.

## [1.0.0] - 2026-04-11
### Added
- Initial Release (Web/Standalone HTML version).
- Integrated digital/analog clock, Japanese holidays calendar, and yearly viewer modal.
- Configured Dark Mode and Glassmorphism styling.
