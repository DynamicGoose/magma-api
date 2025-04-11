pub struct Window {
    pub winit_window: bool,
}

impl Window {
    pub const fn has_winit_window(&self) -> bool {
        self.winit_window
    }
}
