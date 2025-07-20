use crate::{
    ButtonState,
    keyboard::{Key, KeyCode},
};
use magma_app::entities::Entity;

pub struct KeyboardInput {
    pub key: Key,
    pub key_code: KeyCode,
    // pub text: Option<&'static str>, - needs ime support #34
    pub state: ButtonState,
    pub repeat: bool,
    pub window: Entity,
}
