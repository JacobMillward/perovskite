use std::time::Duration;

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
}

impl RenderContext {
    /// Create new Context
    pub fn new(window: Window, target_frame_time: Duration, max_frame_time: Duration) -> Self {
        let id = window.id();
        Self {
            window,
            target_frame_time,
            max_frame_time,
            should_exit: false,
            delta_time: Duration::from_secs(0),
            input: InputManager::new(id),
        }
    }

    /// Get the window
    pub fn window(&self) -> &Window {
        &self.window
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
