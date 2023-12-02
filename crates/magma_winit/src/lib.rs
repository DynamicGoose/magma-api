use magma_app::{default_runner, module::Module, App};
use winit::{event_loop::EventLoop, window::Window};

pub struct WinitModule;

impl WinitModule {
    pub fn create_window() {
        let event_loop = EventLoop::new().unwrap();
        Window::new(&event_loop).unwrap();
    }
}

impl Module for WinitModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.runner = &winit_event_loop;
    }
}

fn winit_event_loop(app: App) {
    let event_loop = EventLoop::new().unwrap();
    Window::new(&event_loop).unwrap();
    default_runner(app);
}
