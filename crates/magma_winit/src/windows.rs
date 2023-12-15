use winit::{event::Event, event_loop::EventLoop, window::Window};

/**
After adding the `WinitModule` the `Windows` resource can be accessed.
*/
pub struct Windows {
    pub windows: Vec<Option<winit::window::Window>>,
    pub event_loop: EventLoop<()>,
    pub events: Vec<Event<()>>,
}

impl Windows {
    /// create a new instance of `Windows`
    pub fn new(event_loop: EventLoop<()>) -> Self {
        Self {
            windows: vec![],
            event_loop,
            events: vec![],
        }
    }
    /**
    spawn a new window
    ```
    use magma_app::App;
    use magma_winit::{WinitModule, windows::Windows};

    let mut app = App::new();
    app.add_module(WinitModule);
    app.world.get_resource_mut::<Windows>().unwrap().spawn();
    ```
    */
    pub fn spawn(&mut self) {
        if let Some(none) = self.windows.iter_mut().find(|window| window.is_none()) {
            *none = Some(Window::new(&self.event_loop).unwrap());
        } else {
            self.windows
                .push(Some(winit::window::Window::new(&self.event_loop).unwrap()));
        }
    }
    /// despawn the window at the given index
    pub fn despawn(&mut self, index: usize) {
        self.windows[index] = None;
    }
}
