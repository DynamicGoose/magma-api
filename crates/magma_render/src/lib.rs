use magma_app::{
    App, Module,
    magma_ecs::rayon::join,
    schedule::{PostUpdate, PreUpdate, ScheduleLabel, Startup, Update},
};
use magma_windowing::Window;
use magma_winit::{
    WinitAppState, WinitModule,
    winit::{
        application::ApplicationHandler,
        event_loop::{ControlFlow, EventLoop},
    },
};

use crate::{
    sync_entities::{MainEntity, SyncEntityModule, SyncToRenderWorld},
    sync_windows::{RenderWindow, SurfaceData, SyncWindowsModule},
};

pub use render_state::RenderState;

pub mod render_state;
pub mod sync_component;
pub mod sync_entities;
pub mod sync_windows;

pub struct RenderModule;

impl Module for RenderModule {
    fn init(self, app: &mut App) {
        app.add_module(WinitModule);
        app.set_runner(rendering_runner);
        app.world.resource_store.insert(RenderState::default());
        // app.world.resource_store.insert(Renderer(renderer));
        app.register_schedule(RenderSync);
        app.add_module(SyncEntityModule);
        app.add_module(SyncWindowsModule);
    }
}

pub struct RenderAppState {
    app_state: WinitAppState,
}

impl RenderAppState {
    pub fn new(app: App) -> Self {
        Self {
            app_state: WinitAppState::new(app),
        }
    }
}

impl ApplicationHandler for RenderAppState {
    fn resumed(&mut self, event_loop: &magma_winit::winit::event_loop::ActiveEventLoop) {
        let render_state = self
            .app_state
            .app
            .world
            .resource_store
            .get_mut::<RenderState>()
            .unwrap();

        render_state
            .render_world
            .component_store
            .register_component::<MainEntity>();
        render_state
            .render_world
            .component_store
            .register_component::<RenderWindow>();
        render_state
            .render_world
            .component_store
            .register_component::<SurfaceData>();
        self.app_state.resumed(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &magma_winit::winit::event_loop::ActiveEventLoop,
        window_id: magma_winit::winit::window::WindowId,
        event: magma_winit::winit::event::WindowEvent,
    ) {
        self.app_state.window_event(event_loop, window_id, event);
    }

    fn device_event(
        &mut self,
        event_loop: &magma_winit::winit::event_loop::ActiveEventLoop,
        device_id: magma_winit::winit::event::DeviceId,
        event: magma_winit::winit::event::DeviceEvent,
    ) {
        self.app_state.device_event(event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &magma_winit::winit::event_loop::ActiveEventLoop) {
        self.app_state.winit_update(event_loop);
        self.app_state.app.run_schedule(RenderSync);
        let mut render_state = self
            .app_state
            .app
            .world
            .resource_store
            .remove::<RenderState>()
            .unwrap();
        join(
            || {
                self.app_state.app.run_schedule(PreUpdate);
                self.app_state.app.run_schedule(Update);
                self.app_state.app.run_schedule(PostUpdate);
                self.app_state.app.world.event_manager.clear();
            },
            || {
                (render_state.renderer)(&mut render_state);
            },
        );

        self.app_state.app.world.resource_store.insert(render_state);
    }
}

fn rendering_runner(mut app: App) {
    app.world
        .component_store
        .create_entity((Window::new(), SyncToRenderWorld))
        .unwrap();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = RenderAppState::new(app);
    app.app_state.app.run_schedule(Startup);
    event_loop.run_app(&mut app).unwrap();
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct RenderSync;

impl ScheduleLabel for RenderSync {}
