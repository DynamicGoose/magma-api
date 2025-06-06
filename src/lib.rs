//! The Magma-API crate is a container crate to combine all the featrues that are in seperate crates.
pub use magma_app;
pub use magma_math;
// pub use magma_audio;
// pub use magma_ui;
#[cfg(feature = "input")]
pub use magma_input;
#[cfg(feature = "windowing")]
pub use magma_windowing;
#[cfg(feature = "winit")]
pub use magma_winit;

use magma_app::module::Module;
// use magma_audio::AudioModule;
// use magma_winit::WinitModule;

pub struct DefaultModules;

impl Module for DefaultModules {
    fn setup(self, app: &mut magma_app::App) {
        #[cfg(feature = "input")]
        app.add_module(magma_input::InputModule);
        #[cfg(feature = "windowing")]
        app.add_module(magma_windowing::WindowingModule);
        #[cfg(feature = "winit")]
        app.add_module(magma_winit::WinitModule);
    }
}

#[cfg(test)]
mod tests {
    use magma_app::App;

    #[test]
    fn create_app() {
        let _app = App::new();
    }
}
