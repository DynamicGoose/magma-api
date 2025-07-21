use crate::{
    ButtonState,
    keyboard::{Key, KeyCode},
    mouse::{MouseButton, MouseScrollUnit},
};
use magma_app::entities::Entity;
use magma_math::Vec2;

pub struct KeyboardInput {
    pub key: Key,
    pub key_code: KeyCode,
    // pub text: Option<&'static str>, - needs ime support #34
    pub state: ButtonState,
    pub repeat: bool,
    pub window: Entity,
}

pub struct MouseButtonInput {
    pub button: MouseButton,
    pub state: ButtonState,
    pub repeat: bool,
    pub window: Entity,
}

pub struct MouseScrollInput {
    pub unit: MouseScrollUnit,
    pub x: f32,
    pub y: f32,
    pub window: Entity,
}

pub struct MouseMotionInput {
    pub delta: Vec2,
    pub window: Entity,
}
