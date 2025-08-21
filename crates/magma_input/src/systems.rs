use magma_app::World;

use crate::{
    ButtonMap,
    input_event::{KeyboardInput, MouseButtonInput},
    keyboard::KeyCode,
    mouse::MouseButton,
};

pub fn update_keyboard_resource(world: &World) {
    // clear before processing new events
    world
        .get_resource_mut::<ButtonMap<KeyCode>>()
        .unwrap()
        .clear();

    world
        .poll_events::<KeyboardInput>()
        .unwrap()
        .iter()
        .for_each(|input| match input.state {
            crate::ButtonState::Pressed => world
                .get_resource_mut::<ButtonMap<KeyCode>>()
                .unwrap()
                .press(input.key_code),
            crate::ButtonState::Released => world
                .get_resource_mut::<ButtonMap<KeyCode>>()
                .unwrap()
                .release(input.key_code),
        });
}

pub fn update_mouse_resource(world: &World) {
    // clear before processing new events
    world
        .get_resource_mut::<ButtonMap<MouseButton>>()
        .unwrap()
        .clear();

    world
        .poll_events::<MouseButtonInput>()
        .unwrap()
        .iter()
        .for_each(|input| match input.state {
            crate::ButtonState::Pressed => world
                .get_resource_mut::<ButtonMap<MouseButton>>()
                .unwrap()
                .press(input.button),
            crate::ButtonState::Released => world
                .get_resource_mut::<ButtonMap<MouseButton>>()
                .unwrap()
                .release(input.button),
        });
}
