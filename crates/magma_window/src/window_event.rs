use std::path::PathBuf;

use magma_app::magma_ecs::entities::Entity;
use magma_math::IVec2;

use crate::window::WindowTheme;

/// An event for signaling a window resize.
pub struct WindowResized {
    pub window: Entity,
    pub width: u32,
    pub height: u32,
}

/// An event signaling a requested redraw of the whole application.
pub struct RedrawRequested;

/// An event signaling a window has been created.
pub struct WindowCreated {
    pub window: Entity,
}

/// An event signaling that a window has been requested to close. It should live one more update cycle with a [`ClosingWindow`](crate::ClosingWindow)
pub struct WindowCloseRequested {
    pub window: Entity,
}

/// An event signaling a window has been closed. The corresponding entity will no longer exist at the time this is emmited.
pub struct WindowClosed {
    pub window: Entity,
}

/// An event signaling a window has been destroyed. The corresponding entity will no longer exist at the time this is emmited.
pub struct WindowDestroyed;

/// An event signaling that the cursor has moved within a window.
pub struct CursorMoved {
    pub window: Entity,
    pub position: IVec2,
}

/// An event signaling the cursor has entered a winbdow.
pub struct CursorEntered {
    pub window: Entity,
}

/// An event signaling the cursor has left a window.
pub struct CursorLeft {
    pub window: Entity,
}

/// An event signaling a focus change for a window.
pub struct WindowFocused {
    pub window: Entity,
    pub focus: bool,
}

/// An event signaling the window's occlusion has changed.
pub enum WindowOcclusion {
    /// The window is occluded by another window.
    Occluded { window: Entity },
    /// The window is not occluded anymore.
    NotOccluded { window: Entity },
}

/// An event signaling file drag and drops.
pub enum FileDragDrop {
    /// A file has been dropped on a window.
    Dropped { window: Entity, path: PathBuf },
    /// A file is hovering over a window.
    Hovered { window: Entity, path: PathBuf },
    /// A file hover has been canceled.
    HoverCanceled { window: Entity },
}

/// An event signaling that the window has moved.
pub struct WindowMoved {
    pub window: Entity,
    pub position: IVec2,
}

/// An event signaling that the window's theme variant changed.
pub struct WindowThemeChanged {
    pub window: Entity,
    pub theme: WindowTheme,
}
