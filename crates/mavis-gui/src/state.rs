use crate::ide::IdeState;
use crate::widgets::terminal::TerminalWidgetState;
use mavis_core::{monitor::ResourceUsage, ConPtySession};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

/// Holds the current state of the MAVIS GUI.
#[derive(Debug, Default)]
pub struct GuiState {
    /// Should the application exit?
    pub should_exit: bool,

    /// Last known resource usage data.
    pub resource_usage: ResourceUsage,

    /// Visibility state of different UI panels or widgets.
    /// Key: Widget/Panel ID (String), Value: visible (bool)
    pub widget_visibility: HashMap<String, bool>,

    // Add other state fields as needed:
    // - Theme information (colors, fonts)
    // - Widget configurations
    // - Data fetched from core (e.g., system info)
    // - User input state
    // - etc.

    // Example: Placeholder for a demo window toggle
    pub show_demo_window: bool,

    // State for the terminal widget
    pub terminal_state: TerminalWidgetState,
    pub show_terminal: bool,
    
    // Skip complex types that don't implement Serialize/Deserialize
    pub conpty_session: Option<Arc<Mutex<ConPtySession>>>,

    // Channel for ConPTY output reading thread
    pub conpty_output_rx: Option<Receiver<Vec<u8>>>,
    pub conpty_output_tx: Option<Sender<Vec<u8>>>, // Kept here temporarily for setup ease

    // State for the IDE component
    pub ide_state: IdeState, // NEW
}

impl GuiState {
    /// Creates a new default GuiState.
    pub fn new() -> Self {
        Self {
            should_exit: false,
            resource_usage: ResourceUsage::default(),
            widget_visibility: HashMap::new(),
            show_demo_window: true, // Show demo window by default initially
            terminal_state: TerminalWidgetState::new(),
            show_terminal: true,
            conpty_session: None,
            conpty_output_rx: None, // Initialize channel ends as None
            conpty_output_tx: None,
            ide_state: IdeState::new(), // NEW: Initialize IdeState
        }
    }

    /// Updates the resource usage data.
    pub fn update_resource_usage(&mut self, usage: ResourceUsage) {
        self.resource_usage = usage;
    }

    /// Sets the visibility of a specific widget/panel.
    pub fn set_widget_visibility(&mut self, id: String, visible: bool) {
        self.widget_visibility.insert(id, visible);
    }

    /// Toggles the visibility of a specific widget/panel.
    /// Returns the new visibility state.
    pub fn toggle_widget_visibility(&mut self, id: &str) -> bool {
        let current_visibility = self.widget_visibility.get(id).copied().unwrap_or(false);
        let new_visibility = !current_visibility;
        self.widget_visibility.insert(id.to_string(), new_visibility);
        new_visibility
    }

    /// Checks if a specific widget/panel is visible.
    pub fn is_widget_visible(&self, id: &str) -> bool {
        self.widget_visibility.get(id).copied().unwrap_or(false)
    }
}