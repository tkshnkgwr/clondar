# Contributing Guidelines (CONTRIBUTING.md) - Clondar

**English** | [日本語版](../ja/CONTRIBUTING.md)

Thank you for your interest in contributing to the `Clondar` project!  
This document outlines guidelines for reporting bugs, proposing features, and submitting pull requests.

---

## 1. Development Principles & Core Guidelines

Please adhere to the following principles when contributing:

1. **Local-First / Offline Operation Guarantee**:
   - Clondar Pro is engineered to run completely offline. Do not introduce network calls, telemetry, or external asset dependencies.
2. **Visual Consistency (Borderless, Transparent, Shadowless)**:
   - Preserve the minimalist desktop widget aesthetic without OS window frames or drop shadows, and render UI icons via vector primitives in `src/icons.rs` rather than OS emoji fonts.
3. **Multi-language Documentation Synchronization**:
   - Keep corresponding files under both `docs/ja/` and `docs/en/` synchronized whenever modifying specifications or features.

---

## 2. Environment Setup

Building and running `Clondar Pro` requires **Rust (1.96.0+ recommended, Windows MSVC target)** and the sibling `common_lib` repository:

1. **Clone Repositories Side-by-Side**:
   ```bash
   git clone https://github.com/tkshnkgwr/clondar.git
   git clone https://github.com/tkshnkgwr/common_lib.git
   cd clondar
   ```
2. **Launch in Development Mode**:
   ```bash
   cargo run
   ```

---

## 3. Commits & Pull Request Steps

### Commit Message Guidelines
Use clear prefixes (e.g., Conventional Commits format):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation update
- `refactor:` Code refactoring
- `perf:` Performance or memory optimization
- `test:` Adding or updating unit tests
- `chore:` Build, CI, or tooling updates

### Pre-PR Checklist
Verify all checks pass cleanly with zero warnings before submitting a pull request:

- [ ] `cargo fmt --check` (Formatting compliance)
- [ ] `cargo clippy --all-targets -- -D warnings` (Clippy static analysis)
- [ ] `cargo test` (Unit test suite)
- [ ] `cargo doc --no-deps --document-private-items` (Rustdoc compilation)
