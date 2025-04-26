// MAVIS GUI - IDE Component (Scintilla Integration)

use crate::error::GuiError;
use imgui::Ui;
use log::{error, info, trace};
use std::ffi::CString;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, SendMessageW, SetWindowPos, ShowWindow, CW_USEDEFAULT,
    HWND_TOP, SW_HIDE, SWP_NOMOVE, WM_CLOSE, WS_CHILD, WS_CLIPSIBLINGS,
};
use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;

// Placeholder for Scintilla constants and functions via scintilla-sys
// use scintilla_sys::*;

// Placeholder for the main editor view struct
#[derive(Debug)]
pub struct EditorView {
    hwnd: HWND,
    // scintilla_ptr: *mut c_void, // Direct pointer if needed
    // texture_id: Option<imgui::TextureId>,
    // file_path: Option<String>,
    // dirty: bool,
    // Add other necessary fields
}

impl EditorView {
    /// Creates a new hidden Scintilla window.
    pub fn new(parent_hwnd: HWND) -> Result<Self, GuiError> {
        info!("Creating new Scintilla editor view");
        // TODO: Load Scintilla DLL if not already loaded
        // TODO: Register Scintilla window class if not already registered

        let class_name = CString::new("Scintilla").unwrap();
        let window_name = CString::new("").unwrap(); // No title needed for hidden window

        let hwnd = unsafe {
            CreateWindowExW(
                Default::default(), // No extended styles
                windows::core::PCWSTR(class_name.as_ptr() as *const u16), // Scintilla class name
                windows::core::PCWSTR(window_name.as_ptr() as *const u16), // Window name (empty)
                WS_CHILD | WS_CLIPSIBLINGS, // Basic styles for a child window
                CW_USEDEFAULT, // x
                CW_USEDEFAULT, // y
                100, // Initial width (will be resized)
                100, // Initial height (will be resized)
                parent_hwnd, // Parent window HWND
                None, // No menu
                None, // Instance handle (can often be null for child windows)
                None, // No additional creation data
            )
        };

        if hwnd.0 == 0 {
            error!("Failed to create Scintilla window");
            // Consider using windows::core::Error::from_win32() for better error info
            return Err(GuiError::InitializationError(
                "Failed to create Scintilla HWND".to_string(),
            ));
        }

        info!("Scintilla HWND created: {:?}", hwnd);
        // TODO: Perform initial Scintilla setup via SendMessageW
        // e.g., SCI_SETCODEPAGE, SCI_SETEOLMODE, etc.

        // Initially hide the window
        unsafe { ShowWindow(hwnd, SW_HIDE) };

        Ok(Self { hwnd })
    }

    /// Renders the editor content to an offscreen bitmap (Placeholder).
    pub fn render_to_bitmap(&self, _width: u32, _height: u32) -> Result<(), GuiError> {
        trace!("Rendering Scintilla HWND {:?} to bitmap", self.hwnd);
        // TODO: Implement GDI bitmap creation and WM_PRINTCLIENT logic
        Ok(())
    }

    /// Updates the ImGui texture with the rendered bitmap (Placeholder).
    pub fn update_texture(&mut self /* ... D3D device context ... */) -> Result<(), GuiError> {
        trace!("Updating ImGui texture for HWND {:?}", self.hwnd);
        // TODO: Implement GDI bitmap -> D3D texture conversion
        Ok(())
    }

    /// Handles input forwarding from ImGui to the Scintilla HWND (Placeholder).
    pub fn forward_input(&self, _ui: &Ui /* ... relevant input data ... */) {
        // TODO: Translate ImGui input to Win32 messages and SendMessageW to self.hwnd
    }

    /// Resizes the hidden Scintilla window.
    pub fn resize(&self, width: i32, height: i32) {
        trace!("Resizing Scintilla HWND {:?} to {}x{}", self.hwnd, width, height);
        unsafe {
            let _ = SetWindowPos(self.hwnd, HWND_TOP, 0, 0, width, height, SWP_NOMOVE);
        }
    }

    /// Sets keyboard focus to the Scintilla window.
    pub fn set_focus(&self) {
        trace!("Setting focus to Scintilla HWND {:?}", self.hwnd);
        unsafe {
            SetFocus(self.hwnd);
        }
    }

    // TODO: Add methods for loading/saving content, setting lexers, etc.
}

impl Drop for EditorView {
    fn drop(&mut self) {
        info!("Destroying Scintilla editor view (HWND: {:?})", self.hwnd);
        if self.hwnd.0 != 0 {
            // Send WM_CLOSE to allow Scintilla to clean up gracefully
            unsafe { SendMessageW(self.hwnd, WM_CLOSE, WPARAM(0), LPARAM(0)) };
            // Note: Actual window destruction might happen elsewhere or rely on parent closing
            self.hwnd = HWND(0); // Mark as invalid
        }
    }
}

// Placeholder for overall IDE state management struct
#[derive(Default, Debug)]
pub struct IdeState {
    pub editors: Vec<EditorView>,
    pub active_editor_index: Option<usize>,
    // Add other IDE-specific state
}

impl IdeState {
    pub fn new() -> Self {
        info!("Initializing IDE State");
        Self {
            editors: Vec::new(),
            active_editor_index: None,
        }
    }

    /// Renders the IDE panel using ImGui.
    pub fn draw(&mut self, ui: &Ui, parent_hwnd: HWND) {
        // Create a window using the correct imgui-rs API
        let window = imgui::Window::new("IDE Panel");
        window
            .size([800.0, 600.0], imgui::Condition::FirstUseEver)
            .build(ui, || {
                // TODO: Implement tab bar for open editors

                if let Some(index) = self.active_editor_index {
                    if let Some(editor) = self.editors.get_mut(index) {
                        let content_size = ui.content_region_avail();
                        let width = content_size[0].max(1.0) as i32;
                        let height = content_size[1].max(1.0) as i32;

                        // Resize hidden HWND if necessary
                        editor.resize(width, height);

                        // Render Scintilla to texture (if needed)
                        // editor.render_to_bitmap(width as u32, height as u32).ok();
                        // editor.update_texture(...).ok();

                        // Display texture
                        // if let Some(texture_id) = editor.texture_id {
                        //     ui.image(texture_id, content_size);
                        // } else {
                        ui.text_disabled("Editor rendering not implemented");
                        // }

                        // Handle input forwarding
                        // if ui.is_item_hovered() || ui.is_item_active() { // More complex focus check needed
                        //     editor.forward_input(ui);
                        //     if ui.is_item_clicked() {
                        //         editor.set_focus();
                        //     }
                        // }
                    }
                } else {
                    ui.text("No file open.");
                }

                // Placeholder for adding a new editor (for testing)
                if ui.button("Add Test Editor") {
                    match EditorView::new(parent_hwnd) {
                        Ok(editor) => {
                            self.editors.push(editor);
                            self.active_editor_index = Some(self.editors.len() - 1);
                        }
                        Err(e) => {
                            error!("Failed to create test editor: {}", e);
                        }
                    }
                }
            });
    }
}