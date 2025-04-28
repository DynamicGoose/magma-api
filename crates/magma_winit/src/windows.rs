use std::collections::HashMap;

use winit::window::{Window as WinitWindow, WindowId};

/// After adding the [`WinitModule`](crate::WinitModule) the [`Windows`] resource can be accessed.
#[derive(Default)]
pub struct Windows {
    pub window_to_entity: HashMap<WindowId, usize>,
    pub entity_to_window: HashMap<usize, WindowId>,
    pub winit_windows: HashMap<WindowId, WinitWindow>,
}

impl Windows {
    pub fn new() -> Self {
        Self::default()
    }
}
