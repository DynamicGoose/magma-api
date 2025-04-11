use std::collections::HashMap;

use magma_app::entities::Entity;
use winit::window::{Window as WinitWindow, WindowId};

/// After adding the [`WinitModule`](crate::WinitModule) the [`Windows`] resource can be accessed.
#[derive(Default)]
pub(crate) struct Windows {
    pub window_to_entity: HashMap<WindowId, usize>,
    pub winit_windows: HashMap<WindowId, WinitWindow>,
}

impl Windows {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}
