use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuiError {
    #[error("Core error: {0}")]
    CoreError(#[from] mavis_core::error::CoreError),

    #[error("Windowing system error (winit): {0}")]
    WindowError(#[from] winit::error::OsError),

    #[error("ImGui error: {0}")]
    ImGuiError(String),

    #[error("Renderer error: {0}")]
    RendererError(String), // Generic renderer error for now

    #[error("Initialization error: {0}")]
    InitializationError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Font loading error: {0}")]
    FontError(String),

    #[error("Image loading error: {0}")]
    ImageError(String),

    #[error("Windows API error: {0:?}")]
    WindowsError(#[from] windows::core::Error),
}

// Helper type alias for Result using GuiError
pub type GuiResult<T> = Result<T, GuiError>;

// For imgui crate
impl From<String> for GuiError {
    fn from(err: String) -> Self {
        GuiError::ImGuiError(err)
    }
}

// No need for separate renderer error conversion since we're using a String already
// The imgui_dx11_renderer doesn't expose a public RendererError type

// Add other specific error conversions as needed, e.g., for font-kit
impl From<font_kit::error::SelectionError> for GuiError {
    fn from(err: font_kit::error::SelectionError) -> Self {
        GuiError::FontError(format!("Font selection error: {}", err))
    }
}

// Replace LoadError with FontLoadingError which is the correct type in font-kit
impl From<font_kit::error::FontLoadingError> for GuiError {
    fn from(err: font_kit::error::FontLoadingError) -> Self {
        GuiError::FontError(format!("Font loading error: {}", err))
    }
}

// Add conversion for stb_image error
// The stb_image crate doesn't have an error module with StbImageError
// Instead of implementing From<std::io::Error> again, let's use a custom function
// since we already have #[from] std::io::Error in the IoError variant
pub fn image_io_error(err: std::io::Error) -> GuiError {
    GuiError::ImageError(format!("Image I/O error: {}", err))
}