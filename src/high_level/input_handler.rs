#![allow(unused)]

use nalgebra::Vector2;
use smallvec::SmallVec;
use std::{borrow::Borrow, hash::Hash};
use winit::{
    dpi::LogicalPosition, DeviceEvent, ElementState, Event, KeyboardInput, MouseButton,
    MouseScrollDelta, VirtualKeyCode, WindowEvent,
};

pub struct InputHandler {
    pressed_keys: SmallVec<[(VirtualKeyCode, u32); 12]>,
    pressed_mouse_buttons: SmallVec<[MouseButton; 12]>,
    mouse_position: Option<(f32, f32)>,
    mouse_delta: Vector2<f32>,
    mouse_wheel_vertical: f32,
    mouse_wheel_horizontal: f32,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            pressed_keys: Default::default(),
            pressed_mouse_buttons: Default::default(),
            mouse_position: Default::default(),
            mouse_delta: Vector2::new(0.0, 0.0),
            mouse_wheel_vertical: Default::default(),
            mouse_wheel_horizontal: Default::default(),
        }
    }

    pub fn send_event(&mut self, event: &Event, hidpi: f32) {
        match *event {
            Event::WindowEvent { ref event, .. } => match *event {
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button,
                    ..
                } => {
                    let mouse_button = button;
                    if self
                        .pressed_mouse_buttons
                        .iter()
                        .all(|&b| b != mouse_button)
                    {
                        self.pressed_mouse_buttons.push(mouse_button);
                    }
                }
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    button,
                    ..
                } => {
                    let mouse_button = button;
                    let index = self
                        .pressed_mouse_buttons
                        .iter()
                        .position(|&b| b == mouse_button);
                    if let Some(i) = index {
                        self.pressed_mouse_buttons.swap_remove(i);
                    }
                }
                WindowEvent::CursorMoved {
                    position: LogicalPosition { x, y },
                    ..
                } => {
                    self.mouse_position = Some(((x as f32) * hidpi, (y as f32) * hidpi));
                }
                WindowEvent::Focused(false) => {
                    self.pressed_keys.clear();
                    self.pressed_mouse_buttons.clear();
                    self.mouse_position = None;
                }
                _ => {}
            },
            Event::DeviceEvent { ref event, .. } => match *event {
                DeviceEvent::MouseMotion { delta } => {
                    self.mouse_delta += Vector2::new(delta.0 as f32, delta.1 as f32)
                }
                DeviceEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(delta_x, delta_y),
                } => {
                    if delta_x != 0.0 {
                        self.mouse_wheel_horizontal = delta_x.signum();
                    }
                    if delta_y != 0.0 {
                        self.mouse_wheel_vertical = delta_y.signum();
                    }
                }
                DeviceEvent::MouseWheel {
                    delta: MouseScrollDelta::PixelDelta(LogicalPosition { x, y }),
                } => {
                    if x != 0.0 {
                        self.mouse_wheel_horizontal = x.signum() as f32;
                    }
                    if y != 0.0 {
                        self.mouse_wheel_vertical = y.signum() as f32;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    /// This function is to be called whenever a frame begins. It resets some input values.
    ///
    /// The `InputSystem` will call this automatically. If you're using that system, you
    /// don't need to call this function.
    pub fn send_frame_begin(&mut self) {
        self.mouse_delta = Vector2::new(0.0, 0.0);
        self.mouse_wheel_vertical = 0.0;
        self.mouse_wheel_horizontal = 0.0;
    }

    /// Returns an iterator over all keys that are down.
    pub fn keys_that_are_down(&self) -> impl Iterator<Item = VirtualKeyCode> + '_ {
        self.pressed_keys.iter().map(|k| k.0)
    }

    /// Checks if a key is down.
    pub fn key_is_down(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.iter().any(|&k| k.0 == key)
    }

    /// Returns an iterator over all pressed mouse buttons
    pub fn mouse_buttons_that_are_down(&self) -> impl Iterator<Item = &MouseButton> {
        self.pressed_mouse_buttons.iter()
    }

    /// Checks if a mouse button is down.
    pub fn mouse_button_is_down(&self, mouse_button: MouseButton) -> bool {
        self.pressed_mouse_buttons
            .iter()
            .any(|&mb| mb == mouse_button)
    }

    /// If the mouse wheel was scrolled this frame this function will return the direction it was
    /// scrolled.
    ///
    /// If "horizontal" is true this will return the horizontal mouse value. You almost always want
    /// the vertical mouse value.
    pub fn mouse_wheel_value(&self, horizontal: bool) -> f32 {
        if horizontal {
            self.mouse_wheel_horizontal
        } else {
            self.mouse_wheel_vertical
        }
    }

    pub fn mouse_movement_delta(&self) -> Vector2<f32> {
        self.mouse_delta
    }

    /// Returns an iterator over all pressed scan codes
    pub fn scan_codes_that_are_down(&self) -> impl Iterator<Item = u32> + '_ {
        self.pressed_keys.iter().map(|k| k.1)
    }

    /// Checks if the key corresponding to a scan code is down.
    pub fn scan_code_is_down(&self, scan_code: u32) -> bool {
        self.pressed_keys.iter().any(|&k| k.1 == scan_code)
    }

    /// Gets the current mouse position.
    ///
    /// this method can return None, either if no mouse is connected, or if no mouse events have
    /// been recorded
    pub fn mouse_position(&self) -> Option<(f32, f32)> {
        self.mouse_position
    }
}
