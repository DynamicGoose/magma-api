/**
The window component can be added to an entity to spawn a window.
*/
#[derive(Default)]
pub struct Window(pub Option<winit::window::Window>);

impl Window {
    pub fn new() -> Self {
        Self::default()
    }
}
