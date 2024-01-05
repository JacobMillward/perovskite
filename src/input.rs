use std::collections::HashMap;

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, Modifiers, MouseButton, WindowEvent},
    keyboard::{KeyCode, ModifiersKeyState, PhysicalKey},
    window::WindowId,
};

/// The state of keyboard modifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct KeyboardModifiers {
    /// Left "shift" key.
    pub left_shift: bool,
    /// Right "shift" key.
    pub right_shift: bool,
    /// Left "alt" key.
    pub left_alt: bool,
    /// Right "alt" key.
    pub right_alt: bool,
    /// Left "control" key.
    pub left_control: bool,
    /// Right "control" key.
    pub right_control: bool,
    /// Left "super" key. This is the "windows" key on PC and "command" key on Mac.
    pub left_super: bool,
    /// Right "super" key. This is the "windows" key on PC and "command" key on Mac.
    pub right_super: bool,
}

impl KeyboardModifiers {
    fn update(&mut self, mods: &Modifiers) {
        self.left_shift = mods.lshift_state() == ModifiersKeyState::Pressed;
        self.right_shift = mods.rshift_state() == ModifiersKeyState::Pressed;
        self.left_alt = mods.lalt_state() == ModifiersKeyState::Pressed;
        self.right_alt = mods.ralt_state() == ModifiersKeyState::Pressed;
        self.left_control = mods.lcontrol_state() == ModifiersKeyState::Pressed;
        self.right_control = mods.rcontrol_state() == ModifiersKeyState::Pressed;
        self.left_super = mods.lsuper_state() == ModifiersKeyState::Pressed;
        self.right_super = mods.rsuper_state() == ModifiersKeyState::Pressed;
    }
}

/// The state of a key or button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputState {
    /// The input was pressed this frame.
    Pressed,
    /// The input is currently held down.
    Down,
    /// The input was released this frame.
    Released,
}

impl From<ElementState> for InputState {
    fn from(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => Self::Pressed,
            ElementState::Released => Self::Released,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum InputType {
    Key(KeyCode),
    Mouse(MouseButton),
}

/// A helper struct for tracking keyboard input.
/// Stores the state of each key, and provides methods for querying the state of each key.
/// Make sure to call `handle_keyboard_event` with keyboard events from winit's event loop.
#[derive(Debug)]
pub struct InputManager {
    window_id: WindowId,
    input_map: HashMap<InputType, InputState>,
    key_modifiers: KeyboardModifiers,
    cursor_position: PhysicalPosition<f64>,
}

impl InputManager {
    /// Creates a new input manager.
    pub(crate) fn new(window_id: WindowId) -> Self {
        Self {
            window_id,
            input_map: HashMap::new(),
            key_modifiers: KeyboardModifiers::default(),
            cursor_position: PhysicalPosition::new(0.0, 0.0),
        }
    }

    /// Updates the input manager with events from winit's event loop.
    pub(crate) fn handle_event(&mut self, event: &winit::event::Event<()>) {
        match event {
            Event::WindowEvent { window_id, event } if *window_id == self.window_id => {
                match event {
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event,
                        is_synthetic: false,
                    } if !event.repeat => {
                        if let PhysicalKey::Code(key_code) = event.physical_key {
                            let input = InputType::Key(key_code);

                            match event.state {
                                ElementState::Pressed => match self.input_map.get(&input) {
                                    Some(&InputState::Released) | None => {
                                        self.input_map.insert(input, InputState::Pressed);
                                    }
                                    Some(&InputState::Pressed) | Some(&InputState::Down) => {
                                        self.input_map.insert(input, InputState::Down);
                                    }
                                },

                                ElementState::Released => {
                                    self.input_map.insert(input, InputState::Released);
                                }
                            }
                        }
                    }

                    WindowEvent::ModifiersChanged(mods) => {
                        self.key_modifiers.update(mods);
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        self.cursor_position = *position;
                    }

                    WindowEvent::MouseInput { state, button, .. } => {
                        let input = InputType::Mouse(*button);

                        match state {
                            ElementState::Pressed => match self.input_map.get(&input) {
                                Some(&InputState::Released) | None => {
                                    self.input_map.insert(input, InputState::Pressed);
                                }
                                Some(&InputState::Pressed) | Some(&InputState::Down) => {
                                    self.input_map.insert(input, InputState::Down);
                                }
                            },
                            ElementState::Released => {
                                self.input_map.insert(input, InputState::Released);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        };
    }

    /// Updates the state of input keys. Must be called once per render frame.
    pub(crate) fn update(&mut self) {
        self.input_map.retain(|_, state| match state {
            InputState::Pressed => {
                *state = InputState::Down;
                true
            }
            InputState::Released => false,
            InputState::Down => true,
        });
    }

    /// Returns true if the key was pressed this frame.
    pub fn key_pressed(&self, key_code: KeyCode) -> bool {
        self.input_map.get(&InputType::Key(key_code)) == Some(&InputState::Pressed)
    }

    /// Returns true if the key is currently down.
    /// Will return true for multiple frames if the key is held down, including the frame it was
    /// pressed.
    pub fn key_down(&self, key_code: KeyCode) -> bool {
        matches!(
            self.input_map.get(&InputType::Key(key_code)),
            Some(&InputState::Pressed) | Some(&InputState::Down)
        )
    }

    /// Returns true if the key was released this frame.
    pub fn key_released(&self, key_code: KeyCode) -> bool {
        self.input_map.get(&InputType::Key(key_code)) == Some(&InputState::Released)
    }

    /// Returns true if the mouse button was pressed this frame.
    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.input_map.get(&InputType::Mouse(button)) == Some(&InputState::Pressed)
    }

    /// Returns true if the mouse button is currently down.
    /// Will return true for multiple frames if the mouse button is held down, including the frame
    /// it was pressed.
    pub fn mouse_down(&self, button: MouseButton) -> bool {
        matches!(
            self.input_map.get(&InputType::Mouse(button)),
            Some(&InputState::Pressed) | Some(&InputState::Down)
        )
    }

    /// Returns true if the mouse button was released this frame.
    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.input_map.get(&InputType::Mouse(button)) == Some(&InputState::Released)
    }
}
