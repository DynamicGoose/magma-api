use winit::{event::Event, event_loop::{self, EventLoop}, window::Window};

/// After adding the [`WinitModule`](crate::WinitModule) the [`Windows`] resource can be accessed.
pub struct Windows {
    pub windows: Vec<Option<winit::window::Window>>,
    pub events: Vec<Event<()>>,
    pub(crate) spawn: bool,
}

impl Windows {
    /// create a new instance of [`Windows`]
    pub fn new() -> Self {
        Self {
            windows: vec![],
            events: vec![],
            spawn: false,
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
    pub fn spawn(&mut self) {
        self.spawn = true;
        if let Some(none) = self.windows.iter_mut().find(|window| window.is_none()) {
            *none = Some(window);
        } else {
            self.windows
                .push(Some(window));
        }
    }

    /// Despawn the window at the given index
    pub fn despawn(&mut self, index: usize) {
        self.windows[index] = None;
    }
}
