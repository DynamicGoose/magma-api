use crate::{
    ButtonState,
    keyboard::{Key, KeyCode},
    mouse::{MouseButton, MouseScrollUnit},
};
use magma_app::entities::Entity;
use magma_math::Vec2;

/// Event emmited for keyboard input
pub struct KeyboardInput {
    /// key name
    pub key: Key,
    /// key code
    pub key_code: KeyCode,
    // pub text: Option<&'static str>, - needs ime support #34
    /// The current state of the key
    pub state: ButtonState,
    /// Sometimes a key being held down for a period of time causes it to be repeated. This will be true if that is the case.
    pub repeat: bool,
    /// The window which emmited the event
    pub window: Entity,
}

/// Event emmited when a mouse button is pressed
pub struct MouseButtonInput {
    /// The button of the mouse
    pub button: MouseButton,
    /// The current state of the button
    pub state: ButtonState,
    /// Sometimes a key being held down for a period of time causes it to be repeated. This will be true if that is the case.
    pub repeat: bool,
    /// The window which emmited the event
    pub window: Entity,
}

/// Event emmited when the mouse scroll wheel is used
pub struct MouseScrollInput {
    /// The unit of the scroll event. Can be either in pixels or lines.
    pub unit: MouseScrollUnit,
    /// The horizontal value
    pub x: f32,
    /// The vertical value
    pub y: f32,
    /// The window which emmited the event
    pub window: Entity,
}

/// Event emmited when the mouse is moved
pub struct MouseMotionInput {
    /// The movement delta
    pub delta: Vec2,
    /// The window which emmited the event
    pub window: Entity,
}
