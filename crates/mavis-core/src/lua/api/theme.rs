use crate::error::CoreError;
use mlua::{Lua, Table, Value};

/// Registers theme-related functions into the Lua state.
/// Creates the `mavis.theme` table.
pub fn register_theme_functions(lua: &Lua, mavis_table: &Table) -> Result<(), CoreError> {
    let theme_table = super::create_nested_table(lua, mavis_table, "theme")?;

    // Placeholder for set_theme function
    let set_theme = lua.create_function(|_lua, theme_arg: Value| {
        // TODO: Implement actual theme setting logic.
        // This will involve parsing the theme_arg (which could be a string name or a table),
        // loading the theme data, and applying it to the GUI state.
        match theme_arg {
            Value::String(s) => {
                println!("Placeholder: set_theme called with theme name: {}", s.to_str()?);
            }
            Value::Table(_) => {
                println!("Placeholder: set_theme called with theme table");
            }
            _ => {
                return Err(mlua::Error::RuntimeError(
                    "Invalid argument to set_theme: expected string or table".to_string(),
                ));
            }
        }
        Ok(())
    })?;

    theme_table.set("set_theme", set_theme)?;

    Ok(())
}