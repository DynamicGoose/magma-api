//! The Magma-API crate is a container crate to combine all the featrues that are in seperate crates.
pub use magma_app;
pub use magma_audio;
pub use magma_ui;
use magma_ui::UIModule;
pub use magma_winit;

use magma_app::module::Module;
use magma_winit::WinitModule;

pub struct DefaultModules;

impl Module for DefaultModules {
    fn setup(&self, app: &mut magma_app::App) {
        app.add_module(WinitModule);
        app.add_module(UIModule);
    }
}

#[cfg(test)]
mod tests {}
