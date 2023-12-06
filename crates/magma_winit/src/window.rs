/// The window struct represents a window.
#[derive(Default)]
pub struct Window(pub Option<winit::window::Window>);

impl Window {
    pub fn new() -> Self {
        Self::default()
    }
}

/// This is added as a resource to the `App`'s `World` when the WinitModule is added and gives access to the `App`'s windows.
pub struct Windows(pub Vec<Window>);

impl Windows {
    /// Add a new window to your `App`
    pub fn add_window(&mut self) {
        self.0.push(Window::new());
    }
}
