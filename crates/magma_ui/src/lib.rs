/*!
This crate provides the [`UIModule`], which can be added to your application along with an `update_ui`-system, which you must implement yourself.
For ease of use the crate also reexports [`iced`] and [`iced_winit`].
The [`UIModule`] also adds the [`WinitModule`] to your [`App`](magma_app::App), so you can draw your application to a window.
When using this with iced, make sure to read the [`iced_winit`] documentation for e.g. converting winit windows to iced windows.
*/

use magma_app::{module::Module, World};
use magma_winit::WinitModule;

pub use iced;
pub use iced_winit;

pub struct UIModule {
    pub update_ui: &'static dyn Fn(&mut World),
}

impl Module for UIModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.add_module(WinitModule);
        app.add_systems(
            magma_app::SystemType::Update,
            (vec![], vec![self.update_ui]),
        );
    }
}
