use magma_app::{App, module::Module};
pub use window::{ClosingWindow, Window};
use window_event::*;

/// Provides the [`Window`] component and related types.
pub mod window;
/// Window related events
pub mod window_event;

/// The Window module for the App
pub struct WindowModule;

impl Module for WindowModule {
    fn setup(self, app: &mut App) {
        app.world.register_component::<Window>();
        app.world.register_component::<ClosingWindow>();

        app.register_event::<WindowResized>();
        app.register_event::<RedrawRequested>();
        app.register_event::<WindowCreated>();
        app.register_event::<WindowCloseRequested>();
        app.register_event::<WindowClosed>();
        app.register_event::<WindowDestroyed>();
        app.register_event::<CursorMoved>();
        app.register_event::<CursorEntered>();
        app.register_event::<CursorLeft>();
        app.register_event::<WindowFocused>();
        app.register_event::<WindowOcclusion>();
        app.register_event::<FileDragDrop>();
        app.register_event::<WindowMoved>();
        app.register_event::<WindowThemeChanged>();
    }
}
