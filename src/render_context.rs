use std::time::Duration;

use anyhow::{Context, Result};
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

use crate::InputManager;

/// Update context
#[derive(Debug)]
pub struct RenderContext {
    window: Window,
    target_frame_time: Duration,
    max_frame_time: Duration,
    should_exit: bool,
    pub(crate) delta_time: Duration,
    pub input: InputManager,
    pixels: Pixels,
}

impl RenderContext {
    /// Create new Context
    pub fn new(
        window: Window,
        target_frame_time: Duration,
        max_frame_time: Duration,
        pixel_buffer_width: u32,
        pixel_buffer_height: u32,
    ) -> Result<Self> {
        let id = window.id();

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(pixel_buffer_width, pixel_buffer_height, surface_texture)
        }
        .with_context(|| "Failed to create pixels context")?;

        Ok(Self {
            window,
            target_frame_time,
            max_frame_time,
            should_exit: false,
            delta_time: Duration::from_secs(0),
            input: InputManager::new(id),
            pixels,
        })
    }

    /// Get the window
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Get the window (mutable)
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn pixels_mut(&mut self) -> &mut Pixels {
        &mut self.pixels
    }

    /// Get the target frame time
    pub fn target_frame_time(&self) -> Duration {
        self.target_frame_time
    }

    /// Get the maximum frame time
    pub fn max_frame_time(&self) -> Duration {
        self.max_frame_time
    }

    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    /// Set if the app should exit
    pub fn exit(&mut self) {
        self.should_exit = true;
    }
}
