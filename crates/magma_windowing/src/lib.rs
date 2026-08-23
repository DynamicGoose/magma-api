use magma_app::{App, module::Module, schedule::PostUpdate};
pub use monitor::{Monitor, PrimaryMonitor};
pub use window::{ClosingWindow, Window};
use window_event::*;

use crate::systems::delete_pending_windows;

/// ECS Monitor representation
pub mod monitor;
/// Thread safe window handles
pub mod raw_handle;
/// Provides the [`Window`] component and related types.
pub mod window;
/// Window related events
pub mod window_event;

mod systems;

/// The Window module for the App
pub struct WindowingModule;

impl Module for WindowingModule {
    fn init(self, app: &mut App) {
        app.world.component_store.register_component::<Monitor>();
        app.world
            .component_store
            .register_component::<PrimaryMonitor>();
        app.world.component_store.register_component::<Window>();
        app.world
            .component_store
            .register_component::<ClosingWindow>();

        app.world.event_manager.register_event::<WindowResized>();
        app.world.event_manager.register_event::<RedrawRequested>();
        app.world.event_manager.register_event::<WindowCreated>();
        app.world
            .event_manager
            .register_event::<WindowCloseRequested>();
        app.world.event_manager.register_event::<WindowClosed>();
        app.world.event_manager.register_event::<WindowDestroyed>();
        app.world.event_manager.register_event::<CursorMoved>();
        app.world.event_manager.register_event::<CursorEntered>();
        app.world.event_manager.register_event::<CursorLeft>();
        app.world.event_manager.register_event::<WindowFocused>();
        app.world.event_manager.register_event::<WindowOcclusion>();
        app.world.event_manager.register_event::<FileDragDrop>();
        app.world.event_manager.register_event::<WindowMoved>();
        app.world
            .event_manager
            .register_event::<WindowThemeChanged>();

        app.add_system(PostUpdate, delete_pending_windows).unwrap();
    }
}
