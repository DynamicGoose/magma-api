use std::path::PathBuf;

use glam::IVec2;

use crate::window::{CursorPosition, WindowTheme};

/// An event for signaling a window resize.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct WindowResized {
    pub window: usize,
    pub width: u32,
    pub height: u32,
}

/// An event signaling a requested redraw of the whole application.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct RedrawRequested;

/// An event signaling a window has been created.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct WindowCreated {
    pub window: usize,
}

/// An event signaling that a window has been requested to close. It should live one more update cycle with a [`ClosingWindow`](crate::ClosingWindow)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct WindowCloseRequested {
    pub window: usize,
}

/// An event signaling a window has been closed. The corresponding entity will no longer exist at the time this is emmited.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct WindowClosed {
    pub window: usize,
}

/// An event signaling a window has been destroyed. The corresponding entity will no longer exist at the time this is emmited.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct WindowDestroyed {
    pub window: usize,
}

/// An event signaling that the cursor has moved within a window.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct CursorMoved {
    pub window: usize,
    pub position: CursorPosition,
}

/// An event signaling the cursor has entered a winbdow.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct CursorEntered {
    pub window: usize,
}

/// An event signaling the cursor has left a window.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct CursorLeft {
    pub window: usize,
}

/// An event signaling a focus change for a window.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct WindowFocused {
    pub window: usize,
    pub focus: bool,
}

/// An event signaling the window's occlusion has changed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum WindowOcclusion {
    /// The window is occluded by another window.
    Occluded { window: usize },
    /// The window is not occluded anymore.
    NotOccluded { window: usize },
}

/// An event signaling file drag and drops.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum FileDragDrop {
    /// A file has been dropped on a window.
    Dropped { window: usize, path: PathBuf },
    /// A file is hovering over a window.
    Hovered { window: usize, path: PathBuf },
    /// A file hover has been canceled.
    HoverCanceled { window: usize },
}

/// An event signaling that the window has moved.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct WindowMoved {
    pub window: usize,
    pub position: IVec2,
}

/// An event signaling that the window's theme variant changed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct WindowThemeChanged {
    pub window: usize,
    pub theme: WindowTheme,
}
