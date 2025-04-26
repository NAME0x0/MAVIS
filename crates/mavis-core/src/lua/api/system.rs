// System API functions for Lua scripts
// These functions provide controlled access to system functionality

use crate::error::CoreError;
use log::{debug, info, warn};
use mlua::{Lua, Table, Value};  // Re-added Value for get_system_info_fn
use std::process::Command;
use std::path::Path;

/// Register system functions in the provided table
/// These functions are only available when unsafe mode is enabled
pub fn register_system_functions(lua: &Lua, table: &Table) -> Result<(), CoreError> {
    // Execute a system command
    let exec_fn = lua.create_function(|lua_ctx, cmd: String| {
        debug!("Lua script executing command: {}", cmd);
        let output = Command::new("cmd")
            .args(&["/C", &cmd])
            .output()
            .map_err(|e| mlua::Error::RuntimeError(format!("Failed to execute command: {}", e)))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        
        // Use the lua context provided to the closure, not the outer scope
        let result_table = lua_ctx.create_table()?;
        result_table.set("stdout", stdout)?;
        result_table.set("stderr", stderr)?;
        result_table.set("status", output.status.code().unwrap_or(-1))?;
        result_table.set("success", output.status.success())?;
        
        Ok(result_table)
    })
    .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to create exec function: {}", e))))?;
    
    table.set("exec", exec_fn)
        .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to set exec function: {}", e))))?;
    
    // Get environment variable
    let getenv_fn = lua.create_function(|_, name: String| {
        match std::env::var(&name) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    })
    .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to create getenv function: {}", e))))?;
    
    table.set("getenv", getenv_fn)
        .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to set getenv function: {}", e))))?;
    
    // Check if a file exists
    let file_exists_fn = lua.create_function(|_, path: String| {
        Ok(Path::new(&path).exists())
    })
    .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to create file_exists function: {}", e))))?;
    
    table.set("file_exists", file_exists_fn)
        .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to set file_exists function: {}", e))))?;
    
    // Launch an application
    let launch_app_fn = lua.create_function(|_, (app_path, args): (String, Option<String>)| {
        debug!("Launching application: {} with args: {:?}", app_path, args);
        
        let mut cmd = Command::new(&app_path);
        if let Some(arg_str) = args {
            cmd.args(arg_str.split_whitespace());
        }
        
        match cmd.spawn() {
            Ok(_) => Ok(true),
            Err(e) => {
                warn!("Failed to launch application {}: {}", app_path, e);
                Ok(false)
            }
        }
    })
    .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to create launch_app function: {}", e))))?;
    
    table.set("launch_app", launch_app_fn)
        .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to set launch_app function: {}", e))))?;
    
    // Get current timestamp
    let get_timestamp_fn = lua.create_function(|_, ()| {
        let now = std::time::SystemTime::now();
        let unix_time = now.duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| mlua::Error::RuntimeError(format!("SystemTime error: {}", e)))?;
        
        Ok(unix_time.as_secs())
    })
    .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to create get_timestamp function: {}", e))))?; // Wrap error

    table.set("get_timestamp", get_timestamp_fn)
        .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to set get_timestamp function: {}", e))))?; // Wrap error

   // Get system information (CPU usage, RAM, etc.)
   let get_system_info_fn = lua.create_function(|_lua, info_type: String| {
       // TODO: Implement logic to fetch system info based on info_type.
       // This will likely involve calling functions from the `monitor` module.
       debug!("Placeholder: get_system_info called for type '{}'", info_type);
       match info_type.as_str() {
           "cpu_usage" => Ok(Value::Number(0.0)), // Placeholder value
           "ram_usage" => Ok(Value::Number(0.0)), // Placeholder value
           "available_ram_mb" => Ok(Value::Integer(0)), // Placeholder value
           _ => Err(mlua::Error::RuntimeError(format!(
               "Unknown system info type: {}",
               info_type
           ))),
       }
   })
   .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to create get_system_info function: {}", e))))?; // Wrap error
   
   table.set("get_system_info", get_system_info_fn)
       .map_err(|e| CoreError::LuaError(mlua::Error::external(format!("Failed to set get_system_info function: {}", e))))?; // Wrap error
    
    info!("System API functions registered successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua; // Removed unused Value import
    
    #[test]
    fn test_register_system_functions() {
        let lua = Lua::new();
        let globals = lua.globals();
        let system_table = lua.create_table().unwrap();
        
        register_system_functions(&lua, &system_table).unwrap();
        globals.set("system", system_table).unwrap();
        
        // Test getenv - this should work regardless of environment
        let result: Option<String> = lua.load("return system.getenv('PATH')").eval().unwrap();
        assert!(result.is_some());
        
        // Test file_exists with the current file (which should exist)
        let current_file = file!();
        let script = format!("return system.file_exists('{}')", current_file.replace("\\", "\\\\"));
        let exists: bool = lua.load(&script).eval().unwrap();
        assert!(exists);
        
        // Test timestamp - just verify it returns a number and doesn't panic
        let timestamp: u64 = lua.load("return system.get_timestamp()").eval().unwrap();
        assert!(timestamp > 0);
    }
}