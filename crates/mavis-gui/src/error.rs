use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuiError {
    #[error("Core error: {0}")]
    CoreError(#[from] mavis_core::error::CoreError),

    #[error("Windowing system error (winit): {0}")]
    WindowError(#[from] winit::error::OsError),

    #[error("ImGui error: {0}")]
    ImGuiError(#[from] imgui::Error),

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

// Specific conversion for the renderer error type if known
// Example for imgui_dx11_renderer - adjust if using a different renderer
impl From<imgui_dx11_renderer::RendererError> for GuiError {
    fn from(err: imgui_dx11_renderer::RendererError) -> Self {
        GuiError::RendererError(format!("DirectX 11 Renderer Error: {}", err))
    }
}

// Add other specific error conversions as needed, e.g., for font-kit
impl From<font_kit::error::SelectionError> for GuiError {
    fn from(err: font_kit::error::SelectionError) -> Self {
        GuiError::FontError(format!("Font selection error: {}", err))
    }
}
impl From<font_kit::error::LoadError> for GuiError {
     fn from(err: font_kit::error::LoadError) -> Self {
         GuiError::FontError(format!("Font loading error: {}", err))
     }
}

// Add conversion for stb_image error
impl From<stb_image::error::StbImageError> for GuiError {
    fn from(err: stb_image::error::StbImageError) -> Self {
        GuiError::ImageError(format!("Image loading error: {}", err))
    }
}