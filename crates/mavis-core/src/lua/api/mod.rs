// Lua API module for MAVIS
// Exposes Rust functions to Lua scripts

pub mod logging;
pub mod system;
pub mod keybindings; // Added keybindings module
pub mod theme; // Added theme module
pub mod widgets; // Added widgets module

use crate::error::CoreError;
use mlua::{Lua, Table};

// Re-export functions for convenient use
pub use logging::register_logging_functions;
pub use system::register_system_functions;
pub use keybindings::register_keybinding_functions; // Added keybindings re-export
pub use theme::register_theme_functions; // Added theme re-export
pub use widgets::register_widget_functions; // Added widgets re-export

// Helper function to create tables with proper error handling
pub(crate) fn create_nested_table(lua: &Lua, parent: &Table, name: &str) -> Result<Table, CoreError> {
    let new_table = lua.create_table()
        .map_err(|e| CoreError::LuaError(format!("Failed to create table '{}': {}", name, e)))?;
        
    parent.set(name, new_table.clone())
        .map_err(|e| CoreError::LuaError(format!("Failed to set table '{}': {}", name, e)))?;
        
    Ok(new_table)
}