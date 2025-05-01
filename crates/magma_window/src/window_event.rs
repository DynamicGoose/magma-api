use std::path::PathBuf;

use magma_app::entities::Entity;
use magma_math::IVec2;

use crate::window::WindowTheme;

pub struct WindowResized {
    pub window: Entity,
    pub width: u32,
    pub height: u32,
}

pub struct RedrawRequested;

pub struct WindowCreated {
    pub window: Entity,
}

pub struct WindowCloseRequested {
    pub window: Entity,
}

pub struct WindowClosed {
    pub window: Entity,
}

pub struct WindowDestroyed {
    pub window: Entity,
}

pub struct CursorMoved {
    pub window: Entity,
    pub position: IVec2,
}

pub struct CursorEntered {
    pub window: Entity,
}

pub struct CursorLeft {
    pub window: Entity,
}

pub struct WindowFocused {
    pub window: Entity,
}

pub enum WindowOcclusion {
    Occluded { window: Entity },
    NotOccluded { window: Entity },
}

pub enum FileDragDrop {
    Dropped { window: Entity, path: PathBuf },
    Hovered { window: Entity, path: PathBuf },
    HoverCanceled { window: Entity },
}

pub struct WindowMoved {
    pub window: Entity,
    pub position: IVec2,
}

pub struct WindowThemeChanged {
    pub window: Entity,
    pub theme: WindowTheme,
}
