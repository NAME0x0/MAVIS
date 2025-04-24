# MAVIS Implementation Plan - Phase 1

This document outlines the initial implementation plan for Phase 1 of the MAVIS project, based on the analysis of `README.md` (Specification v1.3).

## README Analysis Summary (Relevant to Implementation)

*   **Project Structure:** The README defines a workspace layout (Section 6) which the current project structure generally follows. Key crates: `mavis-core`, `mavis-gui`, `mavis-shell`.
*   **Build Process:** Use `cargo build --release` (Section 10.2). Requires Rust 1.70+ and Windows 10 SDK (19041+).
*   **Core Technologies:** Rust, `imgui-rs` (Direct2D/GDI), `mlua` (Lua), ConPTY, `lf`, Scintilla, PDH.
*   **Development Roadmap:** Phase 1 focuses on Core Shell & Terminal (Section 13).
*   **Configuration:** Lua scripts in `%LOCALAPPDATA%\MAVIS\config\`, entry point `init.lua` (Section 7.7, 11).

## Proposed Plan: Phase 1 - Foundational Elements

The initial focus will be on establishing the core structure and foundational components required for Phase 1:

1.  **Verify Workspace Setup:** Ensure the root `Cargo.toml` correctly defines the workspace members (`mavis-core`, `mavis-gui`, `mavis-shell`, the main binary) as per Section 6.
2.  **Establish Core Binary:** Set up the basic structure in `src/main.rs` to initialize logging, configuration loading stubs, and the main application loop structure.
3.  **Initialize GUI Foundation:** Begin setting up the `mavis-gui` crate, initializing `imgui-rs` and the Direct2D backend (with GDI fallback) as described in Section 7.3.
4.  **Basic Shell Integration Stub:** Create placeholder functions or structures in `mavis-shell` related to shell replacement logic (Section 7.1).

## Plan Visualization

```mermaid
graph TD
    A[Analyze README] --> B(Verify Project Structure);
    B --> C{Propose Plan: Focus on Phase 1};
    C --> D(Verify Workspace Setup - Cargo.toml);
    C --> E(Establish Core Binary - src/main.rs);
    C --> F(Initialize GUI Foundation - mavis-gui);
    C --> G(Basic Shell Integration Stub - mavis-shell);