use std::num::NonZero;

use magma_app::{
    Module,
    magma_ecs::{
        Component,
        events::OnEvents,
        query::Query,
        resource::{Res, ResMut},
        systems::IntoSystem,
        world::UnsafeWorldMut,
    },
};
use magma_windowing::{
    Window,
    raw_handle::RawHandleWrapper,
    window::{AlphaMode, PresentMode},
    window_event::{WindowClosed, WindowCreated},
};
use magma_winit::windows::Windows;
use wgpu::{
    CurrentSurfaceTexture::{Lost, Occluded, Outdated, Suboptimal, Success, Timeout, Validation},
    Surface, SurfaceConfiguration, SurfaceTexture, TextureFormat, TextureView,
};

use crate::{
    RenderSync,
    render_state::RenderState,
    sync_component::SyncComponent,
    sync_entities::{MainEntity, RenderEntity},
};

pub struct SyncWindowsModule;

impl Module for SyncWindowsModule {
    fn init(self, app: &mut magma_app::App) {
        app.add_system(RenderSync, sync_deleted_windows).unwrap();
        app.add_system(RenderSync, sync_window_changes.after(sync_deleted_windows))
            .unwrap();
        app.add_system(RenderSync, sync_created_windows.after(sync_window_changes))
            .unwrap();
        let render_state = app.world.resource_store.get_mut::<RenderState>().unwrap();

        render_state
            .render_world
            .component_store
            .register_component::<RenderWindow>();
        render_state
            .render_world
            .component_store
            .register_component::<SurfaceData>();
    }
}

#[derive(Component, Clone, Debug)]
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

#[derive(Component, PartialEq, Eq, Hash, Debug)]
pub struct SurfaceData {
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
}

impl SyncComponent for Window {
    type Out = RenderWindow;
    type ExtraData = Windows;

    fn extract(&self, index: usize, windows: &Windows) -> Self::Out {
        RenderWindow {
            raw_handle: RawHandleWrapper::new(
                windows
                    .winit_windows
                    .get(windows.entity_to_window.get(&index).unwrap())
                    .unwrap(),
            ),
            physical_width: self.resolution.width(),
            physical_height: self.resolution.height(),
            present_mode: self.present_mode,
            desired_maximum_frame_latency: self.desired_maximum_frame_latency,
            texture_view: None,
            texture: None,
            texture_format: None,
            alpha_mode: self.alpha_mode,
            size_changed: false,
            present_mode_changed: false,
            frame_latency_changed: false,
            alpha_mode_changed: false,
        }
    }
}

pub fn sync_deleted_windows(
    events: OnEvents<WindowClosed>,
    mut render_state: ResMut<RenderState>,
    query: Query<&RenderEntity>,
) {
    events
        .events
        .iter()
        .for_each(|event| match query.data.get(event.window) {
            Some(render_entity) => {
                render_state
                    .render_world
                    .component_store
                    .remove_component::<RenderWindow>(render_entity.id());
                render_state
                    .render_world
                    .component_store
                    .remove_component::<SurfaceData>(render_entity.id());
            }
            None => {
                match render_state
                    .render_world
                    .component_store
                    .get_components_ref::<MainEntity>()
                    .unwrap()
                    .iter()
                    .find(|(_, e)| e.id() == event.window)
                {
                    Some((render_entity, _)) => {
                        render_state
                            .render_world
                            .component_store
                            .remove_component::<RenderWindow>(render_entity)
                            .unwrap();
                        render_state
                            .render_world
                            .component_store
                            .remove_component::<SurfaceData>(render_entity)
                            .unwrap();
                    }
                    None => (),
                }
            }
        });
}

pub fn sync_window_changes(
    query: Query<(&Window, &RenderEntity)>,
    mut render_state: ResMut<RenderState>,
) {
    // SAFETY: Not accessing the same data
    let render_windows = unsafe { UnsafeWorldMut::new(&mut render_state.render_world).get_mut() }
        .component_store
        .get_components_mut::<RenderWindow>()
        .unwrap();
    let render_surfaces = unsafe { UnsafeWorldMut::new(&mut render_state.render_world).get_mut() }
        .component_store
        .get_components_mut::<SurfaceData>()
        .unwrap();
    for (index, window) in query.data.0 {
        match query.data.1.get(index) {
            Some(render_entity) => {
                if let Some(render_window) = render_windows.get_mut(render_entity.id()) {
                    let surface_data = render_surfaces.get_mut(render_entity.id()).unwrap();

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
                            PresentMode::Vsync => wgpu::PresentMode::AutoVsync,
                            PresentMode::NoVsync => wgpu::PresentMode::AutoNoVsync,
                            PresentMode::Fifo => wgpu::PresentMode::Fifo,
                            PresentMode::RelaxedFifo => wgpu::PresentMode::FifoRelaxed,
                            PresentMode::Mailbox => wgpu::PresentMode::Mailbox,
                            PresentMode::Immediate => wgpu::PresentMode::Immediate,
                        };
                    }

                    if render_window.desired_maximum_frame_latency
                        != window.desired_maximum_frame_latency
                    {
                        render_window.desired_maximum_frame_latency =
                            window.desired_maximum_frame_latency;
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
                            AlphaMode::Auto => wgpu::CompositeAlphaMode::Auto,
                            AlphaMode::Opaque => wgpu::CompositeAlphaMode::Opaque,
                            AlphaMode::PreMultiplied => wgpu::CompositeAlphaMode::PreMultiplied,
                            AlphaMode::PostMultiplied => wgpu::CompositeAlphaMode::PostMultiplied,
                            AlphaMode::Inherit => wgpu::CompositeAlphaMode::Inherit,
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
                    match surface_data.surface.get_current_texture() {
                        Success(texture) => {
                            let texture_view =
                                texture
                                    .texture
                                    .create_view(&wgpu::wgt::TextureViewDescriptor {
                                        format: Some(texture.texture.format().add_srgb_suffix()),
                                        ..Default::default()
                                    });

                            render_window.texture = Some(texture);
                            render_window.texture_view = Some(texture_view);
                        }
                        Suboptimal(texture) => {
                            let texture_view =
                                texture
                                    .texture
                                    .create_view(&wgpu::wgt::TextureViewDescriptor {
                                        format: Some(texture.texture.format().add_srgb_suffix()),
                                        ..Default::default()
                                    });

                            render_window.texture = Some(texture);
                            render_window.texture_view = Some(texture_view);
                        }
                        Timeout | Occluded | Validation => (),
                        Outdated => {
                            surface_data
                                .surface
                                .configure(render_state.get_device(), &surface_data.config);
                            match surface_data.surface.get_current_texture() {
                                Success(texture) => {
                                    let texture_view = texture.texture.create_view(
                                        &wgpu::wgt::TextureViewDescriptor {
                                            format: Some(
                                                texture.texture.format().add_srgb_suffix(),
                                            ),
                                            ..Default::default()
                                        },
                                    );

                                    render_window.texture = Some(texture);
                                    render_window.texture_view = Some(texture_view);
                                }
                                Suboptimal(texture) => {
                                    let texture_view = texture.texture.create_view(
                                        &wgpu::wgt::TextureViewDescriptor {
                                            format: Some(
                                                texture.texture.format().add_srgb_suffix(),
                                            ),
                                            ..Default::default()
                                        },
                                    );

                                    render_window.texture = Some(texture);
                                    render_window.texture_view = Some(texture_view);
                                }
                                _ => (),
                            }
                        }
                        Lost => panic!("Window Surface Lost"),
                    }
                }
            }
            None => (),
        }
    }
}

pub fn sync_created_windows(
    events: OnEvents<WindowCreated>,
    mut render_state: ResMut<RenderState>,
    windows: Res<Windows>,
    query: Query<(&RenderEntity, &Window)>,
) {
    for event in events.events {
        match query.data.0.get(event.window) {
            Some(render_entity) => {
                let mut extracted_window = query
                    .data
                    .1
                    .get(event.window)
                    .expect("Missing window component!")
                    .extract(event.window, &windows);

                let surface = unsafe {
                    render_state
                        .get_instance()
                        .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                            raw_display_handle: Some(
                                extracted_window.raw_handle.get_display_handle(),
                            ),
                            raw_window_handle: extracted_window.raw_handle.get_window_handle(),
                        })
                        .expect("Failed to create render surface for window!")
                };

                let caps = surface.get_capabilities(render_state.get_adapter());
                let texture_format = *caps.formats.iter().find(|f| f.is_srgb()).unwrap_or(
                    caps.formats
                        .first()
                        .expect("No supported texture formats for surface"),
                );
                let surface_config = SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: texture_format,
                    width: extracted_window.physical_width,
                    height: extracted_window.physical_height,
                    present_mode: *caps.present_modes.first().unwrap(),
                    // match extracted_window.present_mode {
                    //     PresentMode::Vsync => feufeu::wgpu::PresentMode::AutoVsync,
                    //     PresentMode::NoVsync => feufeu::wgpu::PresentMode::AutoNoVsync,
                    //     PresentMode::Fifo => feufeu::wgpu::PresentMode::Fifo,
                    //     PresentMode::RelaxedFifo => feufeu::wgpu::PresentMode::FifoRelaxed,
                    //     PresentMode::Mailbox => feufeu::wgpu::PresentMode::Mailbox,
                    //     PresentMode::Immediate => feufeu::wgpu::PresentMode::Immediate,
                    // },
                    desired_maximum_frame_latency: extracted_window
                        .desired_maximum_frame_latency
                        .map(NonZero::<u32>::get)
                        .unwrap_or(2),
                    alpha_mode: *caps.alpha_modes.first().unwrap(),
                    // match extracted_window.alpha_mode {
                    //     AlphaMode::Auto => feufeu::wgpu::CompositeAlphaMode::Auto,
                    //     AlphaMode::Opaque => feufeu::wgpu::CompositeAlphaMode::Opaque,
                    //     AlphaMode::PreMultiplied => {
                    //         feufeu::wgpu::CompositeAlphaMode::PreMultiplied
                    //     }
                    //     AlphaMode::PostMultiplied => {
                    //         feufeu::wgpu::CompositeAlphaMode::PostMultiplied
                    //     }
                    //     AlphaMode::Inherit => feufeu::wgpu::CompositeAlphaMode::Inherit,
                    // },
                    view_formats: vec![],
                    color_space: if texture_format.is_srgb() {
                        wgpu::SurfaceColorSpace::Srgb
                    } else {
                        wgpu::SurfaceColorSpace::Auto
                    },
                };

                surface.configure(render_state.get_device(), &surface_config);
                extracted_window.texture_format = Some(texture_format);

                render_state
                    .render_world
                    .component_store
                    .assign_components(
                        (
                            extracted_window,
                            SurfaceData {
                                surface,
                                config: surface_config,
                            },
                        ),
                        render_entity.id(),
                    )
                    .unwrap();
            }
            None => (),
        }
    }
}
