# MAVIS: Scintilla IDE Component Integration Plan (Phase 2)

This document outlines the plan for integrating the Scintilla text editing component into the MAVIS GUI, based on the requirements in `README.md` (Section 7.5) and subsequent planning discussions.

**Target:** Implement a basic code editor panel within the `mavis-gui` crate using Scintilla, rendered via an offscreen texture approach within the ImGui UI.

## 1. Dependencies

*   **Add Scintilla Binding:** Add a suitable Scintilla binding crate (e.g., `scintilla-sys` or a higher-level wrapper) to `crates/mavis-gui/Cargo.toml`.
*   **SciLexer.dll:** Determine the strategy for providing `SciLexer.dll`. For initial implementation, bundle the DLL with the application build artifacts. Ensure it's accessible at runtime.

## 2. Create IDE Module (`crates/mavis-gui/src/ide/`)

*   Create a new module: `crates/mavis-gui/src/ide/mod.rs`.
*   Define core structs (e.g., `EditorView`, `EditorState`) to manage individual editor instances and their state (HWND, texture ID, file path, content, dirty status).
*   Implement FFI wrappers using the chosen binding crate to interact with Scintilla:
    *   Initialization (loading SciLexer).
    *   Creating hidden Scintilla HWNDs.
    *   Setting/getting text content.
    *   Configuring basic properties (theme colors, fonts via Scintilla messages).
    *   Setting lexers based on file type.
    *   Handling notifications from Scintilla (e.g., `SCN_UPDATEUI`, `SCN_SAVEPOINTREACHED`, `SCN_SAVEPOINTLEFT`).

## 3. Large File Handling

*   Implement logic within the IDE module (e.g., when opening a file) to check the file size.
*   For files exceeding a configurable threshold (e.g., 10MB), load Scintilla in a simplified mode:
    *   Disable automatic lexing (`SCI_SETLEXER(SCLEX_NULL)`).
    *   Potentially disable complex features or load read-only initially.
    *   Consider user notification/confirmation for attempting to load large files fully.

## 4. GUI Integration (Offscreen Rendering)

This uses an offscreen rendering approach to integrate the native Scintilla control with the ImGui UI.

### 4.1. Scintilla HWND Management

*   For each `EditorView`, create a corresponding native Scintilla Win32 control as a *hidden* window.
*   The `ide/mod.rs` module manages the lifecycle and holds references to these HWNDs.

### 4.2. Rendering Scintilla to a Bitmap

*   Implement `EditorView::render_to_bitmap(&self, width: u32, height: u32) -> Result<BitmapHandle, GuiError>`.
*   Use GDI operations: Create a memory DC and DIB section (bitmap).
*   Send `WM_PRINTCLIENT` to the Scintilla HWND to draw onto the memory DC/Bitmap.

### 4.3. Bitmap to ImGui Texture Conversion

*   Implement logic (e.g., in `mavis-gui/src/renderer/texture_utils.rs`) to convert the GDI bitmap to an `imgui::TextureId`.
*   Access raw bitmap pixel data.
*   Create/update a Direct3D 11 texture (`ID3D11Texture2D`) and Shader Resource View (`ID3D11ShaderResourceView`).
*   Copy pixel data (handle potential format conversion, e.g., BGRA).
*   The `EditorView` stores the resulting `imgui::TextureId`.

### 4.4. Displaying Texture in ImGui (`ui.rs`)

*   In the UI rendering logic for the IDE panel:
    *   Get available region size (`ui.content_region_avail()`).
    *   If size changed: Resize hidden Scintilla HWND (`SetWindowPos`) and recreate rendering resources (bitmap, texture).
    *   Trigger `render_to_bitmap` and texture conversion if needed (e.g., content changed, resize).
    *   Use `ui.image(editor_view.texture_id, [width, height])` to draw the texture.

### 4.5. Input Forwarding

*   In `ui.rs`, within the IDE panel rendering:
    *   Use `ui.is_item_hovered()`, `ui.is_item_active()`, `ui.io()` to capture mouse/keyboard events over the `ui.image` area.
    *   Translate ImGui input (coordinates, keys) to Win32 messages (`WM_MOUSEMOVE`, `WM_LBUTTONDOWN`, `WM_KEYDOWN`, etc.). Perform coordinate mapping.
    *   Send translated messages to the hidden Scintilla HWND (`SendMessage`).
    *   Manage keyboard focus: On click within the image, call `SetFocus` on the Scintilla HWND.

### 4.6. Performance Considerations

*   Optimize texture updates: Only re-render/update texture when Scintilla signals changes (`SCN_UPDATEUI`) or relevant input occurs.

## 5. State Management (`mavis-gui/src/state.rs`)

*   Extend the main `GuiState` struct:
    *   `editors: Vec<EditorState>`: List of states for open editor instances.
    *   `active_editor_index: Option<usize>`: Index of the currently focused editor.
*   The `EditorState` within `ide/mod.rs` should contain file path, dirty status, Scintilla document pointer, etc.

## 6. Basic Functionality

*   Implement UI actions (menus, buttons, keybindings via Lua later) for:
    *   File -> Open: Show file dialog, load content into a new `EditorView`/`EditorState`.
    *   File -> Save: Write content of active `EditorView` back to its file path. Update dirty status.
    *   File -> Close: Remove the `EditorView`/`EditorState`. Handle unsaved changes.
    *   Tab switching logic in the UI.

## 7. Unit Tests

*   Add tests in `crates/mavis-gui/src/ide/mod.rs`:
    *   Verify Scintilla initialization and DLL loading.
    *   Test basic text manipulation wrappers (set/get text).
    *   Test file size checking logic.

## Architecture Diagram Update (Conceptual)

```mermaid
graph TD
    A[mavis-shell/main.rs] --> B(mavis-gui/run_gui);
    B --> C{GUI Event Loop (winit)};
    C --> D[mavis-gui/ui.rs];
    D -- Renders Panels --> E(ImGui UI);
    D -- Uses --> F[mavis-gui/state.rs];
    D -- Uses --> G[mavis-gui/ide/mod.rs];
    G -- Manages --> H(Hidden Scintilla HWNDs);
    G -- Uses --> I(Scintilla FFI / scintilla-sys);
    G -- Interacts with --> J(SciLexer.dll);
    G -- Renders To --> K(Offscreen Bitmap);
    K -- Converted To --> L[D3D Texture (TextureId)];
    E -- Displays --> L;
    E -- Forwards Input --> G;
    F -- Manages --> M(Editor State - Open Files, etc.);

    subgraph mavis-gui crate
        direction LR
        B; C; D; E; F; G; H; I; J; K; L; M;
    end

    subgraph mavis-core crate
        N(Core Logic - File I/O etc.)
    end

    subgraph External
        O(SciLexer.dll)
    end

    G --> O;
    G --> N; // For file operations