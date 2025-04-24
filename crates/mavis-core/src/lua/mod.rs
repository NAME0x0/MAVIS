// Lua scripting engine for MAVIS

pub mod api;
pub mod sandbox;

use crate::config::Config;
use crate::error::CoreError;
use log::{debug, error, info, warn};
use mlua::{Function, Lua, LuaOptions, StdLib, Table, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Manages Lua scripting for MAVIS
pub struct ScriptEngine {
    /// The Lua state
    lua: Arc<Mutex<Lua>>,
    
    /// Path to scripts directory
    scripts_dir: PathBuf,
    
    /// Whether unsafe mode is enabled
    unsafe_mode: bool,
}

impl ScriptEngine {
    /// Create a new Lua script engine with the specified configuration
    pub fn new(config: &Config) -> Result<Self, CoreError> {
        // Configure Lua options based on security settings
        let options = if config.security.enable_sandboxing {
            // Limit available libraries for security when sandboxing is enabled
            let mut libs = StdLib::MATH | StdLib::TABLE | StdLib::STRING;
            
            if config.security.unsafe_mode {
                // Add more powerful libraries in unsafe mode, but still restricted
                libs |= StdLib::IO | StdLib::OS;
            }
            
            LuaOptions::new().packages(libs)
        } else {
            // All libraries available when sandboxing is disabled
            LuaOptions::new()
        };
        
        // Create Lua state with configured options
        let lua = Lua::new_with(options)
            .map_err(|e| CoreError::LuaError(format!("Failed to create Lua state: {}", e)))?;
        
        // Get scripts directory from config or use default
        let local_app_data = crate::utils::get_local_app_data()?;
        let scripts_dir = local_app_data.join("MAVIS").join("scripts");
        
        // Ensure scripts directory exists
        if !scripts_dir.exists() {
            fs::create_dir_all(&scripts_dir)
                .map_err(|e| CoreError::IoError(format!("Failed to create scripts directory: {}", e)))?;
        }
        
        let engine = Self {
            lua: Arc::new(Mutex::new(lua)),
            scripts_dir,
            unsafe_mode: config.security.unsafe_mode,
        };
        
        // Initialize the Lua environment
        engine.initialize(config)?;
        
        info!("Lua script engine initialized");
        Ok(engine)
    }
    
    /// Initialize the Lua environment with MAVIS API functions
    fn initialize(&self, config: &Config) -> Result<(), CoreError> {
        let lua = self.lua.lock().unwrap();
        
        // Create global MAVIS table
        let mavis_table = lua.create_table()
            .map_err(|e| CoreError::LuaError(format!("Failed to create MAVIS table: {}", e)))?;
        
        // Add version information
        mavis_table.set("version", env!("CARGO_PKG_VERSION"))
            .map_err(|e| CoreError::LuaError(format!("Failed to set version: {}", e)))?;
        
        // Register API modules
        self.register_core_api(&lua, &mavis_table)?;
        self.register_ui_api(&lua, &mavis_table)?;
        self.register_system_api(&lua, &mavis_table, config)?;
        
        // Set the MAVIS global
        lua.globals().set("MAVIS", mavis_table)
            .map_err(|e| CoreError::LuaError(format!("Failed to set MAVIS global: {}", e)))?;
        
        // Apply sandbox if enabled
        if config.security.enable_sandboxing {
            sandbox::apply_sandbox(&lua, config.security.unsafe_mode)
                .map_err(|e| CoreError::LuaError(format!("Failed to apply sandbox: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Register core API functions (logging, keybindings, themes, widgets)
    fn register_core_api(&self, lua: &Lua, table: &Table) -> Result<(), CoreError> {
        // Register logging functions directly under MAVIS.log
        let log_table = api::create_nested_table(lua, table, "log")?;
        api::register_logging_functions(lua, &log_table)?;

        // Register keybinding functions under MAVIS.keybindings
        api::register_keybinding_functions(lua, table)?;

        // Register theme functions under MAVIS.theme
        api::register_theme_functions(lua, table)?;

        // Register widget functions under MAVIS.widgets
        api::register_widget_functions(lua, table)?;

        // TODO: Add other core API functions if needed, perhaps under MAVIS.core?
        // let core_table = lua.create_table()
        //     .map_err(|e| CoreError::LuaError(format!("Failed to create core table: {}", e)))?;
        // table.set("core", core_table)
        //     .map_err(|e| CoreError::LuaError(format!("Failed to set core table: {}", e)))?;

        // Note: Removed the separate 'core' sub-table for now, placing functions
        // in more specific tables like 'keybindings', 'theme', 'widgets', 'log'.
        table.set("core", core_table)
            .map_err(|e| CoreError::LuaError(format!("Failed to set core table: {}", e)))?;
        
        Ok(())
    }
    
    /// Register UI API functions
    fn register_ui_api(&self, lua: &Lua, table: &Table) -> Result<(), CoreError> {
        let ui_table = lua.create_table()
            .map_err(|e| CoreError::LuaError(format!("Failed to create UI table: {}", e)))?;
        
        // TODO: Register UI API functions
        
        // Add the UI table to MAVIS table
        table.set("ui", ui_table)
            .map_err(|e| CoreError::LuaError(format!("Failed to set ui table: {}", e)))?;
        
        Ok(())
    }
    
    /// Register system API functions
    fn register_system_api(&self, lua: &Lua, table: &Table, config: &Config) -> Result<(), CoreError> {
        let system_table = lua.create_table()
            .map_err(|e| CoreError::LuaError(format!("Failed to create system table: {}", e)))?;
        
        // Only expose system functions if in unsafe mode
        if self.unsafe_mode {
            api::register_system_functions(lua, &system_table)?;
        } else {
            debug!("System API functions disabled due to safe mode");
        }
        
        // Add the system table to MAVIS table
        table.set("system", system_table)
            .map_err(|e| CoreError::LuaError(format!("Failed to set system table: {}", e)))?;
        
        Ok(())
    }
    
    /// Load and execute a Lua script from the specified path
    pub fn load_script(&self, path: impl AsRef<Path>) -> Result<(), CoreError> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(CoreError::FileNotFound(format!("Script not found: {:?}", path)));
        }
        
        let script_content = fs::read_to_string(path)
            .map_err(|e| CoreError::IoError(format!("Failed to read script: {}", e)))?;
        
        let lua = self.lua.lock().unwrap();
        
        info!("Loading script: {:?}", path);
        lua.load(&script_content)
            .set_name(path.to_string_lossy().as_ref())
            .exec()
            .map_err(|e| CoreError::LuaError(format!("Failed to execute script: {}", e)))?;
            
        Ok(())
    }
    
    /// Load all scripts from the scripts directory
    pub fn load_all_scripts(&self) -> Result<(), CoreError> {
        if !self.scripts_dir.exists() {
            info!("Scripts directory doesn't exist, skipping script loading");
            return Ok(());
        }
        
        let entries = fs::read_dir(&self.scripts_dir)
            .map_err(|e| CoreError::IoError(format!("Failed to read scripts directory: {}", e)))?;
            
        let mut loaded_count = 0;
        
        for entry in entries {
            let entry = entry.map_err(|e| CoreError::IoError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "lua") {
                match self.load_script(&path) {
                    Ok(_) => loaded_count += 1,
                    Err(e) => warn!("Failed to load script {:?}: {}", path, e),
                }
            }
        }
        
        info!("Loaded {} scripts from {:?}", loaded_count, self.scripts_dir);
        Ok(())
    }
    
    /// Call a global Lua function with arguments
    pub fn call_function<A, R>(&self, name: &str, args: A) -> Result<R, CoreError>
    where
        A: for<'lua> mlua::FromLuaMulti<'lua>,
        R: for<'lua> mlua::FromLuaMulti<'lua>,
    {
        let lua = self.lua.lock().unwrap();
        
        // Get the global function
        let globals = lua.globals();
        let func: Function = globals.get(name)
            .map_err(|e| CoreError::LuaError(format!("Function '{}' not found: {}", name, e)))?;
            
        // Call the function with the provided arguments
        func.call(args)
            .map_err(|e| CoreError::LuaError(format!("Error calling function '{}': {}", name, e)))
    }
    
    /// Evaluate Lua code and return the result
    pub fn eval<R>(&self, code: &str) -> Result<R, CoreError>
    where
        R: for<'lua> mlua::FromLuaMulti<'lua>,
    {
        let lua = self.lua.lock().unwrap();
        
        lua.load(code)
            .eval()
            .map_err(|e| CoreError::LuaError(format!("Evaluation error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    #[test]
    fn test_script_engine() {
        let config = Config::default();
        let engine = ScriptEngine::new(&config);
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_evaluate_lua() {
        let config = Config::default();
        let engine = ScriptEngine::new(&config).unwrap();
        
        // Simple expression
        let result: i32 = engine.eval("return 1 + 1").unwrap();
        assert_eq!(result, 2);
    }
}