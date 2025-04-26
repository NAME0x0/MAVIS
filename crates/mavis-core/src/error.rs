use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Theme error: {0}")]
    ThemeError(String),
    
    #[error("Lua error: {0}")]
    LuaError(#[from] mlua::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Filesystem monitoring error: {0}")]
    // TODO: Consider wrapping the specific error type from the `notify` crate if needed.
    NotifyError(String),

    #[error("PDH error: {0}")]
    PdhError(String), // Added for Performance Data Helper errors

    #[error("Windows API error: {0:?}")]
    WindowsError(#[from] windows::core::Error),
    
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Initialization error: {0}")]
    InitializationError(String),
}

/// A specialized `Result` type for MAVIS core operations.
pub type CoreResult<T> = Result<T, CoreError>;