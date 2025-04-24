# MAVIS - Final Implementation Plan (Phase 1 Focus)

This plan outlines the steps to implement the initial phase of the MAVIS project, incorporating refactoring cleanup, documentation updates, and foundational code setup based on `README.md` (v1.3), `docs/implementation_plan.md`, and `docs/implementation_plan_phase1.md`.

**Context:**

*   The redundant top-level `src/` directory has already been removed.
*   `crates/mavis-shell/src/main.rs` is assumed to be the main application entry point.

**Plan Steps:**

1.  **Update `README.md`:**
    *   Modify Section 6 (Project Layout) to remove references to the deleted `src/` directory.
    *   Add details about the `.msi` installer target in Section 10.1 (Installation).
2.  **Add Placeholders:**
    *   Create empty `.gitkeep` files in `assets/fonts/` and `assets/themes/` to ensure Git tracks these directories.
3.  **Generate Documentation:**
    *   Create initial content for the following files based on common templates and project specifics:
        *   `CONTRIBUTING.md`
        *   `CODE_OF_CONDUCT.md`
        *   `CHANGELOG.md`
        *   `THIRD_PARTY_LICENSES.md`
4.  **Phase 1 Implementation Steps (Initial Code Setup):**
    *   Verify `Cargo.toml` workspace members are correctly defined (`mavis-core`, `mavis-gui`, `mavis-shell`, potentially the main binary if separate).
    *   Establish basic structure in `crates/mavis-shell/src/main.rs` (e.g., initialize logging, configuration loading stubs, main application loop placeholder).
    *   Initialize basic GUI setup in `crates/mavis-gui/src/lib.rs` or a dedicated `ui.rs` module (initialize `imgui-rs`, select/setup rendering backend - Direct2D preferred, GDI fallback).
    *   Create placeholder functions or structures in `crates/mavis-shell/src/lib.rs` related to shell integration logic (e.g., registry interaction, crash detection stubs).

**Plan Visualization:**

```mermaid
graph TD
    A[Start: src/ deleted] --> B[1. Update README.md (Layout & MSI)];
    B --> C[2. Add .gitkeep to assets/fonts & assets/themes];
    C --> D[3. Generate Content for MD Files (CONTRIBUTING, COC, CHANGELOG, etc.)];
    D --> E[4. Verify Cargo.toml Workspace];
    E --> F[4. Setup mavis-shell/src/main.rs];
    F --> G[4. Initialize mavis-gui Foundation];
    G --> H[4. Create mavis-shell Stubs];
    H --> I[End Plan: Ready for Implementation];

    subgraph "Documentation & Setup"
        B; C; D;
    end

    subgraph "Phase 1 Code Foundation"
        E; F; G; H;
    end
```

**Next Step:** Switch to 'code' mode to execute these steps.