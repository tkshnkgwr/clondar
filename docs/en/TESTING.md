# Testing Guidelines (TESTING.md) - Clondar

**English** | [日本語版](../ja/TESTING.md)

This document describes the testing strategy, quality gate policies, and verification commands for **Clondar Pro v2.0.0** (Pure Rust edition).

---

## 1. Testing Overview

Because v2.0.0 unifies both core logic and native GUI rendering inside a single Rust crate at the repository root, all verification is executed directly via standard `cargo` tooling.

Primary testing targets include:
- **Japanese Holiday Calculation Engine (`src/holidays.rs`)**:
  - Verifies fixed holidays, legacy and current Happy Mondays, Vernal/Autumnal Equinoxes, Emperors' Birthdays, substitute holidays, bridge national holidays, and year-specific overrides.
- **Built-In Config & Schema Validation (`src/holidays.rs`)**:
  - Validates that embedded `config/holidays.json` parses cleanly (`test_default_holidays_json_validity`) and confirms out-of-range months/days/weeks or malformed date strings are rejected (`test_validation_error_handling`).
- **Shared Crate `common_lib` Integration (`test_holidays_diff_logic`, `test_word_count_logic`)**:
  - Verifies line-by-line LCS diff computation and keyword occurrence counting.
- **Release Binary & NSIS Packaging Verification**:
  - Ensures `cargo build --release` (with `build.rs` / `winres` resource embedding) and `makensis installer/clondar.nsi` succeed without errors.

---

## 2. Test Execution Procedures

Run the following commands from the project root:

### 2.1 Unit Tests
```bash
cargo test
```

### 2.2 Pre-Commit Quality Pipeline
Run every check below with zero warnings and zero errors after modifying Rust source code:

```bash
# 1. Formatting check
cargo fmt --check

# 2. Static analysis / Clippy (warnings denied)
cargo clippy --all-targets -- -D warnings

# 3. Unit tests
cargo test

# 4. Rustdoc verification
cargo doc --no-deps --document-private-items

# 5. Release build check (recommended before tagging)
cargo build --release
```

### 2.3 NSIS Installer Verification (Windows)
```powershell
makensis installer\clondar.nsi
```

---

## 3. Test Maintenance Guidelines

1. **Updating Holiday Coverage**:
   - Whenever Japanese public holiday legislation changes, update `config/holidays.json` and add matching regression tests in `src/holidays.rs`.
2. **Edge Case Verification**:
   - Cover year boundaries, leap years, and consecutive Golden Week substitute holidays.
3. **Hermetic Test Isolation**:
   - Keep unit tests self-contained so they never mutate user `%LOCALAPPDATA%` directories during automated CI runs.
