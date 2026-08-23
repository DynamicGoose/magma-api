use magma_app::magma_ecs::{events::Events, resource::ResMut};

use crate::{
    ButtonMap,
    input_event::{KeyboardInput, MouseButtonInput},
    keyboard::{Key, KeyCode},
    mouse::MouseButton,
};

pub fn update_keyboard_resource(
    mut keycode_map: ResMut<ButtonMap<KeyCode>>,
    mut key_map: ResMut<ButtonMap<Key>>,
    events: Events<KeyboardInput>,
) {
    keycode_map.clear();
    key_map.clear();
    for input in events.events {
        match input.state {
            crate::ButtonState::Pressed => {
                keycode_map.press(input.key_code);
                key_map.press(input.key);
            }
            crate::ButtonState::Released => {
                keycode_map.release(input.key_code);
                key_map.release(input.key);
            }
        }
    }
}

pub fn update_mouse_resource(
    mut button_map: ResMut<ButtonMap<MouseButton>>,
    events: Events<MouseButtonInput>,
) {
    button_map.clear();
    for input in events.events {
        match input.state {
            crate::ButtonState::Pressed => button_map.press(input.button),
            crate::ButtonState::Released => button_map.release(input.button),
        }
    }
}
