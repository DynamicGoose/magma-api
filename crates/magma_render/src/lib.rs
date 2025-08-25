use feufeu::RenderState;
use magma_app::{
    App, AppSchedule,
    module::Module,
    rayon::join,
    schedule::{PostUpdate, PreUpdate, Startup, Update},
};
use magma_windowing::Window;
use magma_winit::{WinitModule, WrappedApp};
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    render_stages::background::BackgroundStage, sync_entity::SyncEntityModule,
    sync_window::SyncWindowsModule,
};

pub mod render_stages;
pub mod sync_component;
pub mod sync_entity;
pub mod sync_window;

pub struct RenderModule;

impl Module for RenderModule {
    fn setup(self, app: &mut magma_app::App) {
        app.add_module(WinitModule);
        app.set_runner(rendering_update_loop);
        app.world.add_resource(RenderState::default()).unwrap();
        app.world.add_resource(Renderer(default_renderer)).unwrap();
        app.register_schedule::<SyncSchedule>();
        app.add_module(SyncEntityModule);
        app.add_module(SyncWindowsModule);
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
        self.app
            .app
            .world
            .get_resource_mut::<RenderState>()
            .unwrap()
            .init_stage::<BackgroundStage>()
            .unwrap();
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

        self.app.app.run_schedule::<SyncSchedule>().unwrap();

        join(
            || {
                self.app.app.run_schedule::<PreUpdate>().unwrap();
                self.app.app.run_schedule::<Update>().unwrap();
                self.app.app.run_schedule::<PostUpdate>().unwrap();
                self.app.app.process_events();
            },
            || {
                (self.app.app.world.get_resource::<Renderer>().unwrap().0)(
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
    app.app.app.run_schedule::<Startup>().unwrap();
    event_loop.run_app(&mut app).unwrap();
}

fn default_renderer(render_state: &RenderState) {
    let now = Instant::now();
    render_state.run_stage::<BackgroundStage>().unwrap();
    println!(
        "{}",
        1.0 / ((1.0 / 1000000.0) * now.elapsed().as_micros() as f32)
    );
}

pub struct SyncSchedule;

impl AppSchedule for SyncSchedule {}
