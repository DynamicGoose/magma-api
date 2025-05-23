//! The Magma-API crate is a container crate to combine all the featrues that are in seperate crates.
pub use magma_app;
pub use magma_math;
// pub use magma_audio;
// pub use magma_ui;
// pub use magma_winit;

use magma_app::module::Module;
// use magma_audio::AudioModule;
// use magma_winit::WinitModule;

pub struct DefaultModules;

impl Module for DefaultModules {
    fn setup(self, _app: &mut magma_app::App) {
        // app.add_module(WinitModule);
        // app.add_module(AudioModule);
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
