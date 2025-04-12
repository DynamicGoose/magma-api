use std::path::PathBuf;

use magma_math::IVec2;

use crate::window::WindowTheme;

pub struct WindowResized {
    pub window: usize,
    pub width: u32,
    pub height: u32,
}

pub struct RequestRedraw;

pub struct WindowCreated {
    pub window: usize,
}

pub struct WindowCloseRequested {
    pub window: usize,
}

pub struct WindowClosed {
    pub window: usize,
}

pub struct WindowDestroyed {
    pub window: usize,
}

pub struct CursorMoved {
    pub window: usize,
    pub position: IVec2,
}

pub struct CursorEntered {
    pub window: usize,
}

pub struct CursorLeft {
    pub window: usize,
}

pub struct WindowFocused {
    pub window: usize,
}

pub enum WindowOcclusion {
    Occluded { window: usize },
    NotOccluded { window: usize },
}

pub enum FileDragDrop {
    Dropped { window: usize, path: PathBuf },
    Hovered { window: usize, path: PathBuf },
    HoverCanceled { window: usize },
}

pub struct WindowMoved {
    pub window: usize,
    pub position: IVec2,
}

pub struct WindowThemeChanged {
    pub window: usize,
    pub theme: WindowTheme,
}

pub struct WindowAttributesChanged {
    pub window: usize,
}
