# MAVIS Implementation Plan

Based on the analysis of `README.md` and the current project state, this plan outlines the steps to implement the core features of the MAVIS project.

## High-Level Plan

```mermaid
graph TD
    A[Start: Analyze README & Current State] --> B{Define Core Crates};
    B --> C[Implement mavis-core];
    C --> D[Implement mavis-gui];
    C --> E[Implement mavis-shell];
    C --> F[Implement Main Binary (src/main.rs)];
    D --> F;
    E --> F;
    F --> G{Integrate Components};
    G --> H[Implement Terminal (ConPTY + LF)];
    G --> I[Implement IDE (Scintilla)];
    G --> J[Implement Resource Monitor (PDH)];
    G --> K[Implement Lua Engine (mlua)];
    H & I & J & K --> L[Refine Configuration & Theming];
    L --> M[Add Basic Tests];
    M --> N[End: Functional Core based on README];

    subgraph "Phase 1: Foundation (crates & main)"
        B; C; D; E; F; G;
    end

    subgraph "Phase 2: Core Features"
        H; I; J; K; L;
    end

    subgraph "Phase 3: Testing & Refinement"
        M; N;
    end
```

## Detailed Steps

1.  **`mavis-core` Enhancement:**
    *   Flesh out the `error.rs` module.
    *   Implement the Lua API surface as described in section 7.7 (bindings for keys, themes, widgets, system info, etc.).
    *   Implement robust configuration loading (`loader.rs`) and watching (`watcher.rs`) using `mlua`.
    *   Implement the resource monitoring logic using PDH APIs (`monitor/` modules).
    *   Add utility functions (`utils/`) as needed for inter-component communication or system interaction.
2.  **`mavis-gui` Implementation:**
    *   Set up the main GUI loop using `imgui-rs` and a suitable backend (e.g., `imgui-dx11-renderer` or `imgui-windows-d2d-renderer` if available/created, falling back to GDI).
    *   Implement the main window structure (Taskbar, Widget Area, Main Workspace).
    *   Create basic widgets (Clock, placeholders for resource monitors).
    *   Integrate theme loading/application based on data from `mavis-core`.
3.  **`mavis-shell` Implementation:**
    *   Implement logic for shell replacement (Registry method initially, Shell Launcher v2 later).
    *   Add crash detection and fallback mechanisms.
    *   Handle safe mode detection.
    *   Implement the manual override key combination.
4.  **`src/main.rs` Implementation:**
    *   Initialize logging, configuration, and core components.
    *   Set up the main application loop, integrating `mavis-core` and `mavis-gui`.
    *   Handle command-line arguments (if any).
    *   Manage the lifecycle of the application and its components.
    *   Integrate the shell replacement logic from `mavis-shell`.
5.  **Terminal Integration (within `mavis-gui` or a dedicated module):**
    *   Use `mavis-core` utilities to interact with the ConPTY API.
    *   Embed a terminal widget using `imgui-rs`.
    *   Parse ANSI sequences from ConPTY output.
    *   Integrate LF: Launch LF within the ConPTY, potentially configure it via `mavis-core`. Implement preview mechanism.
6.  **IDE Integration (within `mavis-gui` or a dedicated module):**
    *   Add `scintilla-sys` or a wrapper crate as a dependency.
    *   Ensure `SciLexer.dll` is bundled or findable.
    *   Create an `imgui-rs` widget to host the Scintilla control.
    *   Configure basic Scintilla settings (lexers, styles).
7.  **Resource Monitor Integration (within `mavis-gui`):**
    *   Create specific `imgui-rs` widgets (graphs, text displays) to show data fetched from `mavis-core`'s monitoring module.
8.  **Lua Engine Integration (within `mavis-core` & `mavis-gui`):**
    *   Ensure the Lua API in `mavis-core` is correctly exposed and callable.
    *   Trigger Lua functions based on events (e.g., keybindings from `mavis-gui`).
    *   Allow Lua scripts to interact with GUI elements (e.g., creating/updating widgets).
    *   Implement hot-reloading logic triggered by file changes detected in `mavis-core`.
9.  **Configuration & Theming:**
    *   Ensure default `config/init.lua`, `config/keybindings.lua`, and potentially theme files are created in `%LOCALAPPDATA%\MAVIS` on first run.
    *   Verify theme application and hot-reloading work correctly.
10. **Testing:**
    *   Add basic unit tests for core logic in `mavis-core` (e.g., config parsing).
    *   Add integration tests in `tests/` to verify component interactions (e.g., Lua calling a core function).