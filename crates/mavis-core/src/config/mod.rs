// Configuration management for MAVIS

mod loader;
mod watcher;

pub use loader::ConfigLoader;
pub use watcher::ConfigWatcher;

use crate::error::CoreError;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};

/// Main configuration struct that holds all MAVIS settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// General application settings
    pub general: GeneralConfig,
    
    /// User interface settings
    pub ui: UiConfig,
    
    /// Terminal settings
    pub terminal: TerminalConfig,
    
    /// Shell replacement settings
    pub shell: ShellConfig,
    
    /// Resource monitoring settings
    pub monitoring: MonitoringConfig,
    
    /// Security settings for Lua scripting
    pub security: SecurityConfig,
}

/// General application settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeneralConfig {
    /// Application log level
    pub log_level: String,
    
    /// User-friendly name
    pub user_name: String,
    
    /// Whether to start MAVIS on boot
    pub start_on_boot: bool,
    
    /// Path to store application data
    pub data_path: String,
}

/// User interface settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UiConfig {
    /// Active theme name
    pub theme: String,
    
    /// Font family
    pub font_family: String,
    
    /// Font size
    pub font_size: f32,
    
    /// DPI scaling factor
    pub scale_factor: f32,
    
    /// Whether to use Direct2D or GDI fallback
    pub use_direct2d: bool,
}

/// Terminal settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TerminalConfig {
    /// Default shell executable
    pub shell_executable: String,
    
    /// Terminal dimensions
    pub columns: u32,
    pub rows: u32,
    
    /// Scrollback buffer size
    pub scrollback_lines: u32,
    
    /// Terminal font family
    pub font_family: String,
    
    /// Terminal font size
    pub font_size: f32,
}

/// Shell replacement settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShellConfig {
    /// Shell replacement method (shelllauncherv2 or registry)
    pub replacement_method: String,
    
    /// Maximum number of crashes before fallback
    pub crash_threshold: u32,
    
    /// Time window for crash detection in seconds
    pub crash_window_seconds: u32,
    
    /// Whether to attempt automatic recovery
    pub auto_recovery: bool,
    
    /// Manual override key combination
    pub override_key_combo: String,
}

/// Resource monitoring settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MonitoringConfig {
    /// How often to update metrics in milliseconds
    pub update_interval_ms: u32,
    
    /// Enable CPU monitoring
    pub monitor_cpu: bool,
    
    /// Enable RAM monitoring
    pub monitor_ram: bool,
    
    /// Enable network monitoring
    pub monitor_network: bool,
    
    /// Enable disk monitoring
    pub monitor_disk: bool,
    
    /// Alert threshold for CPU usage (percentage)
    pub cpu_alert_threshold: u32,
    
    /// Alert threshold for RAM usage (percentage)
    pub ram_alert_threshold: u32,
    
    /// Alert threshold for disk usage (percentage)
    pub disk_alert_threshold: u32,
    
    /// Alert threshold for disk space (percentage free)
    pub disk_space_alert_threshold: u32,
}

/// Security settings for Lua scripting
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecurityConfig {
    /// Whether to enable Lua sandboxing
    pub enable_sandboxing: bool,
    
    /// Whether unsafe mode is enabled
    pub unsafe_mode: bool,
    
    /// List of allowed modules in Lua
    pub allowed_modules: Vec<String>,
}

impl Config {
    /// Create a new default configuration
    pub fn default() -> Self {
        Self {
            general: GeneralConfig {
                log_level: "info".to_string(),
                user_name: "User".to_string(),
                start_on_boot: false,
                data_path: "%LOCALAPPDATA%\\MAVIS".to_string(),
            },
            ui: UiConfig {
                theme: "default_dark".to_string(),
                font_family: "Segoe UI".to_string(),
                font_size: 14.0,
                scale_factor: 1.0,
                use_direct2d: true,
            },
            terminal: TerminalConfig {
                shell_executable: "cmd.exe".to_string(),
                columns: 80,
                rows: 24,
                scrollback_lines: 10000,
                font_family: "Cascadia Code".to_string(),
                font_size: 12.0,
            },
            shell: ShellConfig {
                replacement_method: "registry".to_string(),
                crash_threshold: 3,
                crash_window_seconds: 30,
                auto_recovery: true,
                override_key_combo: "Ctrl+Alt+Shift+F4".to_string(),
            },
            monitoring: MonitoringConfig {
                update_interval_ms: 500,
                monitor_cpu: true,
                monitor_ram: true,
                monitor_network: true,
                monitor_disk: true,
                cpu_alert_threshold: 90,
                ram_alert_threshold: 90,
                disk_alert_threshold: 95,
                disk_space_alert_threshold: 10,
            },
            security: SecurityConfig {
                enable_sandboxing: true,
                unsafe_mode: false,
                allowed_modules: vec![
                    "table".to_string(),
                    "string".to_string(),
                    "math".to_string(),
                ],
            },
        }
    }
    
    /// Save configuration to specified file
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), CoreError> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, self)
            .map_err(|e| CoreError::ConfigError(format!("Failed to save config: {}", e)))
    }
}

// Thread-safe shared configuration
pub type SharedConfig = Arc<Config>;