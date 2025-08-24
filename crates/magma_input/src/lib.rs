use magma_app::module::Module;

pub use button_map::ButtonMap;
pub use button_state::ButtonState;

use crate::{
    input_event::{KeyboardInput, MouseButtonInput, MouseMotionInput, MouseScrollInput},
    keyboard::KeyCode,
    mouse::MouseButton,
    systems::{update_keyboard_resource, update_mouse_resource},
};

mod button_map;
mod button_state;
/// Events emmited by input devices
pub mod input_event;
/// Keyboard specific types
pub mod keyboard;
/// Mouse specific types
pub mod mouse;
mod systems;

/// The input module for the app
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct InputModule;

impl Module for InputModule {
    fn setup(self, app: &mut magma_app::App) {
        app.register_event::<KeyboardInput>();
        app.register_event::<MouseButtonInput>();
        app.register_event::<MouseScrollInput>();
        app.register_event::<MouseMotionInput>();
        app.add_event_systems::<KeyboardInput>(vec![(
            update_keyboard_resource,
            "keyboard_resource".to_string(),
            vec![],
        )])
        .unwrap();
        app.add_event_systems::<MouseButtonInput>(vec![(
            update_mouse_resource,
            "mouse_resource".to_string(),
            vec![],
        )])
        .unwrap();

        app.world
            .add_resource(ButtonMap::<KeyCode>::default())
            .unwrap();
        app.world
            .add_resource(ButtonMap::<MouseButton>::default())
            .unwrap();
    }
}
