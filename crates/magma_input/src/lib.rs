use magma_app::{module::Module, schedule::PreUpdate};

pub use button_map::ButtonMap;
pub use button_state::ButtonState;

use crate::{
    input_event::{KeyboardInput, MouseButtonInput, MouseMotionInput, MouseScrollInput},
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
    fn init(self, app: &mut magma_app::App) {
        app.world.event_manager.register_event::<KeyboardInput>();
        app.world.event_manager.register_event::<MouseButtonInput>();
        app.world.event_manager.register_event::<MouseScrollInput>();
        app.world.event_manager.register_event::<MouseMotionInput>();
        app.add_system(PreUpdate, update_keyboard_resource).unwrap();
        app.add_system(PreUpdate, update_mouse_resource).unwrap();
    }
}
