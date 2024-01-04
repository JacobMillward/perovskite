use std::collections::HashMap;

use winit::{
    event::ElementState,
    keyboard::{KeyCode, PhysicalKey},
};

/// The state of a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyState {
    /// The key was pressed this frame.
    Pressed,
    /// The key is currently held down.
    Down,
    /// The key was released this frame.
    Released,
}

/// A helper struct for tracking keyboard input.
/// Stores the state of each key, and provides methods for querying the state of each key.
/// Make sure to call `handle_keyboard_event` with keyboard events from winit's event loop.
pub struct InputManager {
    key_map: HashMap<KeyCode, KeyState>,
}

impl InputManager {
    /// Creates a new input manager.
    pub fn new() -> Self {
        Self {
            key_map: HashMap::new(),
        }
    }

    /// Updates the input manager with key information. Should be updated via information from
    /// winit's event loop.
    pub fn handle_keyboard_event(&mut self, physical_key: PhysicalKey, state: ElementState) {
        if let PhysicalKey::Code(key_code) = physical_key {
            match state {
                ElementState::Pressed => {
                    let value = self.key_map.entry(key_code).or_insert(KeyState::Released);
                    if let KeyState::Released = value {
                        *value = KeyState::Pressed;
                    } else {
                        *value = KeyState::Down;
                    }
                }
                ElementState::Released => {
                    self.key_map.insert(key_code, KeyState::Released);
                }
            }
        }
    }

    /// Returns true if the key was pressed this frame.
    pub fn key_pressed(&self, key_code: KeyCode) -> bool {
        self.key_map.get(&key_code).unwrap_or(&KeyState::Released) == &KeyState::Pressed
    }

    /// Returns true if the key is currently down.
    /// Will return true for multiple frames if the key is held down, including the frame it was
    /// pressed.
    pub fn key_down(&self, key_code: KeyCode) -> bool {
        let value = self.key_map.get(&key_code).unwrap_or(&KeyState::Released);
        value == &KeyState::Pressed || value == &KeyState::Down
    }

    /// Returns true if the key was released this frame.
    pub fn key_released(&self, key_code: KeyCode) -> bool {
        self.key_map.get(&key_code).unwrap_or(&KeyState::Released) == &KeyState::Released
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_keyboard_event() {
        let mut input_manager = InputManager::new();

        // Verify that the key is marked as released
        assert!(!input_manager.key_pressed(KeyCode::KeyA));
        assert!(!input_manager.key_down(KeyCode::KeyA));
        assert!(input_manager.key_released(KeyCode::KeyA));

        // Simulate a key press event
        input_manager
            .handle_keyboard_event(PhysicalKey::Code(KeyCode::KeyA), ElementState::Pressed);

        // Verify that the key is marked as pressed
        assert!(input_manager.key_pressed(KeyCode::KeyA));
        assert!(input_manager.key_down(KeyCode::KeyA));
        assert!(!input_manager.key_released(KeyCode::KeyA));

        // Simulate a key release event
        input_manager
            .handle_keyboard_event(PhysicalKey::Code(KeyCode::KeyA), ElementState::Released);

        // Verify that the key is marked as released
        assert!(!input_manager.key_pressed(KeyCode::KeyA));
        assert!(!input_manager.key_down(KeyCode::KeyA));
        assert!(input_manager.key_released(KeyCode::KeyA));
    }
}
