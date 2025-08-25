use std::num::NonZero;

use feufeu::{
    RenderState,
    wgpu::{
        Surface, SurfaceConfiguration, SurfaceTexture, TextureFormat, TextureUsages, TextureView,
    },
};
use magma_app::{
    World,
    entities::Entity,
    module::Module,
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
};
use magma_windowing::{
    Window,
    raw_handle::RawHandleWrapper,
    window::{AlphaMode, PresentMode},
    window_event::{WindowClosed, WindowCreated},
};
use magma_winit::windows::Windows;

use crate::{
    SyncSchedule,
    sync_component::SyncComponent,
    sync_entity::{MainEntity, RenderEntity, SyncToRenderWorld},
};

pub struct SyncWindowsModule;

impl Module for SyncWindowsModule {
    fn setup(self, app: &mut magma_app::App) {
        app.add_systems::<SyncSchedule>(vec![(
            sync_windows,
            "sync_windows".to_string(),
            vec!["sync_entities".to_string()],
        )])
        .unwrap();
        let mut render_state = app.world.get_resource_mut::<RenderState>().unwrap();

        render_state
            .render_world
            .register_component::<RenderWindow>();
        render_state
            .render_world
            .register_component::<SurfaceData>();
    }
}

#[derive(Clone, Debug)]
pub struct RenderWindow {
    pub raw_handle: RawHandleWrapper,
    pub physical_width: u32,
    pub physical_height: u32,
    pub present_mode: PresentMode,
    pub desired_maximum_frame_latency: Option<NonZero<u32>>,
    pub texture_view: Option<TextureView>,
    pub texture: Option<SurfaceTexture>,
    pub texture_format: Option<TextureFormat>,
    pub alpha_mode: AlphaMode,
    pub size_changed: bool,
    pub present_mode_changed: bool,
    pub frame_latency_changed: bool,
    pub alpha_mode_changed: bool,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct SurfaceData {
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
}

impl SyncComponent for Window {
    type Out = RenderWindow;

    fn get_data(entity: Entity, world: &World) -> Self::Out {
        let windows = world.get_resource::<Windows>().unwrap();
        let window = world.get_component::<Window>(entity).unwrap();
        RenderWindow {
            raw_handle: RawHandleWrapper::new(
                windows
                    .winit_windows
                    .get(windows.entity_to_window.get(&entity.into()).unwrap())
                    .unwrap(),
            ),
            physical_width: window.resolution.width(),
            physical_height: window.resolution.height(),
            present_mode: window.present_mode,
            desired_maximum_frame_latency: window.desired_maximum_frame_latency,
            texture_view: None,
            texture: None,
            texture_format: None,
            alpha_mode: window.alpha_mode,
            size_changed: false,
            present_mode_changed: false,
            frame_latency_changed: false,
            alpha_mode_changed: false,
        }
    }
}

pub(crate) fn sync_windows(world: &World) {
    let render_state = world.get_resource::<RenderState>().unwrap();

    // sync new windows
    world
        .poll_events::<WindowCreated>()
        .unwrap()
        .iter()
        .for_each(
            |event| match world.get_component::<RenderEntity>(event.window) {
                Ok(render_entity) => {
                    let mut extracted_window = Window::get_data(event.window, world);
                    let surface = unsafe {
                        render_state
                            .get_instance()
                            .create_surface_unsafe(feufeu::wgpu::SurfaceTargetUnsafe::RawHandle {
                                raw_display_handle: extracted_window
                                    .raw_handle
                                    .get_display_handle(),
                                raw_window_handle: extracted_window.raw_handle.get_window_handle(),
                            })
                            .expect("Failed to create render surface")
                    };

                    let caps = surface.get_capabilities(render_state.get_adapter());
                    let texture_format = *caps.formats.iter().find(|f| f.is_srgb()).unwrap_or(
                        caps.formats
                            .first()
                            .expect("No supported texture formats for surface"),
                    );
                    let surface_config = SurfaceConfiguration {
                        usage: TextureUsages::RENDER_ATTACHMENT,
                        format: texture_format,
                        width: extracted_window.physical_width,
                        height: extracted_window.physical_height,
                        present_mode: match extracted_window.present_mode {
                            PresentMode::Vsync => feufeu::wgpu::PresentMode::AutoVsync,
                            PresentMode::NoVsync => feufeu::wgpu::PresentMode::AutoNoVsync,
                            PresentMode::Fifo => feufeu::wgpu::PresentMode::Fifo,
                            PresentMode::RelaxedFifo => feufeu::wgpu::PresentMode::FifoRelaxed,
                            PresentMode::Mailbox => feufeu::wgpu::PresentMode::Mailbox,
                            PresentMode::Immediate => feufeu::wgpu::PresentMode::Immediate,
                        },
                        desired_maximum_frame_latency: extracted_window
                            .desired_maximum_frame_latency
                            .map(NonZero::<u32>::get)
                            .unwrap_or(2),
                        alpha_mode: match extracted_window.alpha_mode {
                            AlphaMode::Auto => feufeu::wgpu::CompositeAlphaMode::Auto,
                            AlphaMode::Opaque => feufeu::wgpu::CompositeAlphaMode::Opaque,
                            AlphaMode::PreMultiplied => {
                                feufeu::wgpu::CompositeAlphaMode::PreMultiplied
                            }
                            AlphaMode::PostMultiplied => {
                                feufeu::wgpu::CompositeAlphaMode::PostMultiplied
                            }
                            AlphaMode::Inherit => feufeu::wgpu::CompositeAlphaMode::Inherit,
                        },
                        view_formats: if texture_format.is_srgb() {
                            vec![texture_format.add_srgb_suffix()]
                        } else {
                            vec![]
                        },
                    };

                    surface.configure(render_state.get_device(), &surface_config);

                    extracted_window.texture_format = Some(texture_format);

                    world
                        .get_resource::<RenderState>()
                        .unwrap()
                        .render_world
                        .assign_components(
                            (
                                Window::get_data(event.window, world),
                                SurfaceData {
                                    surface,
                                    config: surface_config,
                                },
                            ),
                            render_entity.entity(),
                        )
                        .unwrap()
                }
                Err(_) => (),
            },
        );

    // sync closed windows
    world
        .poll_events::<WindowClosed>()
        .unwrap()
        .iter()
        .for_each(
            |event| match world.get_component::<RenderEntity>(event.window) {
                Ok(render_entity) => {
                    render_state
                        .render_world
                        .purge_component::<RenderWindow>(render_entity.entity())
                        .unwrap();
                    render_state
                        .render_world
                        .purge_component::<SurfaceData>(render_entity.entity())
                        .unwrap();
                }
                Err(_) => {
                    match render_state
                        .render_world
                        .query::<(MainEntity,)>()
                        .unwrap()
                        .iter()
                        .find(|e| {
                            e.get_component::<MainEntity>().unwrap().id() == event.window.id()
                        }) {
                        Some(render_entity) => {
                            render_entity.purge_component::<RenderWindow>().unwrap();
                            render_entity.purge_component::<SurfaceData>().unwrap();
                        }
                        None => (),
                    }
                }
            },
        );

    // sync changes
    world
        .query::<(SyncToRenderWorld, Window)>()
        .unwrap()
        .par_iter()
        .for_each(|entity| {
            let window = entity.get_component::<Window>().unwrap();
            let render_entity = entity.get_component::<RenderEntity>().unwrap().entity();
            let mut render_window = render_state
                .render_world
                .get_component_mut::<RenderWindow>(render_entity)
                .unwrap();
            let mut surface_data = render_state
                .render_world
                .get_component_mut::<SurfaceData>(render_entity)
                .unwrap();

            render_window.size_changed = false;
            render_window.present_mode_changed = false;

            if render_window.physical_width != window.resolution.width()
                || render_window.physical_height != window.resolution.height()
            {
                render_window.physical_width = window.resolution.width();
                render_window.physical_height = window.resolution.height();
                render_window.size_changed = true;

                surface_data.config.width = render_window.physical_width;
                surface_data.config.height = render_window.physical_height;
            }

            if render_window.present_mode != window.present_mode {
                render_window.present_mode = window.present_mode;
                render_window.present_mode_changed = true;

                surface_data.config.present_mode = match render_window.present_mode {
                    PresentMode::Vsync => feufeu::wgpu::PresentMode::AutoVsync,
                    PresentMode::NoVsync => feufeu::wgpu::PresentMode::AutoNoVsync,
                    PresentMode::Fifo => feufeu::wgpu::PresentMode::Fifo,
                    PresentMode::RelaxedFifo => feufeu::wgpu::PresentMode::FifoRelaxed,
                    PresentMode::Mailbox => feufeu::wgpu::PresentMode::Mailbox,
                    PresentMode::Immediate => feufeu::wgpu::PresentMode::Immediate,
                };
            }

            if render_window.desired_maximum_frame_latency != window.desired_maximum_frame_latency {
                render_window.desired_maximum_frame_latency = window.desired_maximum_frame_latency;
                render_window.frame_latency_changed = true;

                surface_data.config.desired_maximum_frame_latency = render_window
                    .desired_maximum_frame_latency
                    .map(NonZero::<u32>::get)
                    .unwrap_or(2);
            }

            if render_window.alpha_mode != window.alpha_mode {
                render_window.alpha_mode = window.alpha_mode;
                render_window.alpha_mode_changed = true;

                surface_data.config.alpha_mode = match render_window.alpha_mode {
                    AlphaMode::Auto => feufeu::wgpu::CompositeAlphaMode::Auto,
                    AlphaMode::Opaque => feufeu::wgpu::CompositeAlphaMode::Opaque,
                    AlphaMode::PreMultiplied => feufeu::wgpu::CompositeAlphaMode::PreMultiplied,
                    AlphaMode::PostMultiplied => feufeu::wgpu::CompositeAlphaMode::PostMultiplied,
                    AlphaMode::Inherit => feufeu::wgpu::CompositeAlphaMode::Inherit,
                }
            }

            if render_window.size_changed
                || render_window.present_mode_changed
                || render_window.frame_latency_changed
                || render_window.alpha_mode_changed
            {
                surface_data
                    .surface
                    .configure(render_state.get_device(), &surface_data.config);
            }

            // update swapchain texture
            let texture = surface_data
                .surface
                .get_current_texture()
                .expect("Failed to get surface swapchain texture");
            let texture_view =
                texture
                    .texture
                    .create_view(&feufeu::wgpu::wgt::TextureViewDescriptor {
                        format: Some(texture.texture.format().add_srgb_suffix()),
                        ..Default::default()
                    });

            render_window.texture = Some(texture);
            render_window.texture_view = Some(texture_view);
        });
}
