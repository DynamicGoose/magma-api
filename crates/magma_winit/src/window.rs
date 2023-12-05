#[derive(Default)]
pub struct Window(pub Option<winit::window::Window>);

impl Window {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct Windows(pub Vec<Window>);

impl Windows {
    pub fn add_window(&mut self) {
        self.0.push(Window::new());
    }
}
