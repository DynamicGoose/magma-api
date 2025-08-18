use std::time::Instant;

use feufeu::{
    RenderState,
    wgpu::{Surface, SurfaceConfiguration, SurfaceTargetUnsafe},
};
use magma_app::{App, entities::Entity, module::Module, rayon::join};
use magma_windowing::Window;
use magma_winit::{WinitModule, WrappedApp};
use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
};

use crate::{
    render_stages::background::BackgroundStage,
    sync_module::{EntityRenderEntityMap, SyncModule, SyncToRenderWorld},
};

pub mod render_stages;
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
        self.app
            .app
            .world
            .query::<(Window,)>()
            .unwrap()
            .iter()
            .for_each(|window_entity| {
                let mut map = self
                    .app
                    .app
                    .world
                    .get_resource_mut::<EntityRenderEntityMap>()
                    .unwrap();
                let render_state = self.app.app.world.get_resource::<RenderState>().unwrap();
                match map.entity_to_render_entity.get(&window_entity.into()) {
                    Some(_) => (),
                    None => {
                        window_entity
                            .assign_components((SyncToRenderWorld,))
                            .unwrap();
                        let window_entity: Entity = window_entity.into();
                        let window = self
                            .app
                            .windows
                            .winit_windows
                            .get(
                                self.app
                                    .windows
                                    .entity_to_window
                                    .get(&window_entity)
                                    .unwrap(),
                            )
                            .unwrap();
                        render_state
                            .render_world
                            .query::<(Surface, SurfaceConfiguration)>()
                            .unwrap()
                            .iter()
                            .for_each(|e| e.delete());
                        let surface = unsafe {
                            render_state
                                .get_instance()
                                .create_surface_unsafe(SurfaceTargetUnsafe::RawHandle {
                                    raw_display_handle: window.display_handle().unwrap().as_raw(),
                                    raw_window_handle: window.window_handle().unwrap().as_raw(),
                                })
                                .unwrap()
                        };
                        let surface_caps = surface.get_capabilities(render_state.get_adapter());
                        let surface_format = surface_caps
                            .formats
                            .iter()
                            .find(|f| f.is_srgb())
                            .copied()
                            .unwrap_or(surface_caps.formats[0]);
                        let surface_config = feufeu::wgpu::SurfaceConfiguration {
                            usage: feufeu::wgpu::TextureUsages::RENDER_ATTACHMENT,
                            format: surface_format,
                            width: window.inner_size().width,
                            height: window.inner_size().height,
                            present_mode: surface_caps.present_modes[0],
                            alpha_mode: surface_caps.alpha_modes[0],
                            view_formats: vec![],
                            desired_maximum_frame_latency: 2,
                        };
                        surface.configure(render_state.get_device(), &surface_config);

                        let render_entity = render_state
                            .render_world
                            .create_entity((sync_module::RenderEntity, surface, surface_config))
                            .unwrap();
                        map.insert(window_entity, render_entity);
                    }
                }

                // render_state
                //     .render_world
                //     .query::<(Surface, SurfaceConfiguration)>()
                //     .unwrap()
                //     .iter()
                //     .for_each(|render_entity| {
                //         let entity = map
                //             .render_entity_to_entity
                //             .get(&render_entity.into())
                //             .unwrap()
                //             .to_owned();
                //         if self.app.app.world.get_component::<Window>(entity).is_err() {
                //             map.delete_through_render_entity(&render_entity.into());
                //             render_entity.delete();
                //         }
                //     });
            });

        join(
            || self.app.app.update(),
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
