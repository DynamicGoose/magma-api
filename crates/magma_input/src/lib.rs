use magma_app::module::Module;

pub use button_map::ButtonMap;
pub use button_state::ButtonState;

use crate::{
    input_event::{KeyboardInput, MouseButtonInput, MouseMotionInput, MouseScrollInput},
    keyboard::KeyCode,
    mouse::MouseButton,
};

mod button_map;
mod button_state;
/// Events emmited by input devices
pub mod input_event;
/// Keyboard specific types
pub mod keyboard;
/// Mouse specific types
pub mod mouse;

/// The input module for the app
pub struct InputModule;

impl Module for InputModule {
    fn setup(self, app: &mut magma_app::App) {
        app.register_event::<KeyboardInput>();
        app.register_event::<MouseButtonInput>();
        app.register_event::<MouseScrollInput>();
        app.register_event::<MouseMotionInput>();

        app.world
            .add_resource(ButtonMap::<KeyCode>::default())
            .unwrap();
        app.world
            .add_resource(ButtonMap::<MouseButton>::default())
            .unwrap();
    }
}
