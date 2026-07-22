# Security Policy (SECURITY.md) - Clondar

**English** | [日本語版](../ja/SECURITY.md)

This document outlines the security policies, design principles, and vulnerability reporting procedures for the `Clondar Pro` project.

---

## 1. Security Architecture & Principles

`Clondar Pro` is engineered with the following security tenets to guarantee data isolation and application reliability:

1. **Sandboxing & Permission Isolation (Tauri v2 Capabilities)**:
   - Frontend components (Webview) and Rust backend bindings are separated by Tauri's secure IPC bridge.
   - Using `capabilities/default.json`, we configure permission maps to restrict core operations, explicitly whitelisting only the necessary system APIs (e.g., window drag triggers, coordinates fetching, window closures, and pinning).
2. **Rust-Enforced Memory Safety**:
   - The backend program is compiled in Rust, which guarantees memory safety at compile-time, eliminating concerns regarding typical vulnerabilities (buffer overflows, null pointer dereferencing, race conditions, etc.).
3. **Local-First & Offline Operations**:
   - The application does not communicate with external network entities and operates completely offline. All parameters (e.g. customized holidays, preferences) are cached locally inside the user's LocalAppData folder, eliminating remote attack vectors.

---

## 2. Supported Versions

Security updates are actively supplied for the following release targets:

| Version | Support Status |
| :--- | :---: |
| Latest Release (v1.3.x) | ✅ Supported |
| Outdated Releases (v1.2.x & older) | ❌ Unsupported |

---

## 3. Reporting Vulnerabilities

If you identify a security vulnerability in `Clondar Pro`, do not open a public issue. Instead, report it directly to the repository maintainer or security contact via email.

Please provide the following details to help us investigate the issue:
1. The affected application version and details about your execution environment (e.g., Windows OS build).
2. A description of the vulnerability, along with step-by-step reproduction instructions (PoC code or operations).

Upon receipt, we will investigate the issue, develop patches, and release updates.
