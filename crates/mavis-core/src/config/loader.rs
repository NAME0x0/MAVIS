// Configuration loader for MAVIS - primarily sets up the Lua config environment

use crate::config::Config;
use crate::error::CoreError;
use crate::utils; // Assuming get_local_app_data is here
use log::{debug, info, warn};
use std::fs;
use std::path::{Path, PathBuf};

/// Handles ensuring the user configuration directory exists and contains default files.
pub struct ConfigLoader;

impl ConfigLoader {
    /// Workspace directory containing default Lua config templates.
    const WORKSPACE_CONFIG_TEMPLATE_DIR: &'static str = "./config";
    /// Default Lua config files to copy if missing.
    const DEFAULT_LUA_FILES: &'static [&'static str] = &["init.lua", "keybindings.lua"]; // Add others as needed

    /// Ensures the user config directory exists and copies default Lua files if needed.
    /// Returns a default Config struct; actual configuration is applied by Lua scripts later.
    pub fn ensure_config_env() -> Result<(Config, PathBuf), CoreError> {
        let user_config_dir = Self::get_user_config_dir()?;

        // Ensure the user config directory exists
        if !user_config_dir.exists() {
            info!("Creating user config directory at {:?}", user_config_dir);
            fs::create_dir_all(&user_config_dir).map_err(|e| {
                CoreError::ConfigError(format!(
                    "Failed to create user config directory {:?}: {}",
                    user_config_dir, e
                ))
            })?;
        } else {
            debug!("User config directory found at {:?}", user_config_dir);
        }

        // Check for and copy default Lua files
        Self::copy_default_lua_files(&user_config_dir)?;

        // Return a default config for now. The ScriptEngine will apply the real config.
        Ok((Config::default(), user_config_dir))
    }

    /// Copies default Lua configuration files from the workspace template directory
    /// to the user's config directory if they don't already exist.
    fn copy_default_lua_files(user_config_dir: &Path) -> Result<(), CoreError> {
        let template_dir = PathBuf::from(Self::WORKSPACE_CONFIG_TEMPLATE_DIR);
        if !template_dir.is_dir() {
            warn!(
                "Workspace config template directory not found at {:?}, cannot copy defaults.",
                template_dir
            );
            // This might not be an error if defaults aren't strictly required
            // or if running in a packaged environment.
            return Ok(());
        }

        for file_name in Self::DEFAULT_LUA_FILES {
            let source_path = template_dir.join(file_name);
            let dest_path = user_config_dir.join(file_name);

            if !dest_path.exists() {
                if source_path.exists() {
                    info!("Copying default config file {:?} to {:?}", file_name, dest_path);
                    fs::copy(&source_path, &dest_path).map_err(|e| {
                        CoreError::ConfigError(format!(
                            "Failed to copy default config file {:?} to {:?}: {}",
                            source_path, dest_path, e
                        ))
                    })?;
                } else {
                    warn!(
                        "Default config template file {:?} not found in {:?}, cannot copy.",
                        file_name, template_dir
                    );
                }
            } else {
                debug!("User config file {:?} already exists.", dest_path);
            }
        }
        Ok(())
    }

    /// Gets the primary user configuration directory path (%LOCALAPPDATA%/MAVIS/config).
    pub fn get_user_config_dir() -> Result<PathBuf, CoreError> {
        let local_app_data = utils::get_local_app_data()?;
        Ok(local_app_data.join("MAVIS").join("config"))
    }

    // Removed load_from_file (JSON specific)
    // Removed create_default (replaced by copy_default_lua_files)
    // Removed get_config_paths (simplified to get_user_config_dir)
}