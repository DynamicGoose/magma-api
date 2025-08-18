use feufeu::RenderState;
use magma_app::{App, module::Module, rayon::join};
use magma_windowing::Window;
use magma_winit::{WinitModule, WrappedApp};
use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
};

use crate::sync_module::SyncModule;

pub mod sync_component_module;
pub mod sync_module;

pub struct RenderModule;

impl Module for RenderModule {
    fn setup(self, app: &mut magma_app::App) {
        app.add_module(WinitModule);
        app.set_runner(rendering_update_loop);
        app.world.add_resource(RenderState::default()).unwrap();
        app.world.add_resource(Renderer(default_renderer)).unwrap();
        app.add_module(SyncModule);
    }
}

pub struct RenderApp {
    pub app: WrappedApp,
}

impl RenderApp {
    pub fn new(app: App) -> Self {
        Self {
            app: WrappedApp::new(app),
        }
    }
}

pub struct Renderer(fn(&RenderState));

impl Renderer {
    pub fn new(renderer: fn(&RenderState)) -> Self {
        Self(renderer)
    }
}

impl ApplicationHandler for RenderApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.resumed(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.app.window_event(event_loop, window_id, event);
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.app.device_event(event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.winit_update(event_loop);
        join(
            || self.app.app.update(),
            || {
                self.app.app.world.get_resource::<Renderer>().unwrap().0(
                    &self.app.app.world.get_resource::<RenderState>().unwrap(),
                )
            },
        );
    }
}

fn rendering_update_loop(app: App) {
    app.world.create_entity((Window::new(),)).unwrap();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = RenderApp::new(app);
    event_loop.run_app(&mut app).unwrap();
}

fn default_renderer(_render_state: &RenderState) {
    println!("ha");
}
