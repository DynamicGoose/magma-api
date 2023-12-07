/**
The window component can be added to an entity to spawn a window.
Spawning a window in a running application isn't supported, but you can create a new `App` for that.
*/
#[derive(Default)]
pub struct Window(pub Option<winit::window::Window>);

impl Window {
    pub fn new() -> Self {
        Self::default()
    }
}
