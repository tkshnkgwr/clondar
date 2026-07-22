# Testing Guidelines (TESTING.md) - Clondar

**English** | [日本語版](../ja/TESTING.md)

This document details testing strategies, quality check policies, and execution procedures for the `Clondar Pro` project.

---

## 1. Testing Overview

`Clondar Pro` coordinates backend tasks (Rust / Tauri) alongside frontend components (React / Vitest). We utilize comprehensive test cases to verify widget responsiveness and algorithm accuracy.

Key testing targets include:
- **Holiday Calculation Algorithms (Frontend)**:
  - Verifies date calculations for fixed holidays, Happy Mondays, Vernal/Autumnal Equinoxes, substitute holidays, and custom overrides compiled inside `ui/src/utils/holidays.js` (validated by `holidays.test.js`).
- **Data Load Consistency (Backend)**:
  - Validates structures of built-in fallback holiday JSON templates and confirms clean file recoveries under data corruption.
- **Shared Crate `common_lib` Integration**:
  - Validates LCS string alignments for line diff displays, and checks keyword occurrences metric calculations.
- **Physical Coordinates Management**:
  - Ensures correct captures of absolute physical positions at exit, coordinate restoration on boot, and confirms startup race protection guards (`isRestoringRef`) in mixed multi-monitor setups.
- **Single Instance Verification (Named Mutex)**:
  - Validates the Named Mutex lock, confirming duplicate processes terminate cleanly.

---

## 2. Test Execution Procedures

Run the following routines in local development environments to verify code states:

### 2.1 Backend Tests (Rust)
```bash
# Run Rust unit tests
cargo test --manifest-path src-tauri/Cargo.toml
```

### 2.2 Frontend Tests (React / Vitest)
```bash
# Run Vitest test suites
npm --prefix ui run test
```

### 2.3 Quality Verification Pipeline (Pre-Commit checklist)
Before committing updates or submitting pull requests, run the following verification checks:

```bash
# 1. Run backend tests
cargo test --manifest-path src-tauri/Cargo.toml

# 2. Run static analysis checks (Clippy)
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings

# 3. Check format compliance
cargo fmt --manifest-path src-tauri/Cargo.toml --check

# 4. Validate Rustdoc builds
cargo doc --manifest-path src-tauri/Cargo.toml --no-deps --document-private-items
```

---

## 3. Test Writing Directives

1. **Expanding Holiday Test Metrics**:
   * If legislation updates public holidays, insert corresponding test assertions in `ui/src/utils/holidays.test.js` to verify calculation offsets.
2. **Boundary Auditing**:
   - Ensure comprehensive coverage of edge cases, including year-end wrap-arounds (Dec 31st to Jan 1st), leap years, and multiple consecutive substitute holidays (e.g. May Golden Week slots).
3. **OS-Dependent Scoping**:
   - Wrap Windows-specific Mutex or system tray tests inside `#[cfg(windows)]` directives to prevent compilation errors on non-Windows test runners (e.g. GitHub Actions CI VMs).
