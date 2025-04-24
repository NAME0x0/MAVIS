// Lua sandboxing implementation for MAVIS

use log::{debug, info};
use mlua::{Error as LuaError, Lua, Result as LuaResult, Table, Value};
use std::collections::HashSet;

/// Applies a sandbox environment to the given Lua state
pub fn apply_sandbox(lua: &Lua, unsafe_mode: bool) -> LuaResult<()> {
    info!("Applying Lua sandbox. Unsafe mode: {}", unsafe_mode);
    
    // Get the current environment globals
    let globals = lua.globals();
    
    // List of functions that will be removed in safe mode
    let restricted_functions = if unsafe_mode {
        vec![] // Allow all functions in unsafe mode
    } else {
        vec![
            // OS / Execution functions
            "os.execute", "os.exit", "os.getenv", "os.remove", "os.rename", "os.setlocale",
            "os.tmpname", "package.loadlib",
            
            // IO functions
            "io.close", "io.flush", "io.input", "io.lines", "io.open", "io.output", 
            "io.popen", "io.read", "io.tmpfile", "io.type", "io.write",
            
            // Load/require functions
            "dofile", "loadfile",
            
            // Other potentially unsafe functions
            "collectgarbage",
        ]
    };
    
    // Apply function restrictions
    for func_path in &restricted_functions {
        remove_function(lua, &globals, func_path)?;
    }
    
    // Create a set of allowed modules
    let default_allowed_modules = vec![
        "table", "string", "math"
    ];
    
    // Additional modules in unsafe mode
    let mut allowed_modules = default_allowed_modules;
    if unsafe_mode {
        allowed_modules.push("os");
        allowed_modules.push("io");
    }
    
    // Override the require function with a sandboxed version
    let allowed_modules_set = allowed_modules
        .iter()
        .cloned()
        .collect::<HashSet<&str>>();
        
    sandbox_require(lua, globals, allowed_modules_set)?;
    
    // Apply other sandbox restrictions
    disable_metatable_access(lua)?;
    
    // In all modes, limit CPU usage
    apply_instruction_limit(lua)?;
    
    debug!("Lua sandbox applied successfully");
    Ok(())
}

/// Remove a function from the Lua environment
fn remove_function(lua: &Lua, globals: &Table, path: &str) -> LuaResult<()> {
    let parts: Vec<&str> = path.split('.').collect();
    
    if parts.len() == 1 {
        // Top-level function, just set to nil
        globals.set(parts[0], Value::Nil)?;
    } else if parts.len() >= 2 {
        // Get the table that contains the function
        let mut current_table = globals.clone();
        
        for i in 0..parts.len() - 1 {
            if i < parts.len() - 1 {
                let table_name = parts[i];
                match current_table.get::<_, Table>(table_name) {
                    Ok(next_table) => {
                        current_table = next_table;
                    },
                    Err(_) => {
                        // Table doesn't exist, so the function doesn't exist either
                        return Ok(());
                    }
                }
            }
        }
        
        // Remove the function from its parent table
        current_table.set(parts[parts.len() - 1], Value::Nil)?;
    }
    
    Ok(())
}

/// Override the require function to only allow specific modules
fn sandbox_require(lua: &Lua, globals: &Table, allowed_modules: HashSet<&str>) -> LuaResult<()> {
    // First, get the original require function
    let original_require: mlua::Function = match globals.get("require") {
        Ok(require_fn) => require_fn,
        Err(_) => return Ok(()),  // If require doesn't exist, nothing to do
    };
    
    // Create our sandboxed require function
    let sandboxed_require = lua.create_function(move |lua_ctx, module_name: String| {
        if allowed_modules.contains(module_name.as_str()) {
            // Call the original require with the module name
            original_require.call::<_, Value>(module_name)
        } else {
            Err(LuaError::RuntimeError(
                format!("Module '{}' is not allowed by the security sandbox", module_name)
            ))
        }
    })?;
    
    // Replace the global require function
    globals.set("require", sandboxed_require)?;
    
    Ok(())
}

/// Disable access to metatables for security
fn disable_metatable_access(lua: &Lua) -> LuaResult<()> {
    // Override getmetatable to return nil
    let sandboxed_getmetatable = lua.create_function(|_, _: Value| {
        Ok(Value::Nil)
    })?;
    
    // Override setmetatable to be a no-op that returns the original table
    let sandboxed_setmetatable = lua.create_function(|_, (table, _): (Table, Value)| {
        Ok(table)
    })?;
    
    // Replace the global metatable functions
    let globals = lua.globals();
    globals.set("getmetatable", sandboxed_getmetatable)?;
    globals.set("setmetatable", sandboxed_setmetatable)?;
    
    Ok(())
}

/// Apply an instruction limit to prevent infinite loops
fn apply_instruction_limit(lua: &Lua) -> LuaResult<()> {
    // Set an instruction limit hook (e.g., 1,000,000 instructions)
    // This helps prevent infinite loops or excessive CPU usage
    
    // Note: As of mlua 0.9, the hook API might be limited
    // This is a simplified placeholder implementation
    
    // In practice, you might want to use lua_sethook with LUA_MASKCOUNT
    // through raw FFI if mlua doesn't expose this functionality
    
    // For now, we'll skip actual implementation but log the intent
    debug!("Instruction limiting would be applied here (requires mlua hook support or FFI)");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sandbox_allows_safe_operations() {
        let lua = Lua::new();
        apply_sandbox(&lua, false).unwrap();
        
        // Test that safe operations work
        let result: i32 = lua.load("return 1 + 1").eval().unwrap();
        assert_eq!(result, 2);
        
        // Test safe string operations
        let result: String = lua.load("return string.upper('hello')").eval().unwrap();
        assert_eq!(result, "HELLO");
    }
    
    #[test]
    fn test_sandbox_blocks_unsafe_operations() {
        let lua = Lua::new();
        apply_sandbox(&lua, false).unwrap();
        
        // Test that os.execute is blocked
        let result = lua.load("return os.execute('echo test')").eval::<Value>();
        assert!(result.is_err());
        
        // Test that io operations are blocked
        let result = lua.load("return io.open('test.txt', 'w')").eval::<Value>();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_unsafe_mode_allows_more_operations() {
        let lua = Lua::new();
        apply_sandbox(&lua, true).unwrap();
        
        // In unsafe mode, we should be able to access os module (but maybe not execute)
        let result = lua.load("return type(os)").eval::<String>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "table");
    }
}