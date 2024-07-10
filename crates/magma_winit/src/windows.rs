use winit::{event::Event, event_loop::EventLoop};

/// After adding the [`WinitModule`](crate::WinitModule) the [`Windows`] resource can be accessed.
pub struct Windows<'a> {
    pub windows: Vec<Option<&'a winit::window::Window>>,
    pub event_loop: &'a EventLoop<()>,
    pub events: Vec<&'a Event<()>>,
}

impl<'a> Windows<'a> {
    /// create a new instance of [`Windows`]
    pub fn new(event_loop: &'a EventLoop<()>) -> Self {
        Self {
            windows: vec![],
            event_loop,
            events: vec![],
        }
    }
    /**
    Spawn a new window
    ```
    use magma_app::App;
    use magma_winit::{WinitModule, windows::Windows};

    let mut app = App::new();
    app.add_module(WinitModule);
    app.world.get_resource_mut::<Windows>().unwrap().spawn();
    ```
    */
    pub fn add_window(&mut self, window: &'a winit::window::Window) {
        if let Some(none) = self.windows.iter_mut().find(|window| window.is_none()) {
            *none = Some(window);
        } else {
            self.windows.push(Some(window));
        }
    }

    /// Despawn the window at the given index
    pub fn despawn(&mut self, index: usize) {
        self.windows[index] = None;
    }
}
