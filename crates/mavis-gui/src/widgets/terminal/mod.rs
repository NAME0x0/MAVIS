// Terminal widget for MAVIS GUI

use imgui::Ui;
use log::{debug, info};
use std::sync::{Arc, Mutex};
use mavis_core::ConPtySession;

/// Terminal widget state
#[derive(Debug, Default)]
pub struct TerminalWidgetState {
    // Terminal buffer storage
    buffer: Vec<String>,
    // Terminal cursor position
    cursor_x: usize,
    cursor_y: usize,
    // Input buffer for commands
    input_buffer: String,
    // Terminal size
    width: usize,
    height: usize,
}

impl TerminalWidgetState {
    /// Create a new terminal widget state
    pub fn new() -> Self {
        Self {
            buffer: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            input_buffer: String::with_capacity(256),
            width: 80,
            height: 25,
        }
    }

    /// Process output data from ConPTY
    pub fn process_output(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        // Simple implementation: just convert bytes to UTF-8 string and add to buffer
        if let Ok(text) = String::from_utf8(data.to_vec()) {
            // Very basic terminal emulation - just append to last line or split on newlines
            for c in text.chars() {
                match c {
                    '\n' => {
                        self.cursor_y += 1;
                        self.cursor_x = 0;
                        if self.cursor_y >= self.buffer.len() {
                            self.buffer.push(String::new());
                        }
                    },
                    '\r' => self.cursor_x = 0,
                    '\t' => self.cursor_x = (self.cursor_x + 8) / 8 * 8,
                    _ => {
                        // Ensure we have a line to write to
                        while self.cursor_y >= self.buffer.len() {
                            self.buffer.push(String::new());
                        }

                        // Ensure line is long enough
                        let line = &mut self.buffer[self.cursor_y];
                        while line.chars().count() < self.cursor_x {
                            line.push(' ');
                        }

                        // Insert character at cursor position
                        line.push(c);
                        self.cursor_x += 1;
                    }
                }
            }
            debug!("Processed {} bytes into terminal", data.len());
        }
    }
}

/// Draws the terminal widget using ImGui
pub fn draw_terminal_widget(
    ui: &Ui,
    state: &mut TerminalWidgetState,
    p_open: &mut bool,
    session: Option<&Arc<Mutex<ConPtySession>>>,
) {
    let window = imgui::Window::new("Terminal")
        .size([state.width as f32 * 8.0, state.height as f32 * 16.0], imgui::Condition::FirstUseEver)
        .opened(p_open);

    window.build(ui, || {
        // Update terminal dimensions based on available space
        let available_size = ui.content_region_avail();
        let child_size = [available_size[0], available_size[1] - 30.0];
        
        // Calculate approximate character dimensions
        state.width = (available_size[0] / 8.0) as usize;  // Assuming 8px char width
        state.height = (child_size[1] / 16.0) as usize;    // Assuming 16px char height
        
        // Use imgui::ChildWindow for scrolling area in imgui 0.8.0
        imgui::ChildWindow::new("terminal_output")
            .size(child_size)
            .build(ui, || {
                // Display terminal content from buffer
                for line in &state.buffer {
                    ui.text(line);
                }

                // Auto-scroll to bottom when at bottom
                if ui.scroll_y() >= ui.scroll_max_y() {
                    ui.set_scroll_here_y();
                }
            });

        // Input field
        let mut input_changed = false;
        let input_width = ui.content_region_avail()[0] - 60.0;
        ui.set_next_item_width(input_width);
        
        ui.input_text("##terminal_input", &mut state.input_buffer)
            .enter_returns_true(true)
            .build();
        
        ui.same_line();
        if ui.button("Send") || ui.is_item_active() && ui.is_key_pressed(imgui::Key::Enter) {
            input_changed = true;
        }

        // Process input when Enter is pressed
        if input_changed && !state.input_buffer.is_empty() {
            if let Some(session_ref) = session {
                if let Ok(mut session) = session_ref.lock() {
                    // Send input to ConPTY
                    let input_with_newline = format!("{}\r\n", state.input_buffer);
                    if let Err(e) = session.write(input_with_newline.as_bytes()) {
                        info!("Failed to send input to ConPTY: {}", e);
                    }
                }
            }
            
            // Clear input buffer after sending
            state.input_buffer.clear();
        }
    });
}