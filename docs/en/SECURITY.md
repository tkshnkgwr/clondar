# Security Policy (SECURITY.md) - Clondar

**English** | [日本語版](../ja/SECURITY.md)

This document outlines the security architecture, safety tenets, supported versions, and vulnerability reporting process for **Clondar Pro**.

---

## 1. Security Architecture & Principles

`Clondar Pro` (v2.0.0+) guarantees data isolation and safe local execution through a Pure Rust native architecture:

1. **Zero Browser Engine / Zero JS Runtime**:
   - Starting with v2.0.0, Clondar Pro eliminates WebView2, Chromium subprocesses, and Node.js/JavaScript dependencies entirely, removing browser/IPC attack surfaces.
2. **Rust-Enforced Memory Safety**:
   - All application layers (UI rendering, configuration parsing, system tray handling, and Japanese holiday calculations) are compiled in safe Rust, eliminating buffer overflows, null dereferences, and data races at compile time.
3. **Local-First & Offline Operations**:
   - Clondar Pro performs zero network communication. Preferences (`state.json`) and custom holiday configurations (`holidays.json`) stay strictly within `%LOCALAPPDATA%\com.clondar.pro\`, protected by Serde schema and date-range validation before writes.

---

## 2. Supported Versions

Security updates are provided for the following release branches:

| Version                            | Support Status |
| :--------------------------------- | :------------: |
| Latest Release (v2.0.x)            |  ✅ Supported  |
| Outdated Releases (v1.4.x & older) | ❌ Unsupported |

---

## 3. Reporting Vulnerabilities

If you discover a security vulnerability in `Clondar Pro`, please do not open a public GitHub issue. Contact the repository maintainer directly.

Please include:
1. Affected application version and Windows execution environment details.
2. A clear description of the issue and step-by-step reproduction instructions.

We will investigate promptly and ship a patch release upon confirmation.
