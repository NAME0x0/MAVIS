# Refactoring Plan: Remove Redundant `src` Directory

**Goal:** Remove the redundant top-level `src` directory and its contents, and update documentation accordingly.

**Analysis:** Comparison revealed that files in `src/` (`error.rs`, `lib.rs`, `main.rs`) are distinct from and likely older/redundant compared to the code within the `crates/` subdirectories (`crates/mavis-core`, `crates/mavis-shell`). The primary application logic appears to reside within these workspace crates.

**Steps:**

1.  **Delete Redundant Files:** Use Windows Batch commands (`del`) to delete the files identified as redundant within the `src` directory.
    *   `del src\error.rs`
    *   `del src\lib.rs`
    *   `del src\main.rs`
2.  **Delete Redundant Directory:** Use the Windows Batch command (`rmdir`) to remove the now-empty `src` directory.
    *   `rmdir src`
3.  **Verify Cleanup & Handle Other Empty Dirs (Optional but Recommended):**
    *   Check if any other directories became empty as a side effect.
    *   If found, either remove them (`rmdir`) or add a `.gitkeep` file (`echo. > path\to\dir\.gitkeep`) if the directory structure should be preserved.
4.  **Update Documentation (`README.md`):**
    *   Read the current content of `README.md`.
    *   Identify sections describing the project structure, specifically mentioning the `src/` directory.
    *   Modify these sections to accurately reflect the new structure (primarily using the `crates/` directory).
    *   Apply the changes to `README.md`.

**Execution Flow Diagram:**

```mermaid
graph TD
    A[Start: Confirmed Plan] --> B[1. Delete src/error.rs];
    B --> C[1. Delete src/lib.rs];
    C --> D[1. Delete src/main.rs];
    D --> E[2. Delete src/ directory];
    E --> F{Other Empty Dirs?};
    F -- Yes --> G[3. Handle Empty Dirs (Remove or .gitkeep)];
    F -- No --> H[4. Read README.md];
    G --> H;
    H --> I[4. Identify Structure Description];
    I --> J[4. Modify Description];
    J --> K[4. Apply Changes to README.md];
    K --> L[Implementation Complete];
```

**Next Step:** Switch to 'code' mode to execute these steps.