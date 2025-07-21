use magma_app::{World, events::Events};

use crate::{
    ButtonMap,
    input_event::{KeyboardInput, MouseButtonInput},
    keyboard::KeyCode,
    mouse::MouseButton,
};

pub fn update_keyboard_resource(world: &World) {
    world
        .get_resource::<Events>()
        .unwrap()
        .get_events::<KeyboardInput>()
        .unwrap()
        .iter()
        .for_each(
            |input| match input.downcast_ref::<KeyboardInput>().unwrap().state {
                crate::ButtonState::Pressed => world
                    .get_resource_mut::<ButtonMap<KeyCode>>()
                    .unwrap()
                    .press(input.downcast_ref::<KeyboardInput>().unwrap().key_code),
                crate::ButtonState::Released => world
                    .get_resource_mut::<ButtonMap<KeyCode>>()
                    .unwrap()
                    .release(input.downcast_ref::<KeyboardInput>().unwrap().key_code),
            },
        );
}

pub fn update_mouse_resource(world: &World) {
    world
        .get_resource::<Events>()
        .unwrap()
        .get_events::<MouseButtonInput>()
        .unwrap()
        .iter()
        .for_each(
            |input| match input.downcast_ref::<MouseButtonInput>().unwrap().state {
                crate::ButtonState::Pressed => world
                    .get_resource_mut::<ButtonMap<MouseButton>>()
                    .unwrap()
                    .press(input.downcast_ref::<MouseButtonInput>().unwrap().button),
                crate::ButtonState::Released => world
                    .get_resource_mut::<ButtonMap<MouseButton>>()
                    .unwrap()
                    .release(input.downcast_ref::<MouseButtonInput>().unwrap().button),
            },
        );
}
