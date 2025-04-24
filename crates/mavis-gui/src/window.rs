use crate::error::{GuiError, GuiResult};
use crate::renderer; // Placeholder
use crate::state::GuiState; // Placeholder
use crate::ui; // Placeholder for UI drawing functions

use imgui::{Context, FontConfig, FontSource};
use imgui_dx11_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use log::{debug, error, info};
use mavis_core::config::Config as CoreConfig; // Alias to avoid name clash
use std::{sync::{Arc, Mutex}, time::Instant}; // Add Arc, Mutex
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use windows::Win32::Foundation::HWND;

/// Main entry point to run the MAVIS GUI.
pub fn run_gui(core_config: &CoreConfig, gui_state: Arc<Mutex<GuiState>>) -> GuiResult<()> {
    info!("Starting MAVIS GUI...");

    // 1. Initialize Winit event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("MAVIS Shell")
        .with_inner_size(LogicalSize::new(1280, 720))
        .build(&event_loop)?;

    let hwnd = HWND(window.hwnd()); // Get HWND for renderer

    // 2. Initialize ImGui
    let mut imgui = Context::create();
    imgui.set_ini_filename(None); // Disable ini file for now

    // --- Font Loading ---
    // TODO: Load fonts based on theme/config from mavis-core
    let font_size = 16.0; // Example size
    imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../assets/fonts/Roboto-Regular.ttf"), // Embed a default font
        size_pixels: font_size,
        config: Some(FontConfig {
            rasterizer_multiply: 1.5, // Adjust for better rendering if needed
            ..Default::default()
        }),
    }]);
    // --- End Font Loading ---

    // 3. Initialize Winit platform support
    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);

    // 4. Initialize Renderer (DirectX 11)
    let mut renderer = unsafe { Renderer::new(&mut imgui, hwnd)? };

    // 5. GUI State is passed in via Arc<Mutex<GuiState>>

    let mut last_frame = Instant::now();

    // 6. Main Event Loop
    info!("Entering main event loop...");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Use Poll for continuous rendering

        match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                // Prepare ImGui frame
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Start new ImGui frame
                let ui = imgui.frame();

                // --- Draw UI ---
                // Call the main UI drawing function, locking the shared state
                { // Scope for mutex guard
                    let mut state_guard = gui_state.lock().expect("Failed to lock GuiState mutex");
                    ui::draw_ui(&ui, &mut state_guard, core_config);

                    // Check if UI requested exit
                    if state_guard.should_exit {
                        info!("UI requested exit.");
                        *control_flow = ControlFlow::Exit;
                    }
                } // Mutex guard dropped here
                // --- End Draw UI ---

                // Render the frame
                renderer
                    .render(ui.render())
                    .expect("Failed to render frame");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                info!("Close requested, exiting...");
                *control_flow = ControlFlow::Exit;
            }
            // Handle other window events (resize, input, etc.)
            event => {
                platform.handle_event(imgui.io_mut(), &window, &event);
                // Handle resize specifically for the renderer
                if let Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    ..
                } = event
                {
                    if new_size.width > 0 && new_size.height > 0 {
                        // TODO: Handle renderer resize if necessary (DX11 might handle it via swap chain)
                        debug!("Window resized to: {:?}", new_size);
                    }
                }
            }
        }
    });

    // Note: EventLoop::run never returns normally.
    // Cleanup logic might need to be handled via other means if needed before exit.
}

// Helper trait to get HWND from winit window (requires raw-window-handle feature)
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
trait HwndExt {
    fn hwnd(&self) -> isize;
}

impl HwndExt for Window {
    fn hwnd(&self) -> isize {
        match self.raw_window_handle() {
            RawWindowHandle::Win32(handle) => handle.hwnd as isize,
            _ => panic!("Unsupported window handle type"),
        }
    }
}