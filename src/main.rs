// MAVIS - Main Application Entry Point

use log::{info, error};
use mavis_gui::window::run_gui;
use mavis_core::config::Config;
use std::sync::{Arc, Mutex};
use mavis_gui::state::GuiState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    info!("MAVIS starting up...");

    // Load configuration
    let config = Config::default();
    
    // Create GUI state
    let gui_state = Arc::new(Mutex::new(GuiState::default()));

    // Create and run the main window
    match run_gui(&config, gui_state) {
        Ok(_) => {
            info!("MAVIS shutdown successfully");
            Ok(())
        },
        Err(e) => {
            error!("MAVIS encountered an error: {}", e);
            Err(e.into())
        }
    }
}