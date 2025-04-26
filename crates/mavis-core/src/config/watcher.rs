// Configuration file watcher for hot reloading

use crate::error::CoreError;
use log::{debug, error, info, warn};
use notify::{
    Config as NotifyConfig, Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult,
    Watcher,
}; // Adjusted imports
use std::path::PathBuf; // Removed Path as it's unused
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Callback type for configuration reload requests.
pub type ReloadCallback = Box<dyn FnMut(PathBuf) + Send + 'static>; // Pass changed path

/// Watches configuration files and directories for changes to trigger hot reloading.
pub struct ConfigWatcher {
    // Watcher is kept alive by holding this Option. Dropping it stops watching.
    watcher: Option<RecommendedWatcher>,
    // Callbacks are invoked directly by the notify crate's event handler.
    callbacks: Arc<Mutex<Vec<ReloadCallback>>>,
}

impl ConfigWatcher {
    /// Creates a new config watcher.
    pub fn new() -> Result<Self, CoreError> {
        Ok(Self {
            watcher: None,
            callbacks: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Starts watching the user's config and themes directories.
    pub fn start_watching(&mut self) -> Result<(), CoreError> {
        if self.watcher.is_some() {
            warn!("Watcher already started.");
            return Ok(());
        }

        // Clone Arc for the callback closure
        let callbacks = self.callbacks.clone();

        // Define the event handler closure (notify v6 API)
        let event_handler = move |res: NotifyResult<Event>| {
            match res {
                Ok(event) => {
                    // Use match for EventKind (v6 API)
                    match event.kind {
                        // Check relevant kinds: Create, Modify, Remove
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                            for path in event.paths {
                                // Check if it's a file type we care about
                                if path.extension().map_or(false, |ext| ext == "lua" || ext == "json") {
                                    debug!("Config/theme file change detected: {:?}", path);
                                    // Trigger callbacks
                                    let mut locked_callbacks = match callbacks.lock() {
                                        Ok(guard) => guard,
                                        Err(poisoned) => {
                                            error!("Callback mutex poisoned: {}", poisoned);
                                            // Attempt to recover or simply return
                                            poisoned.into_inner()
                                        }
                                    };
                                    for callback in locked_callbacks.iter_mut() {
                                        // Execute the callback, passing the changed path
                                        callback(path.clone());
                                    }
                                    // Break inner loop once a relevant file is found in the event
                                    break;
                                }
                            }
                        }
                        _ => { /* Ignore other event kinds like Access, Other */ }
                    }
                }
                Err(e) => error!("File watch error: {:?}", e),
            }
        };

        // Create a watcher with the callback and config
        let mut watcher = RecommendedWatcher::new(
            event_handler, // Pass the closure directly
            NotifyConfig::default().with_poll_interval(Duration::from_secs(2)),
        )
        .map_err(|e| CoreError::NotifyError(format!("Failed to create file watcher: {}", e)))?;

        // Determine directories to watch
        let user_config_dir = super::loader::ConfigLoader::get_user_config_dir()?;
        let user_themes_dir = user_config_dir.parent().ok_or_else(|| CoreError::ConfigError("Failed to get parent directory for themes".to_string()))?.join("themes"); // Safer parent access

        // Watch config directory
        if user_config_dir.exists() {
            watcher
                .watch(user_config_dir.as_path(), RecursiveMode::Recursive) // Use as_path()
                .map_err(|e| {
                    CoreError::NotifyError(format!(
                        "Failed to watch config directory {:?}: {}",
                        user_config_dir, e
                    ))
                })?;
            info!("Started watching config directory: {:?}", user_config_dir);
        } else {
            warn!(
                "User config directory {:?} does not exist, cannot watch.",
                user_config_dir
            );
        }

        // Watch themes directory (create if it doesn't exist)
        if !user_themes_dir.exists() {
            info!("Creating user themes directory at {:?}", user_themes_dir);
            std::fs::create_dir_all(&user_themes_dir).map_err(|e| {
                CoreError::IoError(std::io::Error::new( // Use IoError for fs operations
                    std::io::ErrorKind::Other,
                    format!("Failed to create user themes directory {:?}: {}", user_themes_dir, e)
                ))
            })?;
        }
        watcher
            .watch(user_themes_dir.as_path(), RecursiveMode::Recursive) // Use as_path()
            .map_err(|e| {
                CoreError::NotifyError(format!(
                    "Failed to watch themes directory {:?}: {}",
                    user_themes_dir, e
                ))
            })?;
        info!("Started watching themes directory: {:?}", user_themes_dir);

        // Store the watcher to keep it alive
        self.watcher = Some(watcher);

        // No separate thread needed; notify handles event dispatch

        Ok(())
    }

    // event_handler_loop is no longer needed

    /// Registers a callback to be invoked when a relevant config/theme file changes.
    pub fn add_reload_callback<F>(&mut self, callback: F)
    where
        F: FnMut(PathBuf) + Send + 'static,
    {
        let mut locked_callbacks = self.callbacks.lock().unwrap(); // Handle potential poisoning if needed
        locked_callbacks.push(Box::new(callback));
        info!("Added reload callback.");
    }

    // Removed get_config() as watcher no longer holds config state.
}

// Drop implementation is usually not needed for RecommendedWatcher,
// as dropping the watcher instance automatically stops watching.