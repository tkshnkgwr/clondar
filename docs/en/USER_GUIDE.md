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
  Click the pin icon (📌) on the top right of the calendar to pin the widget above other active windows. Click it again to unpin it.
- **Toggle Visibility**:  
  Click the close button (`❌`) on the top right of the widget or press the `Esc` key on your keyboard to hide the window. This does not terminate the app; you can restore the window from the system tray menu.

---

## 3. Clock & Calendar Customizations

### 3.1 Swapping Clock Modes
Click the toggle switch on the right of the clock (Analog/Digital icons) to swap styles.
- **Digital Clock**: Displays monospaced tabular layouts using a bold `Impact`-inspired font.
  - **12H/24H formats**: Double-click the clock numbers or use settings controllers to toggle formats.
  - **Seconds visibility**: Hide or show the seconds sub-clock to create a cleaner look.
- **Analog Clock**: Minimal sweep-second hand layout.

### 3.2 Calendar Grids
- **Monthly view**: Lock height structures to a steady **6-week (42-day)** grid layout to prevent height shifts across months. Public holidays are colored red; hover over them to view holiday labels inside tooltips.
- **Yearly overlay**: Click the calendar icon on the header to display the yearly viewer. Use the arrows to browse previous and next years.

---

## 4. Using the Holidays Manager

Click the gear icon (⚙️) or "**Holidays Config**" (or "**祝日設定**") button on the top right of the calendar to open the Holidays Manager dashboard.

### 4.1 Visual Holiday Editing
- **Adding Entries**:  
  Use the input form on the bottom left to define dates (Month and Day) and holiday names. Click "Add" to stage items.
- **Deleting Entries**:  
  Click the trash can icon on the right of lists to discard staged modifications.

### 4.2 Comparison Diffs (Default Comparison)
The central grid calculates deviations from built-in configurations (using LCS algorithms) and highlights diff lines:
- **Green background (+)**: Custom additions.
- **Red background (-)**: Excluded defaults.

### 4.3 Saving Custom Configurations
Click "**Save and Apply**" on the bottom right to apply modifications.
- Settings are stored securely inside the LocalAppData directory at `%LOCALAPPDATA%/com.clondar.pro/holidays.json`.
- Changes take effect on the calendar grid instantly and persist on relaunch.

---

## 5. System Tray Integrations

Clondar Pro resides inside your taskbar system tray menu. **Right-click** the tray icon to access controls:

- **Show / Hide**: Toggles window visibility.
- **Toggle Always on Top**: Anchor the widget above other windows (toggles checkmark status inside menus).
- **Reset Position**: If the widget gets lost off-screen due to screen changes or resolution updates, select this option to drag the window back to the center of the main display.
- **Exit**: Completely shuts down the application and removes it from the taskbar residency.
