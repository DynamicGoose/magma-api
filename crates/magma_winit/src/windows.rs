use std::collections::HashMap;

use magma_app::{World, entities::Entity};
use magma_windowing::{
    Monitor, Window,
    raw_handle::WindowWrapper,
    window::{
        CursorMode, MonitorSelection, TitlebarButtons, VideoModeSelection, WindowMode,
        WindowPosition, WindowResolution, WindowTheme,
    },
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    monitor::{MonitorHandle, VideoModeHandle},
    window::{CursorGrabMode, Fullscreen, Theme, Window as WinitWindow, WindowButtons, WindowId},
};

/// After adding the [`WinitModule`](crate::WinitModule) the [`Windows`] resource can be accessed.
#[derive(Default)]
pub struct Windows {
    pub window_to_entity: HashMap<WindowId, Entity>,
    pub entity_to_window: HashMap<Entity, WindowId>,
    pub winit_windows: HashMap<WindowId, WindowWrapper<WinitWindow>>,
}

impl Windows {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_window_mut(&mut self, entity: &Entity) -> Option<&mut WindowWrapper<WinitWindow>> {
        match self.entity_to_window.get(entity) {
            Some(id) => self.winit_windows.get_mut(id),
            None => None,
        }
    }

    pub fn remove_window(&mut self, window: Entity) -> Option<WindowWrapper<WinitWindow>> {
        if let Some(window_id) = self.entity_to_window.get(&window) {
            self.window_to_entity.remove(&window_id);
            let wrapped_window = self.winit_windows.remove(&window_id);
            self.entity_to_window.remove(&window);
            wrapped_window
        } else {
            None
        }
    }

    pub fn create_winit_window(
        &mut self,
        world: &World,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window: &Window,
        entity: Entity,
    ) {
        let mut winit_window_attributes = WinitWindow::default_attributes();

        winit_window_attributes = winit_window_attributes
            .with_title(&window.title)
            .with_resizable(window.resizable)
            .with_decorations(window.decorations)
            .with_enabled_buttons(convert_titlebar_buttons(window.titlebar_buttons))
            .with_transparent(window.transparent)
            .with_theme(match window.window_theme {
                WindowTheme::Auto => None,
                WindowTheme::Light => Some(Theme::Light),
                WindowTheme::Dark => Some(Theme::Dark),
            });

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "windows"
        ))]
        if let Some(name) = &window.name {
            #[cfg(all(
                feature = "wayland",
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                    target_os = "windows"
                )
            ))]
            {
                winit_window_attributes =
                    winit::platform::wayland::WindowAttributesExtWayland::with_name(
                        winit_window_attributes,
                        name.clone(),
                        "",
                    );
            }
            #[cfg(all(
                feature = "x11",
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                    target_os = "windows"
                )
            ))]
            {
                winit_window_attributes = winit::platform::x11::WindowAttributesExtX11::with_name(
                    winit_window_attributes,
                    name.clone(),
                    "",
                );
            }
            #[cfg(target_os = "windows")]
            {
                winit_window_attributes =
                    winit::platform::windows::WindowAttributesExtWindows::with_class_name(
                        winit_window_attributes,
                        name.clone(),
                    );
            }
        }

        let monitor_selection = &match window.mode {
            WindowMode::Windowed => None,
            WindowMode::BorderlessFullscreen(monitor_selection) => match monitor_selection {
                MonitorSelection::Current => None,
                MonitorSelection::Primary => event_loop.primary_monitor(),
                MonitorSelection::Entity(entity) => match world.get_component::<Monitor>(entity) {
                    Ok(monitor) => event_loop.available_monitors().nth(monitor.id),
                    Err(_) => None,
                },
            },
            WindowMode::Fullscreen(monitor_selection, _) => match monitor_selection {
                MonitorSelection::Current => None,
                MonitorSelection::Primary => event_loop.primary_monitor(),
                MonitorSelection::Entity(entity) => match world.get_component::<Monitor>(entity) {
                    Ok(monitor) => event_loop.available_monitors().nth(monitor.id),
                    Err(_) => None,
                },
            },
        };

        winit_window_attributes = match window.mode {
            WindowMode::Windowed => {
                if let Some(position) =
                    winit_window_postition(monitor_selection, &window.resolution, &window.position)
                {
                    winit_window_attributes = winit_window_attributes.with_position(position);
                }
                winit_window_attributes.with_inner_size(PhysicalSize::new(
                    window.resolution.width(),
                    window.resolution.height(),
                ))
            }
            WindowMode::BorderlessFullscreen(_) => winit_window_attributes
                .with_fullscreen(Some(Fullscreen::Borderless(monitor_selection.clone()))),
            WindowMode::Fullscreen(_, video_mode_selection) => match monitor_selection {
                Some(monitor) => match video_mode_selection {
                    VideoModeSelection::Current => {
                        winit_window_attributes.with_fullscreen(Some(Fullscreen::Exclusive(
                            monitor
                                .video_modes()
                                .filter(|mode| {
                                    mode.size() == monitor.size()
                                        && Some(mode.refresh_rate_millihertz())
                                            == monitor.refresh_rate_millihertz()
                                })
                                .max_by_key(VideoModeHandle::bit_depth)
                                .expect("coudn't get current video mode"),
                        )))
                    }
                    VideoModeSelection::Specific {
                        size,
                        bit_depth,
                        refresh_rate_millihertz,
                    } => winit_window_attributes.with_fullscreen(Some(Fullscreen::Exclusive(
                        monitor
                            .video_modes()
                            .find(|mode| {
                                mode.size() == PhysicalSize::new(size.x, size.y)
                                    && mode.refresh_rate_millihertz() == refresh_rate_millihertz
                                    && mode.bit_depth() == bit_depth
                            })
                            .expect("coudn't get specified video mode"),
                    ))),
                },
                // fall back to primary monitor if no monitor was found
                None => match video_mode_selection {
                    VideoModeSelection::Current => {
                        let monitor = event_loop
                            .primary_monitor()
                            .expect("Failed to get monitor handle");
                        winit_window_attributes.with_fullscreen(Some(Fullscreen::Exclusive(
                            monitor
                                .video_modes()
                                .filter(|mode| {
                                    mode.size() == monitor.size()
                                        && Some(mode.refresh_rate_millihertz())
                                            == monitor.refresh_rate_millihertz()
                                })
                                .max_by_key(VideoModeHandle::bit_depth)
                                .expect("coudn't get current video mode"),
                        )))
                    }
                    VideoModeSelection::Specific {
                        size,
                        bit_depth,
                        refresh_rate_millihertz,
                    } => {
                        let monitor = event_loop
                            .primary_monitor()
                            .expect("Failed to get monitor handle");
                        winit_window_attributes.with_fullscreen(Some(Fullscreen::Exclusive(
                            monitor
                                .video_modes()
                                .find(|mode| {
                                    mode.size() == PhysicalSize::new(size.x, size.y)
                                        && mode.refresh_rate_millihertz() == refresh_rate_millihertz
                                        && mode.bit_depth() == bit_depth
                                })
                                .expect("coudn't get specified video mode"),
                        )))
                    }
                },
            },
        };

        if window.resize_limit.max_width() != u32::MAX
            || window.resize_limit.max_height() != u32::MAX
        {
            winit_window_attributes =
                winit_window_attributes.with_max_inner_size(PhysicalSize::new(
                    window.resize_limit.max_width(),
                    window.resize_limit.max_height(),
                ))
        }

        if window.resize_limit.min_width() != u32::MIN
            || window.resize_limit.min_height() != u32::MIN
        {
            winit_window_attributes =
                winit_window_attributes.with_min_inner_size(PhysicalSize::new(
                    window.resize_limit.min_width(),
                    window.resize_limit.min_height(),
                ))
        }

        let winit_window = event_loop.create_window(winit_window_attributes).unwrap();

        match window.cursor_mode {
            CursorMode::Free => (),
            CursorMode::Confined => winit_window
                .set_cursor_grab(CursorGrabMode::Confined)
                .or_else(|_| winit_window.set_cursor_grab(CursorGrabMode::Locked))
                .unwrap(),
            CursorMode::Locked => winit_window
                .set_cursor_grab(CursorGrabMode::Locked)
                .or_else(|_| winit_window.set_cursor_grab(CursorGrabMode::Confined))
                .unwrap(),
        }

        winit_window.set_cursor_visible(window.cursor_visible);

        if window.focused {
            winit_window.focus_window();
        }

        self.window_to_entity.insert(winit_window.id(), entity);
        self.entity_to_window.insert(entity, winit_window.id());

        self.winit_windows
            .insert(winit_window.id(), WindowWrapper::new(winit_window));
    }
}

fn winit_window_postition(
    monitor: &Option<MonitorHandle>,
    resolution: &WindowResolution,
    position: &WindowPosition,
) -> Option<PhysicalPosition<i32>> {
    match position {
        WindowPosition::Auto => None,
        WindowPosition::Center => {
            if let Some(monitor) = monitor {
                let screen_size = monitor.size();
                let position = PhysicalPosition {
                    x: ((screen_size.width / 2) - (resolution.width() / 2)) as i32,
                    y: ((screen_size.height / 2) - (resolution.height() / 2)) as i32,
                };

                Some(position)
            } else {
                None
            }
        }
        WindowPosition::Pos(pos) => Some(PhysicalPosition::new(pos.x, pos.y)),
    }
}

pub(crate) fn convert_titlebar_buttons(buttons: TitlebarButtons) -> WindowButtons {
    let mut window_buttons = WindowButtons::empty();
    if buttons.minimize() {
        window_buttons.insert(WindowButtons::MINIMIZE);
    }
    if buttons.maximize() {
        window_buttons.insert(WindowButtons::MAXIMIZE);
    }
    if buttons.close() {
        window_buttons.insert(WindowButtons::CLOSE);
    }
    window_buttons
}
