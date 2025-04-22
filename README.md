# MAVIS (Modular AVIS) - Advanced Windows Shell Environment

**Version:** Pre-alpha (Specification v1.3 - April 22, 2025)
**Status:** Specification Phase / Early Development

[![Build Status](https://img.shields.io/badge/Build-Pending-lightgrey)](...)
[![Tests](https://img.shields.io/badge/Tests-Pending-lightgrey)](...) <!-- Placeholder -->
[![Coverage](https://img.shields.io/badge/Coverage-Pending-lightgrey)](...) <!-- Placeholder -->
[![License](https://img.shields.io/badge/License-MIT-blue)](LICENSE)
[![Latest Release](https://img.shields.io/badge/Release-Pre--alpha-orange)](...) <!-- Placeholder -->
[![Language](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-Dear%20ImGui%20(imgui--rs)%20+%20Direct2D-brightgreen)](https://github.com/imgui-rs/imgui-rs)
[![Scripting](https://img.shields.io/badge/Scripting-Lua%20(mlua)-blueviolet)](https://github.com/mlua-rs/mlua)
[![Windows Compat](https://img.shields.io/badge/Windows-10%20(19045+)%20%7C%2011%20(22621+)-blue)](...)

---

<!-- TOC -->
* [1. Overview & Vision](#1-overview--vision)
* [2. Core Requirements & Philosophy](#2-core-requirements--philosophy)
* [3. Key Features](#3-key-features)
* [4. Screenshots / Demo](#4-screenshots--demo)
* [5. System Architecture](#5-system-architecture)
* [6. Project Layout](#6-project-layout)
* [7. Component Deep Dive](#7-component-deep-dive)
    * [7.1 Shell Replacement](#71-shell-replacement)
    * [7.2 Window Manager & Taskbar](#72-window-manager--taskbar)
    * [7.3 GUI Framework](#73-gui-framework)
    * [7.4 Terminal Subsystem](#74-terminal-subsystem)
    * [7.5 IDE Component](#75-ide-component)
    * [7.6 Resource Monitoring](#76-resource-monitoring)
    * [7.7 Configuration Engine (Lua)](#77-configuration-engine-lua)
* [8. Performance & Optimization Strategies](#8-performance--optimization-strategies)
* [9. Compatibility](#9-compatibility)
    * [9.1 Supported Windows Versions](#91-supported-windows-versions)
    * [9.2 Hardware Requirements](#92-hardware-requirements)
* [10. Installation & Quickstart](#10-installation--quickstart)
    * [10.1 Installation](#101-installation)
    * [10.2 Build & Run (Development/Testing)](#102-build--run-developmenttesting)
    * [10.3 Initial Configuration](#103-initial-configuration)
* [11. Usage & Configuration](#11-usage--configuration)
    * [11.1 Configuration Examples](#111-configuration-examples)
* [12. Troubleshooting & FAQ](#12-troubleshooting--faq)
* [13. Development Roadmap](#13-development-roadmap)
* [14. Future Vision: AI Assistant Integration](#14-future-vision-ai-assistant-integration)
* [15. Testing](#15-testing)
* [16. Contributing](#16-contributing)
* [17. Changelog](#17-changelog)
* [18. Release History](#18-release-history)
* [19. Contact & Support](#19-contact--support)
* [20. License](#20-license)
* [21. Third-Party Licenses](#21-third-party-licenses)
* [22. Acknowledgements & Citations](#22-acknowledgements--citations)
<!-- /TOC -->

---

## 1. Overview & Vision

**MAVIS (Modular AVIS - A Virtual Intelligent Shell)** is an ambitious project aiming to create a modern, lightweight, and highly customizable shell replacement for Microsoft Windows. Built primarily in **Rust**, it leverages contemporary APIs and libraries to offer a performant, safe, and extensible desktop environment. It functions as a full-screen overlay environment, completely replacing the standard `explorer.exe` shell to provide a seamless, integrated workspace.

The core vision is to create a highly productive environment for power users and developers, combining essential tools like a terminal, file explorer, code editor, and system monitor into a unified, scriptable interface powered by **Lua**.

**Ultimate Goal (Future Vision):** A nod to "JARVIS," MAVIS aims to evolve beyond a shell replacement into an intelligent, AI-driven assistant. Future phases plan for the integration of Large Language Models (LLMs) for natural language interaction, task automation, and proactive system management, transforming the user's interaction with their Windows machine.

---

## 2. Core Requirements & Philosophy

* **Extreme Performance:** Achieve near-instantaneous response times and minimal CPU/RAM footprint through Rust, optimized algorithms, and GPU acceleration.
* **Low Resource Usage:** Target minimal idle resource consumption, significantly lower than the default Windows shell.
* **Deep Customizability:** Allow users to tailor every aspect of the interface and behavior via Lua scripting, including themes, widgets, keybindings, and workflows.
* **Stability & Reliability:** Implement robust error handling, fallback mechanisms, and rigorous testing to ensure system stability.
* **Modern Tooling Integration:** Seamlessly integrate essential developer tools like a terminal, file explorer, and code editor.
* **Memory Safety:** Leverage Rust's ownership model to prevent common memory-related bugs and vulnerabilities.
* **Modularity:** Design components to be as independent as possible, facilitating future extensions (hence "Modular AVIS").

---

## 3. Key Features

* **Full Shell Replacement:** Utilizes Shell Launcher v2 API or Registry modification for complete `explorer.exe` replacement. Includes automatic crash detection and fallback to `explorer.exe`.
* **Rust Core:** Foundation built entirely in Rust for safety and performance.
* **Lightweight GUI:** Powered by `imgui-rs` (Dear ImGui bindings) rendering via **Direct2D** for a fluid, GPU-accelerated UI. Includes **GDI fallback** for compatibility.
* **Integrated Terminal Subsystem:** Embeds a terminal using Windows **ConPTY** API and `Termion` (or similar) for ANSI/VT sequence handling.
* **Terminal-Based File Explorer:** Integrates **LF** (List Files) within the terminal, configured for previews and custom actions.
* **Embedded IDE Component:** Features **Scintilla** (via `scintilla-sys` bindings) for code editing with:
    * Syntax highlighting for 20+ languages (using precompiled SciLexer.dll v5.3.6+).
    * Performance limits for large files (>10MB).
    * Planned **Tree-sitter** integration for advanced parsing and potential autocompletion.
    * Basic debugging tools (breakpoints) with isolated process management.
* **Real-time Resource Monitoring:** Displays CPU, RAM, and Network statistics using low-overhead **Windows Performance Data Helper (PDH)** APIs via the `windows` crate. Includes customizable graphs and alerts.
* **Advanced Lua Scripting Engine:** Uses the `mlua` crate for Lua 5.4+ integration:
    * Extensive API for keybindings, widget creation, themes, and system interaction.
    * **Sandboxing** capabilities for script security (configurable).
    * **Hot Reloading** for configuration and theme files without restarting.
* **Customizable Theming:** Define UI appearance (colors, fonts, styles) using JSON or Lua theme files, supporting hot reload.
* **Window Management Helpers:** Provides basic functionalities for managing application windows within the MAVIS environment (details TBD).
* **Widget System:** Display various informational elements (clock, resource monitors, custom Lua widgets) in configurable areas.
* **(Planned) Plugin System:** Future support for extending MAVIS functionality via external DLL plugins.
* **(Planned) AI Integration:** Future phases target voice control (Whisper.cpp) and local LLM features (ONNX Runtime).

---

## 4. Screenshots / Demo

*(Placeholder: Add screenshots or GIFs demonstrating MAVIS in action here)*

---

## 5. System Architecture

MAVIS operates as a single, full-screen process that takes over rendering and primary interaction after `winlogon` initializes the shell.

```plaintext
+-------------------------------------------------------------------+
|                       MAVIS Process (Rust)                        |
| +---------------------------------------------------------------+ |
| |                 Window Manager (Win32 APIs)                   | |
| +---------------------------------------------------------------+ |
| |             GUI Layer (imgui-rs + Direct2D/GDI)               | |
| | +--------------+  +----------------+  +---------------------+ | |
| | | Taskbar/     |  | Widget Area    |  | Main Workspace      | | |
| | | System Tray  |  | (Lua Driven)   |  | (Panels: Term, IDE) | | |
| | +--------------+  +----------------+  +---------------------+ | |
| |        |                 |                     |              | |
| |        +-----------------+---------------------+              | |
| |                          |                                    | |
| | +------------------------V---------------------------------+  | |
| | | Lua Engine (mlua) <------------------> Scripting API     |  | |
| | +----------------------------------------------------------+  | |
| | | Core Modules                                             |  | |
| | | +-----------------------------------------------------+  |  | |
| | | | Terminal (ConPTY -> LF File Explorer)               |  |  | |
| | | +-----------------------------------------------------+  |  | |
| | | | IDE (Scintilla / Tree-sitter)                       |  |  | |
| | | +-----------------------------------------------------+  |  | |
| | | | Resource Monitor (PDH)                              |  |  | |
| | | +-----------------------------------------------------+  |  | |
| | | | Config/Theme Loader (Lua/JSON)                      |  |  | |
| | | +-----------------------------------------------------+  |  | |
| | +----------------------------------------------------------+  | |
| +---------------------------------------------------------------+ |
| |          Win32 / Windows API Layer (`windows` crate)          | |
| +---------------------------------------------------------------+ |
+-------------------------------------------------------------------+
                         | | (System Calls)
                         V V
+-------------------------------------------------------------------+
|                    Windows OS Kernel & APIs                       |
+-------------------------------------------------------------------+
```

Key interactions:

1.  **Shell Replacement:** Initiated by Windows `winlogon` via Shell Launcher v2 or Registry.
2.  **GUI:** `imgui-rs` draws the entire UI, rendered using Direct2D.
3.  **Modules:** Core components (Terminal, IDE, etc.) run within the main process, interacting via internal APIs.
4.  **Scripting:** Lua scripts interact with the Core Modules and GUI via the exposed `mlua` API.
5.  **System Calls:** Uses the `windows` crate for direct access to Win32/PDH/ConPTY APIs.

---

## 6. Project Layout

A clearer, workspace‑friendly layout separating source, crates, docs, and tooling.

```plaintext
mavis/                              # <root of workspace>
├── .github/                        # GitHub Actions workflows
│   └── workflows/
│       └── rust.yml
├── ci/                             # CI helper scripts (lint, release, version bump)
│   └── build.sh
├── docs/                           # User/developer docs
│   ├── architecture.md
│   ├── lua_api.md
│   └── SECURITY.md                 # security policy
├── scripts/                        # Utility scripts (gen-bindings, release, packaging)
│   └── gen_sentinel_bindings.rs
├── assets/                         # Bundled fonts, icons, themes
│   ├── fonts/
│   └── themes/
├── config/                         # Runtime config templates (copied to %LOCALAPPDATA%)
│   ├── init.lua
│   └── keybindings.lua
├── crates/                         # Internal workspace crates
│   ├── mavis-core/                 # core logic
│   ├── mavis-gui/                  # GUI/frontend
│   └── mavis-shell/                # shell‑replacement integration
├── examples/                       # Self‑contained demos and usage samples
│   └── example_widget.lua
├── src/                            # Source for the main MAVIS binary
│   └── main.rs
├── bin/                            # Source for additional binaries (e.g., installer tool)
│   └── installer.rs                # Example: source for an installer tool
├── benches/                        # benchmarks (criterion or built‑in)
│   └── cpu_cycle.rs
├── tests/                          # integration tests
│   └── shell_integration.rs
├── tools/                          # small internal tools (e.g., linting, codegen)
│   └── tidy-config.rs
├── Cargo.toml                      # workspace manifest (members: crates/*, ., bin/*)
├── Cargo.lock
├── CHANGELOG.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── CREDITS.md
├── LICENSE
└── README.md
```

---

## 7. Component Deep Dive

### 7.1 Shell Replacement

* **Primary Method:** Shell Launcher v2 API (Requires compatible Windows Editions - Enterprise/Education). Provides robust integration and UWP app compatibility. Configuration via WMI or XML. Ref: [MS Learn](https://learn.microsoft.com/en-us/windows/configuration/shell-launcher)
* **Fallback Method:** Modifying `HKLM\Software\Microsoft\Windows NT\CurrentVersion\Winlogon\Shell` registry key. Wider compatibility but potentially less stable and may have UWP issues.
* **Stability & Fallback:**
    * **Crash Detection:** Monitors own process health. If >2 crashes occur within 30 seconds of startup, automatically reverts the registry key (if used) to `explorer.exe` and triggers a reboot request.
    * **Safe Mode:** Detects Windows Safe Mode boot and automatically yields to `explorer.exe`.
    * **Manual Override:** Provide a documented key combination (e.g., `Ctrl+Alt+Shift+F4`) during boot (before UI loads) to force `explorer.exe`.
* **Edge Cases:**
    * *Third-Party Shell Hooks:* Installation checks for known incompatible software (e.g., StartIsBack, ExplorerPatcher) and warns the user. Runtime hooks may cause instability.
    * *User Profiles:* Configuration stored strictly within `%LOCALAPPDATA%\MAVIS` to ensure user isolation. Appropriate Directory ACLs applied.

### 7.2 Window Manager & Taskbar

* **Functionality:** Manages visibility and basic state (focus) of external application windows. Provides a taskbar for running applications and system tray icons.
* **Implementation:** Uses Win32 APIs (`EnumWindows`, `SetForegroundWindow`, `ShowWindow`, etc.) via the `windows` crate. Taskbar/tray drawn using `imgui-rs`. *Note: Full tiling/complex window management is a complex future goal.*

### 7.3 GUI Framework

* **Library:** `imgui-rs` (Rust bindings for Dear ImGui).
* **Rendering:** **Direct2D** via `windows` crate bindings for GPU acceleration.
    * Uses texture atlasing to batch draw calls for small UI elements.
    * Implements double-buffering for graph widgets to prevent flicker.
* **Fallback:** **GDI** rendering mode activated automatically if Direct2D initialization fails (e.g., driver issues, incompatible hardware). UI animations may be disabled in GDI mode.

### 7.4 Terminal Subsystem

* **API:** Windows Pseudo Console (ConPTY). Ref: [MS DevBlogs](https://devblogs.microsoft.com/commandline/windows-command-line-introducing-the-windows-pseudo-console-conpty/)
* **Integration:**
    * Uses `CreatePseudoConsole` with default 80x24 size, resizing via `ResizePseudoConsole`.
    * Input handled via `WriteFile` to the ConPTY input pipe (with `ENABLE_VIRTUAL_TERMINAL_INPUT`).
    * Output read via `ReadFile` from the ConPTY output pipe, buffered in 4KB chunks, parsed for ANSI sequences using `Termion` (or similar).
* **File Explorer:** **LF** (List Files) runs within the ConPTY instance.
    * *Configuration:* Pre-configured `lfrc` to disable default previews and potentially use custom keybinds (e.g., `Ctrl+P`).
    * *Preview Hook:* A mechanism (TBD - possibly stdout parsing or custom LF patch) sends the selected file path and MIME type (using `libmagic` bindings) back to MAVIS.
    * *Preview Display:* Text files rendered in the Scintilla component; Images rendered using `stb_image-rs` bindings; Other types show basic info or have configurable external viewers.
* **Edge Cases:**
    * *Network Drives (LF):* Directory read operations on UNC paths might time out briefly; LF's caching behavior helps mitigate this. Consider a small UI indicator for slow loads.
    * *File Locking (LF):* Operations like delete/rename might fail if file is locked. LF typically handles this; MAVIS logs errors if needed.
    * *High Throughput:* Ensure output buffer handling doesn't block the UI thread during commands generating large amounts of output (e.g., `find /`).

### 7.5 IDE Component

* **Editing Core:** **Scintilla** editing component. Accessed via C API using Rust FFI (likely via the `scintilla-sys` crate or a custom wrapper).
* **Syntax Highlighting:** Uses dynamically loaded **SciLexer.dll** (targeting v5.3.6+). Lexers pre-configured for common languages (Rust, Lua, Python, C++, Java, JS, TS, HTML, CSS, JSON, YAML, TOML, Markdown, Shell, etc.).
* **Performance:**
    * *Large Files:* Real-time lexing/parsing disabled for files > 10MB. File loads in a simplified "read-only" or basic highlighting mode. User confirmation needed for full editing.
    * *Autocomplete (Future - Tree-sitter):* Language server protocol (LSP) or Tree-sitter query execution capped at 100ms per interaction to prevent UI lag.
* **Debugging:**
    * *Breakpoints:* Stored in a simple SQLite database (`%LOCALAPPDATA%\MAVIS\debug.db`) linking file paths/hashes to line numbers.
    * *Process Isolation:* If integrating actual debuggers (future goal), use Windows Job Objects to manage and terminate potentially hung debugger processes safely.

### 7.6 Resource Monitoring

* **Data Source:** Windows Performance Data Helper (PDH) API via `windows` crate.
    * *CPU:* Monitors `\Processor(_Total)\% Processor Time` counter, sampled every 500ms.
    * *RAM:* Monitors `\Memory\Available MBytes` and calculates usage percentage. Per-process details via `GetProcessMemoryInfo`.
    * *Network:* Monitors `\Network Interface(*)\Bytes Total/sec` via PDH or uses `GetIfTable`/`GetTcpTable` for basic stats.
* **Rendering:** Uses `imgui-rs` widgets (`PlotLines` for graphs, custom widgets for gauges/text). Direct2D backend ensures smooth updates.
* **Alerts:** Configurable thresholds (via Lua) trigger system notifications (e.g., using `Shell_NotifyIcon`) when CPU usage > 90% for 60s, or available RAM < 5%.

### 7.7 Configuration Engine (Lua)

* **Engine:** `mlua` crate providing Lua 5.4+ bindings for Rust.
* **API Surface:** Exposes Rust functions to Lua for:
    * `bind_key(key_combo, function)`
    * `set_theme(theme_table)`
    * `add_widget(area, widget_config)`
    * `launch_app(path, args)`
    * `get_system_info(type)` (e.g., 'cpu_usage')
    * Filesystem operations (scoped/sandboxed)
    * Configuration value get/set.
* **Configuration Files:** Located in `%LOCALAPPDATA%\MAVIS\config\`. Main entry point `init.lua`.
* **Theming:** Theme definitions in `.json` (static) or `.lua` (dynamic) files in `%LOCALAPPDATA%\MAVIS\themes\`. JSON schema provided for validation.
* **Hot Reload:** Uses `ReadDirectoryChangesW` API to monitor config and theme directories. Changes trigger Lua state reload (where safe) or theme recompilation/application.
* **Sandboxing:** `mlua`'s sandbox features are used. By default, scripts have limited access (e.g., no arbitrary `io`, `os.execute`). A global `UNSAFE_MODE = true` setting (requires explicit user action, e.g., editing a specific file) can bypass restrictions for trusted scripts.

---

## 8. Performance & Optimization Strategies

* **Immediate Mode GUI:** `imgui-rs` inherently minimizes drawing overhead.
* **GPU Acceleration:** Direct2D rendering offloads UI drawing from the CPU.
* **Rust Optimizations:** Release builds (`--release`) with Link Time Optimization (LTO) enabled. Careful use of `async` for I/O bound tasks if applicable later.
* **Memory Management:** Rust's ownership model minimizes leaks. Use `Arc` for shared read-only data (e.g., themes) and `Mutex`/`RwLock` for shared mutable state where necessary. Avoid unnecessary allocations in render loops.
* **Lazy Loading:**
    * *IDE Components:* Scintilla/Tree-sitter initialized only when an editor panel is first opened.
    * *File Previews:* Preview generation triggered on demand, results potentially cached in memory for recently viewed files.
* **Lightweight Embeds:** Using ConPTY directly is more efficient than embedding a full third-party terminal emulator application.

---

## 9. Compatibility

### 9.1 Supported Windows Versions

| Version     | Tested Builds        | Minimum Required Build | Notes                                     |
| :---------- | :------------------- | :--------------------- | :---------------------------------------- |
| Windows 10  | `19045.4529+`        | `19041` (v2004)        | Shell Launcher v2 requires specific editions |
| Windows 11  | `22621.3527+`        | `22000` (Initial)      | Optimized for newer APIs if available    |

*Note: Builds are indicative based on API availability. Testing across various updates is required.*

### 9.2 Hardware Requirements

* **Minimum:**
    * CPU: 64-bit Dual-Core @ 1.8 GHz
    * RAM: 4 GB
    * GPU: DirectX 11 capable (Intel HD 520 / AMD Radeon R5 / Nvidia GeForce 800 series equivalent or newer) for Direct2D. GDI fallback available.
    * Storage: SSD recommended for configuration/cache read/write speed (approx. 100MB install size + user data).
* **Recommended:**
    * CPU: 64-bit Quad-Core @ 2.5 GHz+
    * RAM: 8 GB+ (16GB for heavy multitasking/development)
    * GPU: DirectX 12 capable (dedicated preferred, e.g., Nvidia GTX 1050 / AMD RX 560 or newer)
    * Storage: NVMe SSD

---

## 10. Installation & Quickstart

**⚠️ WARNING: Replacing your Windows shell is an advanced procedure that can lead to system instability or lockout if done incorrectly. Ensure you have recovery media (Windows Installation USB) and backup important data before proceeding.**

### 10.1 Installation

1.  **Download:** Obtain the latest MAVIS release `.zip` or installer (`.msi` - TBD) from the [Releases page](...) (Link TBD).
2.  **Extract/Install:** Extract the archive to a permanent location (e.g., `C:\Program Files\MAVIS`) or run the installer.
3.  **Configuration (Choose ONE method):**
    * **Method A: Shell Launcher v2 (Recommended - Requires Win Ent/Edu/IoT):**
        * Requires configuration via `PowerShell` (WMI) or `Provisioning Packages` (.ppkg).
        * Refer to specific documentation provided with MAVIS and Microsoft's Shell Launcher docs. This method needs setup *before* logging in as the target user.
    * **Method B: Registry Modification (Use with extreme caution):**
        * Run `regedit.exe` as Administrator.
        * Navigate to `HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Winlogon`.
        * **CRITICAL:** Backup the existing value of the `Shell` key (typically `explorer.exe`).
        * Modify the `Shell` value to the full path of `MAVIS.exe` (e.g., `C:\Program Files\MAVIS\MAVIS.exe`).
        * **To Revert:** Change the `Shell` value back to `explorer.exe`. See [Troubleshooting](#12-troubleshooting--faq).
4.  **Reboot/Relogin:** Sign out and sign back in, or restart your computer. MAVIS should now load instead of the standard desktop.

### 10.2 Build & Run (Development/Testing)

**Prerequisites**

- Rust 1.70+ (stable)
- Windows 10 SDK (build 19041+)
- PowerShell (for Shell Launcher v2 setup, if used)

**Build & Run**

```powershell
# Clone the repository
git clone https://github.com/NAME0x0/MAVIS.git
cd mavis

# Build the project in release mode
cargo build --release

# Run the executable (Note: This runs MAVIS as a normal app, not as the shell)
# For shell replacement, follow the steps in Section 10.1.
& .\target\release\mavis.exe
```

### 10.3 Initial Configuration
- On first launch (as a regular app or as the shell), default configuration files will be created in `%LOCALAPPDATA%\MAVIS\`.
- You can start customizing by editing files like `init.lua` and `keybindings.lua` in `%LOCALAPPDATA%\MAVIS\config\`.

---

## 11. Usage & Configuration

* **Initial Run:** On first launch, MAVIS will create default configuration files in `%LOCALAPPDATA%\MAVIS\`.
* **Configuration:** Primarily done by editing Lua scripts in `%LOCALAPPDATA%\MAVIS\config\`. The main file is `init.lua`.
* **Theming:** Place theme files (`.json` or `.lua`) in `%LOCALAPPDATA%\MAVIS\themes\` and select the theme in your Lua config.
* **Keybindings:** Defined in Lua using the `bind_key` function.
* **Hot Reload:** Saving changes to `.lua` or `.json` files in the config/themes directories should apply changes automatically where supported (e.g., themes, some widget settings). A manual reload function might be provided via Lua API or keybinding.

### 11.1 Configuration Examples

**Keybindings (`%LOCALAPPDATA%\MAVIS\config\keybindings.lua` or `init.lua`):**

```lua
-- Launch Windows Terminal with Win + Enter
bind_key("Win+Return", function() launch_app("wt.exe") end)

-- Request system shutdown with Win + Shift + Q
bind_key("Win+Shift+Q", function() request_shutdown() end)

-- Example: Show/hide a custom widget
bind_key("Ctrl+Alt+Space", function() toggle_widget("my_custom_widget") end)

-- Example: Set a theme
bind_key("Ctrl+Alt+T", function() set_theme("solarized_dark") end)
```

**Theme (`%LOCALAPPDATA%\MAVIS\themes\my_theme.json`):**

```json
{
  "name": "My Custom Theme",
  "colors": {
    "background": "#1e1e2e",
    "foreground": "#cdd6f4",
    "accent": "#89b4fa",
    "terminal_bg": "#11111b",
    "terminal_fg": "#bac2de",
    "widget_bg": "#313244",
    "widget_fg": "#cdd6f4"
  },
  "fonts": {
    "default": { "family": "JetBrainsMono Nerd Font", "size": 11.0 },
    "terminal": { "family": "Cascadia Code PL", "size": 10.0 }
  },
  "styles": {
    "window_padding": [8.0, 8.0],
    "item_spacing": [4.0, 4.0],
    "border_size": 1.0
  }
}
```

---

## 12. Troubleshooting & FAQ

*   **MAVIS fails to start / Stuck on black screen:**
    *   **Revert to Explorer:** Boot into Safe Mode (hold Shift while clicking Restart). Once in Safe Mode, run `regedit.exe`, navigate to `HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Winlogon`, and change the `Shell` value back to `explorer.exe`. Reboot normally.
    *   **Manual Override:** Try holding `Ctrl+Alt+Shift+F4` during the boot process (before the MAVIS UI would normally appear) to force `explorer.exe` for that session.
    *   **Check Logs:** Look for logs in `%LOCALAPPDATA%\MAVIS\logs\` (if implemented).
*   **Missing SciLexer.dll:** Ensure the correct version of `SciLexer.dll` is present in the MAVIS installation directory or a location specified in the configuration. Download it from the Scintilla website if needed.
*   **Configuration/Theme not loading:** Check Lua syntax in your config files. Ensure theme files are correctly formatted JSON or Lua. Check file paths.
*   **Performance Issues:** Ensure your hardware meets the minimum requirements. Check resource monitor for bottlenecks. Try disabling complex widgets or features via Lua config.

---

## 13. Development Roadmap

* **Phase 1 (Core Shell & Terminal):** Q3 2025 (Target)
    * Stable Shell Replacement implementation (Shell Launcher v2 + Registry).
    * Basic `imgui-rs` GUI framework with Direct2D rendering.
    * Functional ConPTY terminal integration with Termion (or similar).
    * LF integration within the terminal.
    * Initial taskbar implementation.
* **Phase 2 (IDE & Resources):** Q4 2025 (Target)
    * Scintilla integration for text/code editing.
    * PDH-based resource monitoring widgets (CPU, RAM).
    * Basic file preview system (Text via Scintilla, Images via stb_image).
* **Phase 3 (Scripting & Themes):** Q1 2026 (Target)
    * Develop comprehensive Lua API (`mlua`) for configuration.
    * Implement theme loading (JSON/Lua) and hot reloading.
    * Refine keybinding system with conflict resolution.
    * Implement basic Lua sandboxing.
* **Phase 4 (AI & Extensibility):** Q2 2026+ (Target)
    * **Plugin System:** Architecture for loading external DLLs/plugins.
    * **Cloud Sync:** Optional E2E encrypted config backup/sync (libsodium).
    * **Voice Control:** Integrate offline speech-to-text (e.g., Whisper.cpp).
    * **AI Features:** Explore local LLM inference (ONNX Runtime) for features like command suggestions or basic natural language processing. Define LLM interaction API.

---

## 14. Future Vision: AI Assistant Integration

Phase 4 represents the beginning of the transition towards an AI-powered shell. The long-term vision for MAVIS includes:

* **Natural Language Interface:** Process complex commands given via text or voice.
* **Contextual Awareness:** Understand user workflow, open applications, and file context.
* **Proactive Assistance:** Offer suggestions, manage notifications intelligently, automate routine tasks.
* **Deep System Interaction:** Control system settings, manage files/applications, query information based on natural language requests.
* **Security & Control:** Granular permissions for AI actions, user confirmation for sensitive operations, transparent processing.

---

## 15. Testing

MAVIS uses Rust's built-in testing framework.

```powershell
# Run all tests (unit and integration)
cargo test

# Run only unit tests (often faster)
# Unit tests might be marked with #[ignore] if they require specific setup
# or run `cargo test -- --show-ignored` to see them.
# Conventionally, run tests NOT marked as ignored:
cargo test -- --skip ignored
# Or run only tests marked as ignored (typically integration tests):
cargo test -- --ignored

# Run specific integration tests by name
cargo test --test <integration_test_filename_without_rs> -- --ignored

# Run tests with backtrace on failure
RUST_BACKTRACE=1 cargo test
```

*   **CI/CD:** GitHub Actions workflows (see `.github/workflows`) are planned to run tests automatically on pushes and pull requests. Badges for build status, test results, and code coverage (e.g., via Codecov) will be added at the top of this README.

---

## 16. Contributing

Contributions are welcome! Please read the following documents before contributing:

*   **`CONTRIBUTING.md`** (TBD): Guidelines for the development process, coding standards, issue reporting, and pull request submission.
*   **`CODE_OF_CONDUCT.md`** (TBD): Expected standards of behavior in the community.

**General Workflow:**

1.  Fork the repository.
2.  Create a feature branch (`git checkout -b feature/AmazingFeature`).
3.  Commit your changes (`git commit -m 'Add some AmazingFeature'`). Adhere to conventional commit messages (TBD).
4.  Push to the branch (`git push origin feature/AmazingFeature`).
5.  Open a Pull Request.

---

## 17. Changelog

See **`CHANGELOG.md`** (TBD) for a detailed history of changes in each release.

An **`## [Unreleased]`** section will be maintained in the changelog for upcoming changes.

---

## 18. Release History

*(Placeholder: This section will list past releases and their key highlights once available)*

*   **v0.1.0-alpha (TBD):** Initial pre-alpha release.

---

## 19. Contact & Support

*   **Bug Reports & Feature Requests:** Please use the [GitHub Issues](...) page (Link TBD). Check existing issues first. Use the provided templates (bug report, feature request).
*   **Questions & Discussion:** Use [GitHub Discussions](...) (Link TBD) or the project's Discord server (Link TBD).
*   **Security Vulnerabilities:** Please report security issues privately according to the (TBD) `SECURITY.md` policy.

---

## 20. License

MAVIS is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 21. Third-Party Licenses

MAVIS uses several third-party libraries and assets. While the core code is MIT licensed, some dependencies or bundled assets (like fonts or icons) may have different licenses. A summary of these licenses can be found in **`THIRD_PARTY_LICENSES.md`** (TBD).

---

## 22. Acknowledgements & Citations

MAVIS stands on the shoulders of giants. We extend our gratitude to the developers and communities behind:

* **Rust Language:** [rust-lang.org](https://www.rust-lang.org/)
* **Dear ImGui:** [github.com/ocornut/imgui](https://github.com/ocornut/imgui)
* **`imgui-rs`:** [github.com/imgui-rs/imgui-rs](https://github.com/imgui-rs/imgui-rs)
* **`mlua`:** [github.com/mlua-rs/mlua](https://github.com/mlua-rs/mlua)
* **`windows-rs`:** [github.com/microsoft/windows-rs](https://github.com/microsoft/windows-rs)
* **`termion` / `crossterm`:** Terminal libraries for Rust.
* **`lf` File Manager:** [github.com/gokcehan/lf](https://github.com/gokcehan/lf)
* **Scintilla:** [scintilla.org](https://www.scintilla.org/)
* **Tree-sitter:** [tree-sitter.github.io](https://tree-sitter.github.io/tree-sitter/)
* **Microsoft:** Documentation for Win32, PDH, ConPTY, Shell Launcher APIs.

*(Specific versions and further library credits will be maintained in a dedicated `CREDITS` or `Cargo.toml` file).*

---

*This README reflects the MAVIS project specification v1.3 as of April 22, 2025. Details are subject to change during development.*
