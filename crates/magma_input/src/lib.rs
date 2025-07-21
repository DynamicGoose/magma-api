use magma_app::module::Module;

pub use button_state::ButtonState;

use crate::input_event::{KeyboardInput, MouseButtonInput, MouseMotionInput, MouseScrollInput};

mod button_state;
pub mod input_event;
pub mod keyboard;
pub mod mouse;

pub struct InputModule;

impl Module for InputModule {
    fn setup(self, app: &mut magma_app::App) {
        app.register_event::<KeyboardInput>();
        app.register_event::<MouseButtonInput>();
        app.register_event::<MouseScrollInput>();
        app.register_event::<MouseMotionInput>();
    }
}
