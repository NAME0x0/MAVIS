// MAVIS Core - Core functionality for MAVIS shell environment (Config, Lua, Monitor, ConPTY, etc.)

// Top-level modules
pub mod config;
pub mod error;
pub mod lua;
pub mod monitor;
// pub mod theme; // Removed: Theme is part of lua::api, not top-level
pub mod utils;
pub mod conpty; // Added ConPTY module

// Re-exports for easier use by other crates
pub use config::Config;
pub use error::{CoreError, CoreResult};
pub use lua::ScriptEngine;
pub use conpty::ConPtySession; // Re-export ConPtySession

use log::{info, warn};
use std::sync::Once;

// Global initialization for MAVIS core
static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        info!("Initializing MAVIS Core v{}", env!("CARGO_PKG_VERSION"));
        
        // Perform global initialization tasks
        utils::filesystem::ensure_local_dirs().unwrap_or_else(|e| {
            warn!("Failed to ensure local directories: {}", e);
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        // This should not panic when called multiple times
        initialize();
        initialize();
        
        // Successful test if we reach here
        assert!(true);
    }
}