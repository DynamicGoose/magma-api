use winit::{event::Event, event_loop::EventLoop, window::Window};

/**
The window component can be added to an entity to spawn a window.
*/
pub struct Windows {
    pub windows: Vec<Option<winit::window::Window>>,
    pub event_loop: EventLoop<()>,
    pub event: Event<()>,
}

impl Windows {
    pub fn new(event_loop: EventLoop<()>) -> Self {
        Self {
            windows: vec![],
            event_loop,
            event: Event::AboutToWait,
        }
    }
    pub fn spawn(&mut self) {
        if let Some(none) = self.windows.iter_mut().find(|window| window.is_none()) {
            *none = Some(Window::new(&self.event_loop).unwrap());
        } else {
            self.windows
                .push(Some(winit::window::Window::new(&self.event_loop).unwrap()));
        }
    }
    pub fn despawn(&mut self, index: usize) {
        self.windows[index] = None;
    }
}
