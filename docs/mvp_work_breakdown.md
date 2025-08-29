# MAVIS MVP Work Breakdown

**Goal:** Transform current codebase into a working Phase 1 MVP  
**Current Status:** 60-70% complete  
**Target Completion:** 6-8 weeks  

## Critical Path Tasks

### 1. Build System Fixes (Priority: Critical, Effort: 1-2 days)

**Current Issue:** Project doesn't compile due to Linux dependencies in a Windows-only project.

**Required Changes:**
- Remove or make optional Linux-specific dependencies (fontconfig, etc.)
- Add Windows-specific feature flags
- Update Cargo.toml with proper platform targets
- Test compilation on Windows environment

**Files to Modify:**
- `Cargo.toml` (workspace and individual crates)
- CI configuration files
- Build scripts

### 2. GUI Rendering Implementation (Priority: Critical, Effort: 1-2 weeks)

**Current State:** Placeholder ImGui integration without actual rendering.

**Required Changes:**

#### 2a. Direct2D Backend Implementation
- Complete `crates/mavis-gui/src/renderer.rs` implementation
- Add Direct2D initialization and context management
- Implement ImGui renderer for Direct2D
- Add GDI fallback renderer

#### 2b. Window Integration
- Fix `crates/mavis-gui/src/window.rs` to properly initialize ImGui
- Add proper platform integration with Winit
- Implement event handling and input processing
- Connect renderer to main event loop

#### 2c. Basic UI Components
- Enhance `crates/mavis-gui/src/ui.rs` with functional widgets
- Implement basic taskbar functionality
- Add window management UI
- Connect resource monitoring display

**Files to Create/Modify:**
- `crates/mavis-gui/src/renderer.rs` - Complete implementation
- `crates/mavis-gui/src/window.rs` - Fix platform integration
- `crates/mavis-gui/src/ui.rs` - Enhance UI components
- `crates/mavis-gui/src/widgets/taskbar.rs` - New taskbar component

### 3. Shell Registration Implementation (Priority: Critical, Effort: 1 week)

**Current State:** Placeholder functions with no actual Windows integration.

**Required Changes:**

#### 3a. Registry Manipulation
- Implement Windows registry reading/writing in `ShellManager`
- Add Shell Launcher v2 API integration
- Implement shell registration and unregistration

#### 3b. Crash Detection and Recovery
- Complete crash monitoring logic
- Implement startup timing and crash counting
- Add automatic recovery mechanisms
- Implement reboot request functionality

#### 3c. Safe Mode Detection
- Add Windows Safe Mode detection using `GetSystemMetrics`
- Implement manual override key detection using `GetAsyncKeyState`

**Files to Modify:**
- `crates/mavis-shell/src/lib.rs` - Complete all TODO items
- Add new Windows API helper modules

**TODOs to Complete:**
```rust
// TODO: Implement actual initialization logic
// TODO: Implement actual key state checking (e.g., GetAsyncKeyState)
// TODO: Implement logic for Shell Launcher v2 or Registry modification.
// TODO: Implement logic to revert Shell Launcher v2 or Registry changes.
// TODO: Implement Safe Mode detection (e.g., check GetSystemMetrics SM_CLEANBOOT).
// TODO: Implement crash detection logic (e.g., tracking startup times, crash counts).
// TODO: Implement reboot request (e.g., InitiateSystemShutdownEx).
```

### 4. Terminal and LF Integration (Priority: High, Effort: 1 week)

**Current State:** Basic ConPTY session management without LF integration.

**Required Changes:**

#### 4a. LF Integration
- Add LF binary detection and management
- Configure LF with appropriate settings for MAVIS
- Implement file preview integration
- Add LF output parsing and display

#### 4b. Terminal UI Implementation
- Create terminal widget for GUI
- Implement ANSI escape sequence parsing
- Add terminal input handling
- Connect ConPTY output to terminal display

#### 4c. File Management Features
- Implement file preview system
- Add basic file operations through LF
- Connect file selection to preview system

**Files to Create/Modify:**
- `crates/mavis-gui/src/widgets/terminal.rs` - New terminal widget
- `crates/mavis-core/src/conpty/mod.rs` - Enhance LF integration
- `crates/mavis-core/src/file_preview.rs` - New file preview system

## Secondary Priority Tasks

### 5. Lua API Implementation (Priority: Medium, Effort: 1 week)

**Current State:** Framework exists but many functions are placeholders.

**Required Changes:**
- Complete keybinding registration logic
- Implement theme system functionality
- Add widget manipulation APIs
- Complete system information APIs

**Files to Modify:**
- `crates/mavis-core/src/lua/api/keybindings.rs`
- `crates/mavis-core/src/lua/api/theme.rs`
- `crates/mavis-core/src/lua/api/widgets.rs`
- `crates/mavis-core/src/lua/api/system.rs`

### 6. Taskbar Implementation (Priority: Medium, Effort: 1 week)

**Current State:** Not implemented.

**Required Changes:**
- Create basic taskbar component
- Implement window management
- Add system tray integration
- Basic application launcher

**Files to Create:**
- `crates/mavis-gui/src/widgets/taskbar.rs`
- `crates/mavis-gui/src/window_manager.rs`

## Testing and Integration Tasks

### 7. End-to-End Testing (Priority: Medium, Effort: 1 week)

**Required Tasks:**
- Integration testing of all components
- Windows compatibility testing
- Performance optimization
- Bug fixing and polish

### 8. Documentation Updates (Priority: Low, Effort: 2-3 days)

**Required Tasks:**
- Update installation instructions
- Add troubleshooting guide
- Update build documentation
- Create user guide

## Risk Assessment

### High Risk Items
1. **Direct2D Integration Complexity:** May require significant Windows graphics expertise
2. **Shell Registration Stability:** Critical for system stability, requires careful testing
3. **ConPTY/LF Integration:** Potential compatibility issues between components

### Mitigation Strategies
1. **Start with GDI fallback** for initial GUI implementation
2. **Implement robust fallback mechanisms** for shell registration
3. **Gradual integration testing** for terminal components

## Resource Requirements

### Technical Skills Needed
- Windows API expertise (Shell, Registry, Graphics)
- ImGui/Direct2D rendering knowledge
- ConPTY and terminal integration experience
- Rust systems programming experience

### Development Environment
- Windows 10/11 development machine
- Windows SDK 19041+
- Rust 1.70+ toolchain
- LF binary for testing

## Success Criteria for MVP

1. ✅ **Project compiles and runs on Windows**
2. ✅ **Basic GUI with functional widgets**
3. ✅ **Shell replacement functionality works**
4. ✅ **Terminal with LF integration functional**
5. ✅ **Configuration system working**
6. ✅ **Resource monitoring displayed in GUI**
7. ✅ **Basic taskbar implementation**

## Timeline Estimate

```
Week 1: Build fixes + GUI foundation
Week 2: GUI rendering completion
Week 3: Shell registration implementation
Week 4: Terminal/LF integration
Week 5: Lua API completion + Taskbar
Week 6: Testing and integration
Week 7-8: Polish and documentation
```

This breakdown provides a clear path from the current 60-70% completion state to a functional MVP that meets all Phase 1 requirements.