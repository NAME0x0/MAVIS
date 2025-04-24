use anyhow::{Context, Result}; // Using anyhow for easy error handling in main
use log::{error, info, LevelFilter};
use mavis_core::{
    config::{ConfigLoader, ConfigWatcher, Config as CoreConfig}, // Import CoreConfig
    error::CoreError,
    lua::ScriptEngine,
    monitor::ResourceMonitor,
    conpty::ConPtySession,
};
use mavis_gui::{self, state::GuiState};
use std::{
    io::Read, // Added Read trait
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, // Added mpsc
        Arc, Mutex,
    },
    thread, // Added thread
    time::Duration,
};

// Shared state or communication channel between threads might be needed later
// For example, using Arc<Mutex<...>> or channels (mpsc, tokio::sync::mpsc)

fn main() -> Result<()> {
    // 1. Initialize Logging
    env_logger::builder()
        .filter_level(LevelFilter::Debug) // Changed to Debug for more info during dev
        .init();

    info!("Starting MAVIS Shell v{}", env!("CARGO_PKG_VERSION"));

    // 2. Initialize Core Subsystems
    mavis_core::initialize();
    mavis_gui::initialize(); // Initialize GUI subsystem

    // 3. Load Configuration Environment
    // ConfigLoader now ensures the directory exists and copies defaults
    let (mut core_config, config_dir) = ConfigLoader::ensure_config_env()
        .context("Failed to ensure configuration environment")?;
    info!("Using configuration directory: {:?}", config_dir);

    // 4. Initialize Lua Script Engine
    // The ScriptEngine will load init.lua, which applies the actual config
    let script_engine = Arc::new(
        ScriptEngine::new(&core_config).context("Failed to initialize Lua script engine")?,
    );

    // Execute initial configuration scripts (e.g., init.lua)
    // TODO: Determine the exact script(s) to run initially. init.lua seems logical.
    let init_script_path = config_dir.join("init.lua");
    if init_script_path.exists() {
        info!("Loading initial Lua script: {:?}", init_script_path);
        script_engine
            .load_script(&init_script_path)
            .context(format!("Failed to load initial script {:?}", init_script_path))?;
        // TODO: Potentially update core_config based on Lua execution if needed,
        // or manage config primarily within Lua/GuiState.
    } else {
        info!(
            "Initial script {:?} not found, using default config.",
            init_script_path
        );
    }

    // 5. Initialize Resource Monitor (if enabled in config)
    // TODO: Check config before starting monitor
    let mut resource_monitor = ResourceMonitor::new(&core_config)
        .context("Failed to initialize resource monitor")?;
    resource_monitor
        .start(&core_config)
        .context("Failed to start resource monitor")?;
    info!("Resource monitor started.");

    // 6. Initialize Config Watcher
    let mut config_watcher =
        ConfigWatcher::new().context("Failed to initialize config watcher")?;

    // --- Reload Callback ---
    // Define what happens when a config file changes
    let script_engine_clone = script_engine.clone();
    config_watcher.add_reload_callback(move |changed_path| {
        info!("Config file change detected: {:?}", changed_path);
        // Determine which script(s) to reload based on the path
        // For now, let's just reload init.lua for any .lua change in config dir
        // and potentially trigger theme reload for .json in themes dir
        let config_dir_clone = config_dir.clone(); // Clone for closure
        let script_engine_inner_clone = script_engine_clone.clone(); // Clone Arc

        // TODO: Make reload logic more granular based on changed_path
        if changed_path
            .extension()
            .map_or(false, |ext| ext == "lua")
            && changed_path.starts_with(&config_dir_clone)
        {
            let init_script = config_dir_clone.join("init.lua");
            if init_script.exists() {
                info!("Reloading Lua script: {:?}", init_script);
                // It might be better to re-create the Lua state or clear relevant parts
                // before reloading to avoid accumulating state or errors.
                // For now, just reload the script.
                match script_engine_inner_clone.load_script(&init_script) {
                    Ok(_) => info!("Successfully reloaded {:?}", init_script),
                    Err(e) => error!("Failed to reload {:?}: {}", init_script, e),
                }
                // TODO: Signal GUI to update based on potential config changes
            }
        } else if changed_path
            .extension()
            .map_or(false, |ext| ext == "json") // Assuming themes are JSON
            && changed_path.starts_with(&config_dir_clone.parent().unwrap().join("themes"))
        {
            info!("Theme file changed: {:?}", changed_path);
            // TODO: Signal GUI to reload the specific theme or re-apply theme settings.
            // This might involve calling a Lua function like mavis.theme.set_theme(...)
            let theme_name = changed_path.file_stem().unwrap_or_default().to_string_lossy();
             if !theme_name.is_empty() {
                 match script_engine_inner_clone.eval::<()>(&format!("MAVIS.theme.set_theme('{}')", theme_name)) {
                     Ok(_) => info!("Applied theme '{}' via Lua", theme_name),
                     Err(e) => error!("Failed to apply theme '{}' via Lua: {}", theme_name, e),
                 }
             }
        }
    });
    // --- End Reload Callback ---

    config_watcher
        .start_watching()
        .context("Failed to start config watcher")?;
    info!("Configuration watcher started.");

    // 7. Initialize ConPTY Session (Attempting to launch LF)
    // TODO: Make command configurable and handle lf.exe not found gracefully.
    let conpty_command = "lf.exe"; // Target LF
    let conpty_session_arc = match ConPtySession::new(conpty_command, 80, 25) {
        Ok(session) => {
            info!("ConPTY session created successfully for '{}'.", conpty_command);
            Some(Arc::new(Mutex::new(session)))
        }
        Err(e) => {
            error!("Failed to create ConPTY session: {}", e);
            None // Continue without terminal functionality
        }
    };

    // 8. Setup ConPTY Output Reading Thread
    let (conpty_output_tx, conpty_output_rx) = mpsc::channel::<Vec<u8>>(); // Create channel
    let mut conpty_reader_thread: Option<thread::JoinHandle<()>> = None;

    if let Some(session_arc) = conpty_session_arc.clone() { // Clone Arc for the thread
        let tx_clone = conpty_output_tx.clone(); // Clone Sender for the thread

        conpty_reader_thread = Some(thread::spawn(move || {
            info!("ConPTY reader thread started.");
            let mut buffer = [0u8; 4096]; // Read buffer

            loop {
                // Lock the session mutex to access the read method
                let read_result = {
                    let mut session_guard = match session_arc.lock() {
                        Ok(guard) => guard,
                        Err(poisoned) => {
                            error!("ConPTY session mutex poisoned! Reader thread exiting.");
                            // Send an empty vec to signal error/exit? Or just break.
                            let _ = tx_clone.send(Vec::new()); // Signal exit/error
                            break;
                        }
                    };
                    // Use read method directly (blocking)
                     match session_guard.read(&mut buffer) {
                         Ok(0) => {
                             info!("ConPTY read 0 bytes, assuming process exited. Reader thread exiting.");
                             let _ = tx_clone.send(Vec::new()); // Signal exit
                             break; // EOF or process closed pipe
                         }
                         Ok(bytes_read) => {
                             debug!("ConPTY read {} bytes.", bytes_read);
                             Ok(buffer[..bytes_read].to_vec()) // Convert slice to Vec
                         }
                         Err(e) => {
                             error!("Error reading from ConPTY: {}. Reader thread exiting.", e);
                             let _ = tx_clone.send(Vec::new()); // Signal exit/error
                             break; // Propagate error or handle specific cases
                         }
                     }
                };

                // Send data if read was successful
                if let Ok(data) = read_result {
                    if data.is_empty() { // Check if we broke loop due to EOF/Error signal
                        break;
                    }
                    if tx_clone.send(data).is_err() {
                        info!("ConPTY output channel closed by receiver. Reader thread exiting.");
                        break; // Receiver likely dropped (GUI closed)
                    }
                } else {
                    // Error already logged, break loop
                    break;
                }

                // Optional: Add a small sleep to prevent tight loop if needed,
                // but blocking read should handle this.
                // thread::sleep(Duration::from_millis(10));
            }
            info!("ConPTY reader thread finished.");
        }));
        info!("ConPTY reader thread spawned.");
    }


    // 9. Initialize and Run the GUI
    let gui_state = Arc::new(Mutex::new(GuiState {
        conpty_session: conpty_session_arc, // Use the original Arc here
        conpty_output_rx: Some(conpty_output_rx), // Store the receiver end
        conpty_output_tx: Some(conpty_output_tx), // Store sender for potential input later
        ..GuiState::new()
    }));
    info!("Launching GUI...");

    // Pass the shared state to the GUI function.
    if let Err(e) = mavis_gui::run_gui(&core_config, gui_state.clone()) {
        error!("GUI exited with error: {}", e);
        // Perform any cleanup before exiting the application
    }

    // 10. Cleanup
    info!("MAVIS Shell shutting down.");
    // ConPtySession Drop handles termination.
    // Wait for reader thread to finish if it was started.
    if let Some(handle) = conpty_reader_thread {
        info!("Waiting for ConPTY reader thread to join...");
        let _ = handle.join().map_err(|e| error!("ConPTY reader thread panicked: {:?}", e));
        info!("ConPTY reader thread joined.");
    }

    Ok(())
}