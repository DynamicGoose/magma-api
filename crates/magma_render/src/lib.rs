use feufeu::RenderState;
use magma_app::{App, module::Module};
use magma_windowing::Window;
use magma_winit::{WinitModule, WrappedApp};
use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
};

pub struct RenderModule;

impl Module for RenderModule {
    fn setup(self, app: &mut magma_app::App) {
        app.add_module(WinitModule);
        app.set_runner(rendering_update_loop);
    }
}

pub struct RenderApp {
    winit_app: WrappedApp,
    render_state: RenderState<'static, 'static>,
    renderer: fn(&RenderState),
}

impl RenderApp {
    pub fn new(app: App, init: fn(&mut RenderState), renderer: fn(&RenderState)) -> Self {
        let mut app = Self {
            winit_app: WrappedApp::new(app),
            render_state: RenderState::default(),
            renderer,
        };
        init(&mut app.render_state);

        app
    }
}

impl ApplicationHandler for RenderApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.winit_app.resumed(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.winit_app.window_event(event_loop, window_id, event);
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.winit_app.device_event(event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.winit_app.about_to_wait(event_loop);
        (self.renderer)(&self.render_state);
    }
}

fn rendering_update_loop(app: App) {
    app.world.create_entity((Window::new(),)).unwrap();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = RenderApp::new(app, init_default_renderer, default_renderer);
    event_loop.run_app(&mut app).unwrap();
}

fn init_default_renderer(_render_state: &mut RenderState) {}

fn default_renderer(_render_state: &RenderState) {}
