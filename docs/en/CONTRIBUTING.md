# Contributing Guidelines (CONTRIBUTING.md) - Clondar

**English** | [日本語版](../ja/CONTRIBUTING.md)

Thank you for your interest in contributing to the `Clondar` project!
This document outlines guidelines for reporting bugs, suggesting features, and submitting pull requests.

---

## 1. Development Principles & Core Guidelines

Please adhere to the following principles when contributing to this project.

1. **Local-First / Offline Operation Guarantee**:
   - The application is designed to function entirely offline. Do not introduce CDNs or external asset URLs; bundle all assets locally using Vite.
2. **Visual Consistency (Borderless, Transparent, Shadowless)**:
   - Ensure you preserve the "ultimate minimalism" widget look. Keep borders, title bars, and OS window drop shadows completely removed.
3. **Multi-language Documentation Synchronization**:
   - When modifying specifications or introducing features, keep files under both `docs/ja/` and `docs/en/` updated in sync.

---

## 2. Environment Setup

Building and running the project requires **Node.js (v26.4.0+ recommended)** and **Rust (1.96.0+ recommended)**. If working on the shared library `common_lib` concurrently, configure local path overrides.

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/tkshnkgwr/clondar.git
   cd clondar
   ```
2. **Install Dependencies**:
   ```bash
   npm --prefix ui install
   ```
3. **Launch in Dev Mode**:
   ```bash
   cargo tauri dev
   ```

---

## 3. Commits & Pull Request Steps

### Commit Message Guidelines
Use the Conventional Commits format for your commit messages:

- `feat:` Introduces new features
- `fix:` Fixes bugs
- `docs:` Modifies documentation
- `refactor:` Refactors code structure
- `perf:` Optimizes performance
- `test:` Adds or modifies unit testing
- `chore:` Changes build configurations or automation scripts

### Pre-PR Checklist
Before submitting a pull request, run the following verification checks and ensure all pass without warnings:

- [ ] `cargo test` (Backend tests pass)
- [ ] `npm --prefix ui run test` (Frontend Vitest tests pass)
- [ ] `cargo clippy --all-targets -- -D warnings` (Clippy issues resolved)
- [ ] `cargo fmt --check` (Formatting rules applied)
- [ ] `cargo doc --no-deps --document-private-items` (Rustdoc compilation completes successfully)
