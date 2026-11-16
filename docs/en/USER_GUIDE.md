# Clondar Pro User Guide

**English** | [日本語版](../ja/USER_GUIDE.md)

Thank you for choosing **Clondar Pro v2.0.0**.  
This guide covers installation, basic widget controls, clock & calendar customization, and the built-in Holidays Manager.

---

## 1. Installation & Startup

### 1.1 System Requirements
- **OS**: Windows 10 / 11 (64-bit)
- **External Runtimes**: None required (Pure Rust native binary — no WebView2 Runtime or Node.js required)

### 1.2 Installation Steps
1. Double-click the setup executable (`Clondar_Pro_2.0.0_x64-setup.exe`).
2. Select your installer language (**English** or **日本語**).
3. Follow the NSIS setup wizard (default location: `%LOCALAPPDATA%\Clondar Pro\Clondar.exe`).
4. Launch **Clondar Pro** from the Start Menu.

*(You can also place and run the standalone portable `Clondar.exe` binary directly.)*

---

## 2. Interface Layout & Basic Controls

Clondar Pro runs as a lightweight, borderless widget blending cleanly onto your desktop:

- **Drag to Move**:  
  Click and drag the title bar or background area to place the widget anywhere. Physical coordinates are saved automatically to `%LOCALAPPDATA%\com.clondar.pro\state.json`.
- **Always on Top (Pin)**:  
  Click the pin icon on the top-right title bar to keep the widget pinned above other windows.
- **Background Transparency**:  
  Click the sun/transparency icon on the title bar or bottom controls to toggle transparent background mode.
- **Help & App Info (`?`)**:  
  Click the help (`?`) icon on the title bar to view application info, version badge, and quick feature cards.
- **Hide to System Tray**:  
  Click the close (`✕`) icon or press `Esc` (if a modal is open, `Esc` closes the modal first) to minimize/hide the window to the system tray without terminating the app.

---

## 3. Clock & Calendar Customizations

### 3.1 Switching Clock Modes
Use the centered `DIGITAL` / `ANALOG` tabs above the clock:
- **Digital Clock**: Displays bold, high-contrast tabular numbers without horizontal jitter.
  - **12H / 24H Format**: Click the "`24H` / `12H`" button below the clock to switch between 24-hour and 12-hour (AM/PM) formats. *(Disabled while Analog mode is active.)*
  - **Seconds Toggle**: Click "`秒表示: ON / OFF`" to show or hide seconds.
  - **Dark / Light Theme**: Click the theme icon button below the clock to switch between Dark and Light color palettes.
- **Analog Clock**: Displays an enlarged circular analog clock with smooth sweep second-hand motion.

### 3.2 Calendar Navigation
- **Monthly View**: Locked to a **6-week (42-day)** grid layout so height remains fixed across all months. Japanese public holidays are highlighted in red with hover tooltips. Click "**今月**" (This Month) to jump back to today's month.
- **Yearly View**: Click "**今年**" (This Year) in the calendar header to open a full-canvas 12-month overview modal with previous/next year controls.

---

## 4. Using the Holidays Manager

Click "**祝日設定**" (Holidays Config) in the top-right calendar header to open the Holidays Manager modal.

### 4.1 Visual Editing & Raw JSON Editing
- **Visual Editor**: Enter a date (`MM-DD`, e.g. `07-07`) and holiday name, then click add to append a custom fixed holiday, or delete existing items in one click.
- **Raw Config Data (JSON)**: Directly inspect or edit the complete JSON rules (including Happy Mondays and year-specific overrides). Schema and date range validation run automatically before saving.

### 4.2 Comparison Diffs & Statistics
- **Diff Tab**: Compares your active rules against built-in defaults (`config/holidays.json`) using LCS line diffing (highlighted additions/deletions).
- **Statistics Bar**: Counts occurrences of key holiday types across your configuration.

### 4.3 Saving & Resetting
- **Save & Apply**: Validates and saves rules to `%LOCALAPPDATA%\com.clondar.pro\holidays.json`, updating the calendar immediately.
- **Reset**: Restores the built-in default Japanese holiday rules safely at any time.

---

## 5. System Tray Menu

**Right-click** the Clondar Pro tray icon in the Windows notification area to access:

- **Show / Hide**: Toggles widget visibility.
- **Toggle Always on Top**: Toggles pinned state in sync with the title bar button.
- **Reset Position**: Recenters the widget on the screen if moved off-display after monitor changes.
- **Exit**: Saves settings and terminates Clondar Pro cleanly.
