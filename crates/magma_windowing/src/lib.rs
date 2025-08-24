use magma_app::{App, module::Module, schedule::PostUpdate};
pub use monitor::{Monitor, PrimaryMonitor};
pub use window::{ClosingWindow, Window};
use window_event::*;

use crate::systems::{delete_pending_windows, focused, mark_closed_windows, moved, resized};

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
    fn setup(self, app: &mut App) {
        app.world.register_component::<Monitor>();
        app.world.register_component::<PrimaryMonitor>();
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

        app.add_systems::<PostUpdate>(vec![(
            delete_pending_windows,
            "delete_pending_windows".to_string(),
            vec![],
        )])
        .unwrap();

        app.add_event_systems::<WindowCloseRequested>(vec![(
            mark_closed_windows,
            "mark_closing_windows".to_string(),
            vec![],
        )])
        .unwrap();

        app.add_event_systems::<WindowResized>(vec![(
            resized,
            "resized_windows".to_string(),
            vec![],
        )])
        .unwrap();

        app.add_event_systems::<WindowMoved>(vec![(moved, "moved_windows".to_string(), vec![])])
            .unwrap();

        app.add_event_systems::<WindowFocused>(vec![(
            focused,
            "focused_windows".to_string(),
            vec![],
        )])
        .unwrap();
    }
}
