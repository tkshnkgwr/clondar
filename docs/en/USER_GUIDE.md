# Clondar Pro User Guide

**English** | [日本語版](../ja/USER_GUIDE.md)

Thank you for choosing Clondar Pro.  
This guide provides detailed instructions on installing the application, basic operations, customizing the clocks and calendars, and using the "Holidays Manager".

---

## 1. Installation & Booting

### 1.1 Requirements
- **OS**: Windows 10 / 11 (64bit)
- **Dependency**: Microsoft Edge WebView2 Runtime  
  *Note: Installed by default on most modern Windows systems. If missing, a download wizard automatically launches during startup.*

### 1.2 Installation Steps
1. Double-click the installer package (`app_x.x.x_x64_en-US.msi` or `app-setup.exe`).
2. When the language prompts appear (日本語 / English), select "English".
3. Follow the installation wizard directives to choose target paths and complete setup.
4. Launch **Clondar** from the desktop shortcut or your Start menu.

---

## 2. Interface Layout & Basic Controls

Clondar Pro functions as a transparent desktop widget blending natively into your wallpaper.

- **Dragging/Repositioning**:  
  Click and drag anywhere on the widget surface (e.g. transparent background spaces where no text displays) to reposition it on the screen.
- **Set Always on Top (Pinning)**:  
  Click the pin icon (📌) on the title bar to pin the widget above other active windows. Click it again to unpin it.
- **Toggle Transparency**:  
  Click the sun icon (☀️) to toggle background blur and glassmorphism transparency.
- **Help & Information (❓)**:  
  Click the help icon (❓) on the title bar to display an overlay modal showing application details, current version, and feature guidelines.
- **Toggle Visibility**:  
  Click the close button (`❌`) on the top right of the widget or press the `Esc` key on your keyboard to hide the window (if a modal is open, `Esc` closes the modal first). This does not terminate the app; you can restore the window from the system tray menu.

---

## 3. Clock & Calendar Customizations

### 3.1 Swapping Clock Modes
Click the tab buttons above the clock (`DIGITAL` / `ANALOG`) to swap styles.
- **Digital Clock**: Displays monospaced tabular layouts using a bold `Impact`-inspired font.
  - **12H/24H formats**: Click the "`24H` / `12H`" button below the clock to toggle between 12-hour (with AM/PM) and 24-hour formats.
  - **Seconds visibility**: Click the "`秒表示: ON / OFF`" button below the clock to show or hide the seconds indicator.
  - **Dark / Light Theme**: Click the sun/moon icon button below the clock to switch between dark and light themes instantly.
- **Analog Clock**: Minimal sweep-second hand layout.

### 3.2 Calendar Grids
- **Monthly view**: Locks height structures to a steady **6-week (42-day)** grid layout to prevent height shifts across months. Public holidays are colored red; hover over them to view holiday labels inside tooltips. Click "**今月**" (This Month) to jump back to the current month.
- **Yearly overlay**: Click the "**今年**" (This Year) button on the calendar header to display the full-screen yearly calendar modal. Use the arrows to browse previous and next years.

---

## 4. Using the Holidays Manager

Click the "**祝日設定**" (Holidays Config) button on the top right of the calendar to open the Holidays Manager dashboard.

### 4.1 Visual Holiday Editing & Raw Data Editing
- **Adding Entries ("祝日の編集" / Edit Tab)**:  
  Use the input form on the right side to define the date (`MM-DD` format, e.g., `07-07`) and holiday name, then click "リストに追加" (Add to List) to stage the item.
- **Deleting Entries**:  
  Click the trash can icon on the right of any item in the fixed holidays list to remove it.
- **Raw Config Data ("設定生データ" / Raw Tab)**:  
  Directly edit the underlying JSON configuration text. Syntax and schema validations are automatically performed when switching tabs or saving.

### 4.2 Comparison Diffs & Statistics
- **Comparison Diffs ("変更差分 (Diff)" Tab)**: Compares your active configuration with the built-in default configuration (using LCS algorithms) and highlights line-by-line differences:
  - **Green highlight (+)**: Newly added lines.
  - **Red highlight (-)**: Deleted default lines.
- **Configuration Statistics**: The left sidebar automatically counts and displays occurrences of key terms (such as "の日" or "天皇誕生日").

### 4.3 Saving & Resetting Configurations
- **Save and Apply ("保存して適用")**: Click the "**保存して適用**" button at the top right to validate and save your changes to `%LOCALAPPDATA%/com.clondar.pro/holidays.json`. Changes take effect on the calendar grid immediately and persist across restarts.
- **Reset ("初期化")**: Click the "**初期化**" button at the top right to safely restore the built-in default holiday configuration at any time.

---

## 5. System Tray Integrations

Clondar Pro resides inside your taskbar system tray menu. **Right-click** the tray icon to access controls:

- **Show / Hide ("表示 / 非表示")**: Toggles window visibility.
- **Toggle Always on Top ("最前面表示の切替")**: Anchors the widget above other windows (synchronizes immediately with the pin icon on the window title bar).
- **Reset Position ("位置をリセット")**: If the widget gets lost off-screen due to monitor changes or resolution updates, select this option to center the window on the primary display immediately.
- **Exit ("終了")**: Completely shuts down the application and removes it from the system tray.
