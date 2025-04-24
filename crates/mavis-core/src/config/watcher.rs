// Configuration file watcher for hot reloading

use crate::error::CoreError;
use crate::utils; // Assuming get_local_app_data is here
use log::{debug, error, info, warn};
use notify::{Config as NotifyConfig, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex}; // Keep Mutex for the callback Vec
use std::thread;
use std::time::Duration;

/// Callback type for configuration reload requests.
pub type ReloadCallback = Box<dyn FnMut(PathBuf) + Send + 'static>; // Pass changed path

/// Watches configuration files and directories for changes to trigger hot reloading.
pub struct ConfigWatcher {
    watcher: Option<RecommendedWatcher>,
    _thread_handle: Option<thread::JoinHandle<()>>,
    callbacks: Arc<Mutex<Vec<ReloadCallback>>>, // Callbacks need to be shared with the thread
}

impl ConfigWatcher {
    /// Creates a new config watcher.
    pub fn new() -> Result<Self, CoreError> {
        Ok(Self {
            watcher: None,
            _thread_handle: None,
            callbacks: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Starts watching the user's config and themes directories.
    pub fn start_watching(&mut self) -> Result<(), CoreError> {
        if self.watcher.is_some() {
            warn!("Watcher already started.");
            return Ok(());
        }

        let (tx, rx) = mpsc::channel();

        // Create a watcher
        let mut watcher = RecommendedWatcher::new(
            tx,
            NotifyConfig::default().with_poll_interval(Duration::from_secs(2)), // Slightly longer poll interval
        )
        .map_err(|e| CoreError::NotifyError(format!("Failed to create file watcher: {}", e)))?;

        // Determine directories to watch
        let user_config_dir = super::loader::ConfigLoader::get_user_config_dir()?;
        let user_themes_dir = user_config_dir.parent().unwrap().join("themes"); // Assumes themes dir is sibling to config

        // Watch config directory
        if user_config_dir.exists() {
            watcher
                .watch(&user_config_dir, RecursiveMode::Recursive)
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
                CoreError::ConfigError(format!(
                    "Failed to create user themes directory {:?}: {}",
                    user_themes_dir, e
                ))
            })?;
        }
        watcher
            .watch(&user_themes_dir, RecursiveMode::Recursive)
            .map_err(|e| {
                CoreError::NotifyError(format!(
                    "Failed to watch themes directory {:?}: {}",
                    user_themes_dir, e
                ))
            })?;
        info!("Started watching themes directory: {:?}", user_themes_dir);

        self.watcher = Some(watcher);

        // Clone Arc for the thread
        let callbacks = self.callbacks.clone();

        // Spawn thread to handle events
        let thread_handle = thread::spawn(move || {
            Self::event_handler_loop(rx, callbacks);
        });

        self._thread_handle = Some(thread_handle);

        Ok(())
    }

    /// The loop running on the watcher thread to process events.
    fn event_handler_loop(
        rx: mpsc::Receiver<Result<Event, notify::Error>>,
        callbacks: Arc<Mutex<Vec<ReloadCallback>>>,
    ) {
        info!("Config watcher thread started.");
        for result in rx {
            match result {
                Ok(event) => {
                    // Check for relevant modification events (Create, Modify, Rename)
                    // We check for create/rename too in case files are edited via temp file swaps.
                    if event.kind.is_modify() || event.kind.is_create() || event.kind.is_rename() {
                        for path in event.paths {
                            // Check if it's a file type we care about
                            if path.extension().map_or(false, |ext| ext == "lua" || ext == "json") {
                                debug!("Config/theme file change detected: {:?}", path);
                                // Trigger callbacks
                                let mut locked_callbacks = callbacks.lock().unwrap();
                                for callback in locked_callbacks.iter_mut() {
                                    // Execute the callback, passing the changed path
                                    callback(path.clone());
                                }
                                // Break inner loop once a relevant file is found in the event
                                break;
                            }
                        }
                    }
                }
                Err(e) => error!("File watch error: {:?}", e),
            }
        }
        info!("Config watcher thread finished.");
    }

    /// Registers a callback to be invoked when a relevant config/theme file changes.
    pub fn add_reload_callback<F>(&mut self, callback: F)
    where
        F: FnMut(PathBuf) + Send + 'static,
    {
        let mut locked_callbacks = self.callbacks.lock().unwrap();
        locked_callbacks.push(Box::new(callback));
        info!("Added reload callback.");
    }

    // Removed get_config() as watcher no longer holds config state.
}

// Optional: Implement Drop to ensure the watcher is cleaned up?
// impl Drop for ConfigWatcher {
//     fn drop(&mut self) {
//         info!("Dropping ConfigWatcher.");
//         // The watcher should automatically unwatch when dropped.
//         // We might want to join the thread handle here if necessary,
//         // but it might block shutdown. Consider if needed.
//     }
// }