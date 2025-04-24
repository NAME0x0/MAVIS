//! Terminal widget using imgui and leveraging ConPTY from mavis-core.

use imgui::{InputTextFlags, StyleColor, Ui, Window};
use log::debug;
use mavis_core::ConPtySession; // Will be needed later
use std::sync::{Arc, Mutex}; // For potential shared state

// TODO: Define state specific to the terminal widget
// - Buffer for terminal output lines
// - Input buffer for user commands
// - Reference to the ConPtySession
// - Scrollback settings
// - ANSI/VT sequence parser state
pub struct TerminalWidgetState {
    // Example state fields
    output_buffer: Vec<String>,
    input_buffer: String,
    // conpty_session: Option<Arc<Mutex<ConPtySession>>>, // Maybe managed elsewhere
    needs_scroll_to_bottom: bool,
}

impl TerminalWidgetState {
    pub fn new() -> Self {
        Self {
            output_buffer: vec!["Welcome to MAVIS Terminal (WIP)...".to_string()],
            input_buffer: String::with_capacity(256),
            // conpty_session: None,
            needs_scroll_to_bottom: true,
        }
    }

    /// Processes raw byte output from the ConPTY session.
    /// For now, converts lossily to UTF-8 and appends to the buffer.
    /// TODO: Implement proper ANSI/VT sequence parsing.
    pub fn process_output(&mut self, data: &[u8]) {
        if data.is_empty() {
            // Could indicate EOF or error from reader thread
            self.output_buffer.push("--- Terminal session ended ---".to_string());
            self.needs_scroll_to_bottom = true;
            return;
        }

        // Simple lossy conversion for now
        let output_str = String::from_utf8_lossy(data);
        debug!("Received terminal output chunk: {} bytes", data.len());

        // Append to buffer (potentially splitting lines later)
        // For simplicity, just append the whole chunk as one line for now.
        // A better approach would parse lines based on \n or \r\n.
        self.output_buffer.push(output_str.to_string());
        self.needs_scroll_to_bottom = true;
    }

    /// Sends user input to the ConPTY session.
    pub fn send_input(&mut self, session_arc: &Arc<Mutex<ConPtySession>>, input: &str) {
        // Append newline appropriate for the shell (e.g., cmd.exe needs \r\n)
        let mut command = input.to_string();
        command.push_str("\r\n");

        match session_arc.lock() {
            Ok(mut session_guard) => {
                match session_guard.write(command.as_bytes()) {
                    Ok(bytes_written) => {
                        debug!("Wrote {} bytes to ConPTY input: {}", bytes_written, input);
                    }
                    Err(e) => {
                        error!("Failed to write to ConPTY session: {}", e);
                        self.output_buffer.push(format!("--- Error sending input: {} ---", e));
                        self.needs_scroll_to_bottom = true;
                    }
                }
            }
            Err(poisoned) => {
                error!("ConPTY session mutex poisoned during input send: {}", poisoned);
                self.output_buffer.push("--- Terminal session error (mutex poisoned) ---".to_string());
                self.needs_scroll_to_bottom = true;
            }
        }
    }
}

/// Draws the terminal widget.
pub fn draw_terminal_widget(
    ui: &Ui,
    state: &mut TerminalWidgetState,
    is_open: &mut bool,
    session: Option<&Arc<Mutex<ConPtySession>>>, // Add session parameter
) {
    Window::new("Terminal")
        .opened(is_open)
        .size([800.0, 600.0], imgui::Condition::FirstUseEver)
        .build(ui, || {
            // Reserve space for the input line at the bottom
            let footer_height = ui.text_line_height_with_spacing() + ui.style().frame_padding[1] * 2.0;
            let output_region_height = ui.content_region_avail()[1] - footer_height;

            // Output area (scrollable)
            let output_child = ui.begin_child_with_sizing(
                "terminal_output",
                [0.0, output_region_height], // Width = auto, Height = calculated
                true, // Border
                imgui::WindowFlags::HORIZONTAL_SCROLLBAR,
            );
            if output_child {
                // Display buffered output lines
                // TODO: Use a more efficient buffer and rendering approach (e.g., clipping)
                // TODO: Handle colors/styles from ANSI codes
                for line in &state.output_buffer {
                    ui.text_unformatted(line);
                }

                // Auto-scroll to bottom if needed
                if state.needs_scroll_to_bottom || (ui.scroll_y() >= ui.scroll_max_y() && ui.is_window_focused()) {
                    ui.set_scroll_here_y(1.0);
                    state.needs_scroll_to_bottom = false;
                }
            }
            ui.end_child(); // End output_child

            ui.separator();

            // Input area
            let input_flags = InputTextFlags::ENTER_RETURNS_TRUE | InputTextFlags::NO_UNDO_REDO; // Example flags
            let input_text = ui.input_text("##terminal_input", &mut state.input_buffer)
                .flags(input_flags)
                .build();

            // Handle input submission (Enter key pressed)
            if input_text && !state.input_buffer.is_empty() {
                let command_to_send = state.input_buffer.clone(); // Clone input before clearing
                debug!("Terminal input submitted: {}", command_to_send);

                // Add command to output buffer (optional, for local echo feel)
                // state.output_buffer.push(format!("> {}", command_to_send));

                // Send input to ConPTY session
                if let Some(sess_arc) = session {
                    // Call the send_input method (to be implemented)
                    state.send_input(sess_arc, &command_to_send);
                } else {
                    state.output_buffer.push("--- No active terminal session ---".to_string());
                }

                state.input_buffer.clear();
                state.needs_scroll_to_bottom = true;
                ui.set_keyboard_focus_here_with_offset(0.0);
            }

            // Keep focus on input field when window is focused
            if ui.is_window_focused() && !ui.is_any_item_active() && !ui.is_mouse_clicked(imgui::MouseButton::Left) {
                 ui.set_keyboard_focus_here_with_offset(0.0);
            }
        });
}