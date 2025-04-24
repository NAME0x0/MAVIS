// Utility functions for MAVIS core

pub mod filesystem;

// Re-export commonly used functions
pub use filesystem::{ensure_local_dirs, get_local_app_data, get_install_dir};