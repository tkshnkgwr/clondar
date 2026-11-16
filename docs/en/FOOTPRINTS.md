# Footprints (FOOTPRINTS.md) - Clondar

**English** | [日本語版](../ja/FOOTPRINTS.md)

This document records binary sizes, real-hardware RAM/CPU benchmarks, and release compilation optimizations for **Clondar Pro v2.0.0** (Pure Rust / `egui + glow` edition).

---

## 1. Measurement Environment

- **OS**: Windows 11 64-bit
- **Toolchain**: Rust stable (`x86_64-pc-windows-msvc`), NSIS (`makensis`)
- **Target Assets**:
  - Application Executable: `target/release/Clondar.exe`
  - Installer Package: `installer/Clondar_Pro_2.0.0_x64-setup.exe`

---

## 2. Footprint Records

### 2.1 Binary & Package Sizes (Verified on v2.0.0 Build)

| Build Type / Asset Name                  |           Size            | Notes                                             |
| :--------------------------------------- | :-----------------------: | :------------------------------------------------ |
| `Clondar.exe` (Release)                  | **4.89 MB** (5,125,120 B) | LTO, `opt-level = 'z'`, stripped, embedded icon   |
| `Clondar_Pro_2.0.0_x64-setup.exe` (NSIS) | **2.59 MB** (2,714,117 B) | Bilingual JA/EN NSIS installer (zlib compression) |

> **Highlights**: By adopting `egui + glow` (OpenGL only) with zero WebView2 dependency, the standalone executable weighs only **4.89 MB** and the installer **2.59 MB** (~3 MB smaller than the `wgpu` prototype binary at `7.91 MB`).

---

### 2.2 Live Simultaneous Memory & CPU Benchmark (2026-11-16 16:38:13)

Real Windows desktop measurements comparing legacy Tauri/WebView2 builds against Phase C (`wgpu` vs. `glow`):

| Build / Architecture                | Processes | Private Memory |  WorkingSet   | Cumulative CPU Time |
| :---------------------------------- | :-------: | :------------: | :-----------: | :------------------ |
| **Legacy (`v1.4.1`, WebView2)**     |     7     |   207.51 MB    |   192.17 MB   | 34.30 s (~100 min)  |
| **Phase B (`v1.4.4`, Active View)** |     7     |   190.74 MB    |   364.81 MB   | 25.03 s (~35 min)   |
| **Phase C Proto (`egui + wgpu`)**   |     1     |   293.06 MB    |   349.54 MB   | 6.17 s (~2 min)     |
| **Phase C Adopted (`egui + glow`)** |   **1**   |  **68.45 MB**  | **115.76 MB** | **1.78 s** (~2 min) |

* **vs. `wgpu` Prototype**: **`293.06 MB` → `68.45 MB`** (**`-224.61 MB` / -76.6% reduction**)
* **vs. Phase B WebView2 (Active)**: **`190.74 MB` → `68.45 MB`** (**`-122.29 MB` / -64.1% reduction**)
* **vs. Legacy `v1.4.1`**: **`-139.06 MB`** (**-67.0% reduction**)
* **Root Cause Eliminated**: `eframe 0.37.0`'s default `wgpu` feature loads D3D12, Vulkan, and shader compiler DLLs (`igd12um64xel.dll`, `igvk64.dll`, `dxcompiler.dll`, etc.) consuming >200 MB of driver allocations. Explicitly selecting **`glow` (OpenGL only)** removes all of them completely.

---

## 3. Release Compilation Optimization Configuration (`Cargo.toml`)

```toml
[profile.release]
opt-level = 'z'     # Optimize for binary size
lto = true          # Enable Link-Time Optimization
codegen-units = 1   # Maximize cross-crate optimization
panic = 'abort'     # Omit unwinding tables
strip = true        # Strip symbol tables and debug entries
```
