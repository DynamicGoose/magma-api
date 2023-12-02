use magma_app::module::Module;
use winit::{event_loop::EventLoop, window::Window};

pub struct WinitModule;

impl WinitModule {
    pub fn create_window() {
        let event_loop = EventLoop::new().unwrap();
        Window::new(&event_loop).unwrap();
    }
}

impl Module for WinitModule {
    fn setup(&self, _app: &mut magma_app::App) {}
}
