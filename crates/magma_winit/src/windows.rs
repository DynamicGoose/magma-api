use winit::event::{DeviceEvent, WindowEvent};

/// After adding the [`WinitModule`](crate::WinitModule) the [`Windows`] resource can be accessed.
#[derive(Default)]
pub struct Windows {
    pub windows: Vec<Option<winit::window::Window>>,
    pub window_events: Vec<WindowEvent>,
    pub device_events: Vec<DeviceEvent>,
    pub(crate) spawn: u32,
}

impl Windows {
    pub(crate) fn new() -> Self {
        Self {
            windows: vec![],
            window_events: vec![],
            device_events: vec![],
            spawn: 0,
        }
    }
    /**
    Spawn a specified amount of windows
    ```
    use magma_app::App;
    use magma_winit::{WinitModule, windows::Windows};

    let mut app = App::new();
    app.add_module(WinitModule);
    // spawn the window
    app.world
        .get_resource_mut::<Windows>().unwrap().spawn(1);
    ```
    */
    pub fn spawn(&mut self, num: u32) {
        self.spawn += num;
    }

    /// Despawn the window at the given index
    pub fn despawn(&mut self, index: usize) {
        self.windows[index] = None;
    }
}
