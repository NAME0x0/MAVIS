# MAVIS (Modular AVIS) - Advanced Windows Shell Environment

**Version:** Pre-alpha (Specification v1.3 - April 22, 2025)
**Status:** Specification Phase / Early Development

[![Build Status](https://img.shields.io/badge/Build-Pending-lightgrey)](...)
[![License](https://img.shields.io/badge/License-MIT-blue)](LICENSE)
[![Language](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-Dear%20ImGui%20(imgui--rs)%20+%20Direct2D-brightgreen)](https://github.com/imgui-rs/imgui-rs)
[![Scripting](https://img.shields.io/badge/Scripting-Lua%20(mlua)-blueviolet)](https://github.com/mlua-rs/mlua)
[![Windows Compat](https://img.shields.io/badge/Windows-10%20(19045+)%20%7C%2011%20(22621+)-blue)](...)

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

## 4. System Architecture

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

## 5. Recommended Project Directory Structure

This structure follows standard Rust project conventions while accommodating MAVIS's specific needs like configuration, themes, and potential plugins.

```plaintext
mavis/
├── .cargo/                # Cargo configuration (optional, e.g., config.toml for build profiles)
├── .github/               # GitHub specific files (e.g., workflows for CI/CD)
│   └── workflows/
│       └── rust.yml       # Example CI workflow
├── assets/                # Static assets (fonts, default icons, default themes)
│   ├── fonts/
│   │   └── default_font.ttf
│   ├── icons/
│   └── themes/
│       ├── default_dark.json
│       └── default_light.json
├── config/                # Default configuration files copied to user dir on first run
│   ├── init.lua           # Default main Lua config script
│   └── keybindings.lua    # Example default keybindings
├── crates/                # Optional: For organizing internal workspace crates (if project becomes complex)
│   └── mavis-core/        # Example core logic crate
│   └── mavis-gui/         # Example GUI logic crate
├── docs/                  # Project documentation (user guides, technical specs)
│   ├── architecture.md
│   └── lua_api.md
├── examples/              # Example Lua scripts or usage scenarios
├── src/                   # Main source code directory
│   ├── api/               # Modules defining the Lua API
│   ├── components/        # Core components (terminal, ide, resources, etc.)
│   ├── config/            # Config loading and management logic
│   ├── gui/               # UI rendering logic (imgui setup, widgets)
│   ├── shell/             # Shell replacement logic (integration, fallback)
│   ├── utils/             # Utility functions and helpers
│   ├── main.rs            # Main application entry point
│   └── lib.rs             # Library entry point (if structured as a library + binary)
├── target/                # Build artifacts (created by Cargo, usually gitignored)
├── tests/                 # Integration tests
├── .gitignore             # Specifies intentionally untracked files that Git should ignore
├── Cargo.lock             # Records exact dependency versions used in a build
├── Cargo.toml             # Main project manifest (metadata, dependencies)
├── LICENSE                # Project license file (e.g., LICENSE-MIT)
└── README.md              # This file: High-level project overview
```

**Key Points:**

*   **`Cargo.toml`**: The heart of the Rust project, defining dependencies and metadata.
*   **`src/`**: Contains all the Rust source code. Organizing into submodules (`gui`, `components`, `api`, etc.) is crucial for maintainability.
*   **`assets/`**: Stores default assets shipped with the application.
*   **`config/`**: Contains default configuration files that might be copied to a user-specific directory (like `%LOCALAPPDATA%\MAVIS`) on first run.
*   **`crates/`**: Useful for larger projects to break down functionality into smaller, manageable internal libraries (workspace).
*   **`docs/`**: Essential for documenting architecture, APIs, and usage.
*   **`.gitignore`**: Crucial for keeping the repository clean (e.g., ignoring `target/`).

---

## 6. Component Deep Dive

### 6.1 Shell Replacement

* **Primary Method:** Shell Launcher v2 API (Requires compatible Windows Editions - Enterprise/Education). Provides robust integration and UWP app compatibility. Configuration via WMI or XML. Ref: [MS Learn](https://learn.microsoft.com/en-us/windows/configuration/shell-launcher)
* **Fallback Method:** Modifying `HKLM\Software\Microsoft\Windows NT\CurrentVersion\Winlogon\Shell` registry key. Wider compatibility but potentially less stable and may have UWP issues.
* **Stability & Fallback:**
    * **Crash Detection:** Monitors own process health. If >2 crashes occur within 30 seconds of startup, automatically reverts the registry key (if used) to `explorer.exe` and triggers a reboot request.
    * **Safe Mode:** Detects Windows Safe Mode boot and automatically yields to `explorer.exe`.
    * **Manual Override:** Provide a documented key combination (e.g., `Ctrl+Alt+Shift+F4`) during boot (before UI loads) to force `explorer.exe`.
* **Edge Cases:**
    * *Third-Party Shell Hooks:* Installation checks for known incompatible software (e.g., StartIsBack, ExplorerPatcher) and warns the user. Runtime hooks may cause instability.
    * *User Profiles:* Configuration stored strictly within `%LOCALAPPDATA%\MAVIS` to ensure user isolation. Appropriate Directory ACLs applied.

### 6.2 Window Manager & Taskbar

* **Functionality:** Manages visibility and basic state (focus) of external application windows. Provides a taskbar for running applications and system tray icons.
* **Implementation:** Uses Win32 APIs (`EnumWindows`, `SetForegroundWindow`, `ShowWindow`, etc.) via the `windows` crate. Taskbar/tray drawn using `imgui-rs`. *Note: Full tiling/complex window management is a complex future goal.*

### 6.3 GUI Framework

* **Library:** `imgui-rs` (Rust bindings for Dear ImGui).
* **Rendering:** **Direct2D** via `windows` crate bindings for GPU acceleration.
    * Uses texture atlasing to batch draw calls for small UI elements.
    * Implements double-buffering for graph widgets to prevent flicker.
* **Fallback:** **GDI** rendering mode activated automatically if Direct2D initialization fails (e.g., driver issues, incompatible hardware). UI animations may be disabled in GDI mode.

### 6.4 Terminal Subsystem

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

### 6.5 IDE Component

* **Editing Core:** **Scintilla** editing component. Accessed via C API using Rust FFI (likely via the `scintilla-sys` crate or a custom wrapper).
* **Syntax Highlighting:** Uses dynamically loaded **SciLexer.dll** (targeting v5.3.6+). Lexers pre-configured for common languages (Rust, Lua, Python, C++, Java, JS, TS, HTML, CSS, JSON, YAML, TOML, Markdown, Shell, etc.).
* **Performance:**
    * *Large Files:* Real-time lexing/parsing disabled for files > 10MB. File loads in a simplified "read-only" or basic highlighting mode. User confirmation needed for full editing.
    * *Autocomplete (Future - Tree-sitter):* Language server protocol (LSP) or Tree-sitter query execution capped at 100ms per interaction to prevent UI lag.
* **Debugging:**
    * *Breakpoints:* Stored in a simple SQLite database (`%LOCALAPPDATA%\MAVIS\debug.db`) linking file paths/hashes to line numbers.
    * *Process Isolation:* If integrating actual debuggers (future goal), use Windows Job Objects to manage and terminate potentially hung debugger processes safely.

### 6.6 Resource Monitoring

* **Data Source:** Windows Performance Data Helper (PDH) API via `windows` crate.
    * *CPU:* Monitors `\Processor(_Total)\% Processor Time` counter, sampled every 500ms.
    * *RAM:* Monitors `\Memory\Available MBytes` and calculates usage percentage. Per-process details via `GetProcessMemoryInfo`.
    * *Network:* Monitors `\Network Interface(*)\Bytes Total/sec` via PDH or uses `GetIfTable`/`GetTcpTable` for basic stats.
* **Rendering:** Uses `imgui-rs` widgets (`PlotLines` for graphs, custom widgets for gauges/text). Direct2D backend ensures smooth updates.
* **Alerts:** Configurable thresholds (via Lua) trigger system notifications (e.g., using `Shell_NotifyIcon`) when CPU usage > 90% for 60s, or available RAM < 5%.

### 6.7 Configuration Engine (Lua)

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

## 7. Performance & Optimization Strategies

* **Immediate Mode GUI:** `imgui-rs` inherently minimizes drawing overhead.
* **GPU Acceleration:** Direct2D rendering offloads UI drawing from the CPU.
* **Rust Optimizations:** Release builds (`--release`) with Link Time Optimization (LTO) enabled. Careful use of `async` for I/O bound tasks if applicable later.
* **Memory Management:** Rust's ownership model minimizes leaks. Use `Arc` for shared read-only data (e.g., themes) and `Mutex`/`RwLock` for shared mutable state where necessary. Avoid unnecessary allocations in render loops.
* **Lazy Loading:**
    * *IDE Components:* Scintilla/Tree-sitter initialized only when an editor panel is first opened.
    * *File Previews:* Preview generation triggered on demand, results potentially cached in memory for recently viewed files.
* **Lightweight Embeds:** Using ConPTY directly is more efficient than embedding a full third-party terminal emulator application.

---

## 8. Compatibility

### 8.1 Supported Windows Versions

| Version     | Tested Builds        | Minimum Required Build | Notes                                     |
| :---------- | :------------------- | :--------------------- | :---------------------------------------- |
| Windows 10  | `19045.4529+`        | `19041` (v2004)        | Shell Launcher v2 requires specific editions |
| Windows 11  | `22621.3527+`        | `22000` (Initial)      | Optimized for newer APIs if available    |

*Note: Builds are indicative based on API availability. Testing across various updates is required.*

### 8.2 Hardware Requirements

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

## 9. Installation & Setup

**⚠️ WARNING: Replacing your Windows shell is an advanced procedure that can lead to system instability or lockout if done incorrectly. Ensure you have recovery media (Windows Installation USB) and backup important data before proceeding.**

1.  **Download:** Obtain the latest MAVIS release `.zip` or installer (`.msi` - TBD) from the Releases page.
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
        * **To Revert:** Change the `Shell` value back to `explorer.exe`.
4.  **Reboot/Relogin:** Sign out and sign back in, or restart your computer. MAVIS should now load instead of the standard desktop.

---

## 10. Getting Started / Quickstart

**Prerequisites**

- Rust 1.70+ (stable)
- Windows 10 SDK (build 19041+)
- PowerShell (for Shell Launcher v2 setup, if used)

**Build & Run (Development/Testing)**

```powershell
# Clone the repository
git clone https://github.com/NAME0x0/MAVIS.git # Replace with actual repo URL
cd mavis

# Build the project in release mode
cargo build --release

# Run the executable (Note: This runs MAVIS as a normal app, not as the shell)
# For shell replacement, follow the steps in Section 9.
& .\target\release\mavis.exe
```

**Initial Configuration**
- On first launch (as a regular app or as the shell), default configuration files will be created in `%LOCALAPPDATA%\MAVIS\`.
- You can start customizing by editing files like `init.lua` and `keybindings.lua` in `%LOCALAPPDATA%\MAVIS\config\`.

---

## 11. Usage & Configuration

* **Initial Run:** On first launch, MAVIS will create default configuration files in `%LOCALAPPDATA%\MAVIS\`.
* **Configuration:** Primarily done by editing Lua scripts in `%LOCALAPPDATA%\MAVIS\config\`. The main file is `init.lua`.
* **Theming:** Place theme files (`.json` or `.lua`) in `%LOCALAPPDATA%\MAVIS\themes\` and select the theme in your Lua config.
* **Keybindings:** Defined in Lua using the `bind_key` function. Example:

    ```lua
    -- In init.lua or a dedicated keybindings.lua file
    bind_key("Win+Return", function() launch_app("wt.exe") end) -- Launch Windows Terminal
    bind_key("Win+Shift+Q", function() request_shutdown() end) -- Example system action
    bind_key("Ctrl+Alt+Space", function() show_widget("resource_monitor") end)
    ```

* **Hot Reload:** Saving changes to `.lua` or `.json` files in the config/themes directories should apply changes automatically where supported (e.g., themes, some widget settings). A manual reload function might be provided via Lua API or keybinding.

---

## 12. Development Roadmap

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

## 13. Future Vision: AI Assistant Integration

Phase 4 represents the beginning of the transition towards an AI-powered shell. The long-term vision for MAVIS includes:

* **Natural Language Interface:** Process complex commands given via text or voice.
* **Contextual Awareness:** Understand user workflow, open applications, and file context.
* **Proactive Assistance:** Offer suggestions, manage notifications intelligently, automate routine tasks.
* **Deep System Interaction:** Control system settings, manage files/applications, query information based on natural language requests.
* **Security & Control:** Granular permissions for AI actions, user confirmation for sensitive operations, transparent processing.

---

## 14. Validation & Testing Strategy

* **Unit Testing:** Rust tests for core logic, API handlers, configuration parsing.
* **Integration Testing:** Testing interactions between modules (Terminal <-> LF <-> Previewer, Lua API <-> Core).
* **Manual Testing:** Rigorous testing across supported Windows versions and diverse hardware. Focus on stability, performance, and edge cases defined in the specification.
* **Test Cases:** Specific test procedures for critical functions (e.g., Shell fallback, large file handling, resource leak detection).
* **Traceability:** Link requirements from the specification document to design decisions, code implementation, and test cases.

---

## 15. Contributing

Contributions are highly encouraged! Please read `CONTRIBUTING.md` (TBD) for details on the development process, coding standards, issue reporting, and pull request submission for MAVIS.

---

## 16. License

MAVIS is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 17. Acknowledgements & Citations

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

*This README reflects the MAVIS project specification v1.2 as of April 21, 2025. Details are subject to change during development.*
