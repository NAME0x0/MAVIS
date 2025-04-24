// Logging API functions for Lua scripts
// These functions provide integration with MAVIS logging system

use crate::error::CoreError;
use log::{debug, error, info, trace, warn};
use mlua::{Lua, Table};

/// Register logging functions in the provided table
pub fn register_logging_functions(lua: &Lua, table: &Table) -> Result<(), CoreError> {
    // Register trace function
    let trace_fn = lua.create_function(|_, message: String| {
        trace!("[Lua] {}", message);
        Ok(())
    })
    .map_err(|e| CoreError::LuaError(format!("Failed to create trace function: {}", e)))?;
    
    table.set("trace", trace_fn)
        .map_err(|e| CoreError::LuaError(format!("Failed to set trace function: {}", e)))?;
    
    // Register debug function
    let debug_fn = lua.create_function(|_, message: String| {
        debug!("[Lua] {}", message);
        Ok(())
    })
    .map_err(|e| CoreError::LuaError(format!("Failed to create debug function: {}", e)))?;
    
    table.set("debug", debug_fn)
        .map_err(|e| CoreError::LuaError(format!("Failed to set debug function: {}", e)))?;
    
    // Register info function
    let info_fn = lua.create_function(|_, message: String| {
        info!("[Lua] {}", message);
        Ok(())
    })
    .map_err(|e| CoreError::LuaError(format!("Failed to create info function: {}", e)))?;
    
    table.set("info", info_fn)
        .map_err(|e| CoreError::LuaError(format!("Failed to set info function: {}", e)))?;
    
    // Register warn function
    let warn_fn = lua.create_function(|_, message: String| {
        warn!("[Lua] {}", message);
        Ok(())
    })
    .map_err(|e| CoreError::LuaError(format!("Failed to create warn function: {}", e)))?;
    
    table.set("warn", warn_fn)
        .map_err(|e| CoreError::LuaError(format!("Failed to set warn function: {}", e)))?;
    
    // Register error function
    let error_fn = lua.create_function(|_, message: String| {
        error!("[Lua] {}", message);
        Ok(())
    })
    .map_err(|e| CoreError::LuaError(format!("Failed to create error function: {}", e)))?;
    
    table.set("error", error_fn)
        .map_err(|e| CoreError::LuaError(format!("Failed to set error function: {}", e)))?;
    
    // Register formatted_log function that accepts a level and a message
    let formatted_log_fn = lua.create_function(|_, (level, message): (String, String)| {
        match level.to_lowercase().as_str() {
            "trace" => trace!("[Lua] {}", message),
            "debug" => debug!("[Lua] {}", message),
            "info" => info!("[Lua] {}", message),
            "warn" | "warning" => warn!("[Lua] {}", message),
            "error" | "err" => error!("[Lua] {}", message),
            _ => info!("[Lua] {}", message), // Default to info level
        }
        Ok(())
    })
    .map_err(|e| CoreError::LuaError(format!("Failed to create formatted_log function: {}", e)))?;
    
    table.set("log", formatted_log_fn)
        .map_err(|e| CoreError::LuaError(format!("Failed to set formatted_log function: {}", e)))?;
    
    info!("Logging API functions registered successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::{Lua, Value};
    
    #[test]
    fn test_register_logging_functions() {
        let lua = Lua::new();
        let globals = lua.globals();
        let mavis_table = lua.create_table().unwrap();
        let log_table = lua.create_table().unwrap();
        
        register_logging_functions(&lua, &log_table).unwrap();
        globals.set("log", log_table).unwrap();
        
        // This should not panic
        let result = lua.load("log.info('Test info message')").exec();
        assert!(result.is_ok());
        
        let result = lua.load("log.error('Test error message')").exec();
        assert!(result.is_ok());
        
        let result = lua.load("log.log('debug', 'Test debug message with level')").exec();
        assert!(result.is_ok());
    }
}