use magma_app::{magma_ecs::events::EventPoll, resources::ResMut};

use crate::{
    ButtonMap,
    input_event::{KeyboardInput, MouseButtonInput},
    keyboard::KeyCode,
    mouse::MouseButton,
};

pub fn update_keyboard_resource(
    mut button_map: ResMut<ButtonMap<KeyCode>>,
    events: EventPoll<KeyboardInput>,
) {
    // clear before processing new events
    button_map.clear();
    events.events.iter().for_each(|input| match input.state {
        crate::ButtonState::Pressed => button_map.press(input.key_code),
        crate::ButtonState::Released => button_map.release(input.key_code),
    });
}

pub fn update_mouse_resource(
    mut button_map: ResMut<ButtonMap<MouseButton>>,
    events: EventPoll<MouseButtonInput>,
) {
    // clear before processing new events
    button_map.clear();

    events.events.iter().for_each(|input| match input.state {
        crate::ButtonState::Pressed => button_map.press(input.button),
        crate::ButtonState::Released => button_map.release(input.button),
    });
}
