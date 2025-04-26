// Placeholder for custom ImGui widgets specific to MAVIS.
// Simple widgets can be defined directly within the ui.rs module where they are used.
// This module is intended for more complex, reusable custom widgets.

use imgui::Ui;
use crate::state::GuiState;

// Example placeholder function for a custom widget
pub fn draw_custom_meter(ui: &Ui, label: &str, value: f32, min: f32, max: f32) {
    // TODO: Implement custom drawing logic using ImGui draw commands
    ui.text(format!("{}: {:.1}%", label, value)); // Simple text for now
    ui.progress_bar(value / (max - min))
        .size([-1.0, 0.0]) // Full width
        .overlay_text(&format!("{:.1}", value))
        .build();
}

// Another example
pub fn draw_status_indicator(ui: &Ui, label: &str, is_active: bool) {
    let color = if is_active {
        [0.0, 1.0, 0.0, 1.0] // Green
    } else {
        [1.0, 0.0, 0.0, 1.0] // Red
    };
    let text = if is_active { "Active" } else { "Inactive" };

    ui.text(label);
    ui.same_line();
    ui.text_colored(color, text);
}