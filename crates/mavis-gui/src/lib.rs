// MAVIS GUI - Dear ImGui based graphical user interface

// Modules
pub mod error;
pub mod renderer;
pub mod state;
pub mod ui;
pub mod window;
pub mod widgets; // For custom or complex widgets

// Re-exports for easier access from main binary
pub use error::GuiError;
pub use window::run_gui; // Assuming run_gui will be the main entry point

use log::info;

/// Initializes the GUI subsystem (if needed).
/// Currently just logs initialization.
pub fn initialize() {
    info!("Initializing MAVIS GUI v{}", env!("CARGO_PKG_VERSION"));
    // Any one-time GUI setup can go here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        // Basic test to ensure initialization doesn't panic
        initialize();
        assert!(true);
    }
}