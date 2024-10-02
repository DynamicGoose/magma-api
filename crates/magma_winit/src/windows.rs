use winit::event::{DeviceEvent, WindowEvent};

/// After adding the [`WinitModule`](crate::WinitModule) the [`Windows`] resource can be accessed.
#[derive(Default)]
pub struct Windows {
    pub windows: Vec<Option<winit::window::Window>>,
    pub window_events: Vec<WindowEvent>,
    pub device_events: Vec<DeviceEvent>,
    pub(crate) spawn: bool,
}

impl Windows {
    /// create a new instance of [`Windows`]
    pub fn new() -> Self {
        Self {
            windows: vec![],
            window_events: vec![],
            device_events: vec![],
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
    app.world.resources_write().get_mut::<Windows>().unwrap().spawn();
    ```
    */
    pub fn spawn(&mut self) {
        self.spawn = true;
    }

    /// Despawn the window at the given index
    pub fn despawn(&mut self, index: usize) {
        self.windows[index] = None;
    }
}
