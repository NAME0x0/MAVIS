use crate::error::CoreError;
use mlua::{Lua, Table, Value};

/// Registers widget-related functions into the Lua state.
/// Creates the `mavis.widgets` table.
pub fn register_widget_functions(lua: &Lua, mavis_table: &Table) -> Result<(), CoreError> {
    let widgets_table = super::create_nested_table(lua, mavis_table, "widgets")?;

    // Placeholder for add_widget function
    let add_widget = lua.create_function(|_lua, (area, config): (String, Table)| {
        // TODO: Implement actual widget addition logic.
        // This will involve parsing the config table, creating the widget state,
        // and communicating with the GUI to display it in the specified area.
        println!("Placeholder: add_widget called for area '{}'", area);
        // You might want to inspect the config table here for debugging
        // for pair in config.pairs::<String, Value>() {
        //     let (key, value) = pair?;
        //     println!("  Config: {} = {:?}", key, value);
        // }
        Ok(())
    })?;

    // Placeholder for toggle_widget function
    let toggle_widget = lua.create_function(|_lua, widget_id: String| {
        // TODO: Implement widget toggling logic.
        // This needs to find the widget by its ID and signal the GUI to show/hide it.
        println!("Placeholder: toggle_widget called for ID '{}'", widget_id);
        Ok(())
    })?;

    widgets_table.set("add_widget", add_widget)?;
    widgets_table.set("toggle_widget", toggle_widget)?;

    Ok(())
}