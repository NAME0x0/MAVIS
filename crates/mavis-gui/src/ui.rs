use crate::state::GuiState;
use crate::widgets;
use imgui::Ui;
use log::debug; // Added debug
use mavis_core::config::Config as CoreConfig;

/// Main function to draw the MAVIS user interface.
pub fn draw_ui(ui: &Ui, state: &mut GuiState, core_config: &CoreConfig) {
    // --- Process Async Updates (e.g., ConPTY output) ---
    process_async_updates(state);

    // --- Top Menu Bar ---
    draw_menu_bar(ui, state);

    // --- Main Content Area ---
    // This is where widgets defined in Lua will eventually be drawn.
    // For now, we can draw some basic info or the demo window.

    // Example: Show resource usage from GuiState
    ui.window("System Monitor")
        .size([300.0, 150.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("CPU Usage: {:.1}%", state.resource_usage.cpu_usage));
            ui.text(format!(
                "Memory Usage: {:.1}% ({:.1} / {:.1} GB)",
                state.resource_usage.memory_usage,
                state.resource_usage.used_memory as f64 / 1_073_741_824.0, // Convert bytes to GB
                state.resource_usage.total_memory as f64 / 1_073_741_824.0
            ));
            ui.separator();
            ui.text(format!(
                "Network Down: {:.2} KB/s",
                state.resource_usage.network_down_bytes as f64 / 1024.0 // Bytes to KB
            ));
            ui.text(format!(
                "Network Up: {:.2} KB/s",
                state.resource_usage.network_up_bytes as f64 / 1024.0 // Bytes to KB
            ));
             ui.separator();
            ui.text(format!("Disk Usage: {:.1}%", state.resource_usage.disk_usage));
             ui.text(format!(
                 "Disk Read: {:.2} MB/s",
                 state.resource_usage.disk_read_bytes as f64 / 1_048_576.0 // Bytes to MB
             ));
             ui.text(format!(
                 "Disk Write: {:.2} MB/s",
                 state.resource_usage.disk_write_bytes as f64 / 1_048_576.0 // Bytes to MB
             ));
        });


    // --- Demo Window ---
    if state.show_demo_window {
        ui.show_demo_window(&mut state.show_demo_window);
    }

    // --- Terminal Widget ---
    if state.show_terminal {
        // Clone the Option<Arc> to pass to the widget function
        let session_ref = state.conpty_session.as_ref();
        widgets::terminal::draw_terminal_widget(
            ui,
            &mut state.terminal_state,
            &mut state.show_terminal,
            session_ref, // Pass the Option<&Arc<Mutex<ConPtySession>>>
        );
    }

    // --- Handle Exit Request ---
    if state.should_exit {
        // TODO: Find a way to signal exit to the main loop cleanly.
        // This might involve returning a value or using a shared flag accessible by the loop.
        // For now, just log it.
        log::info!("UI requested exit.");
        // A simple approach for winit is to request redraw and then check the flag
        // before rendering, then call control_flow = ControlFlow::Exit;
        // Or use EventLoopProxy to send a custom event.
    }
}

/// Draws the main menu bar.
fn draw_menu_bar(ui: &Ui, state: &mut GuiState) {
    ui.main_menu_bar(|| {
        ui.menu("File", || {
            if ui.menu_item("Exit") {
                state.should_exit = true;
            }
        });
        ui.menu("View", || {
            // Example: Toggle demo window visibility
            if ui.menu_item_config("Show Demo Window")
                .selected(state.show_demo_window)
                .build() {
                state.show_demo_window = !state.show_demo_window;
            }
            ui.separator();
            // Toggle Terminal window visibility
             if ui.menu_item_config("Show Terminal")
                 .selected(state.show_terminal)
                 .build() {
                 state.show_terminal = !state.show_terminal;
             }
            // TODO: Add menu items to toggle other specific MAVIS widgets based on state.widget_visibility
        });
        // Add other menus (e.g., "Help")
    });
}

/// Processes updates received from background threads (e.g., ConPTY reader).
fn process_async_updates(state: &mut GuiState) {
    // Check for ConPTY output
    if let Some(rx) = state.conpty_output_rx.as_ref() {
        // Use try_recv for non-blocking check
        while let Ok(data) = rx.try_recv() {
             debug!("Received {} bytes from ConPTY channel", data.len());
            state.terminal_state.process_output(&data);
            if data.is_empty() {
                // Reader thread signaled exit/error, remove receiver?
                // state.conpty_output_rx = None; // Or handle state differently
                debug!("Reader thread signaled exit/error.");
            }
        }
    }
}

// TODO: Implement functions to draw specific widgets based on Lua configuration.
// fn draw_cpu_widget(ui: &Ui, state: &GuiState) { ... }
// fn draw_memory_widget(ui: &Ui, state: &GuiState) { ... }