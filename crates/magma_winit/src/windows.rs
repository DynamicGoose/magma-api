use std::collections::HashMap;

use magma_app::entities::Entity;
use magma_math::IVec2;
use magma_window::{
    Window,
    window::{Monitor, VideoMode, WindowMode, WindowPosition, WindowResolution, WindowTheme},
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    monitor::VideoModeHandle,
    window::{CursorGrabMode, Fullscreen, Window as WinitWindow, WindowButtons, WindowId},
};

/// After adding the [`WinitModule`](crate::WinitModule) the [`Windows`] resource can be accessed.
#[derive(Default)]
pub struct Windows {
    pub window_to_entity: HashMap<WindowId, Entity>,
    pub entity_to_window: HashMap<Entity, WindowId>,
    pub winit_windows: HashMap<WindowId, WinitWindow>,
}

impl Windows {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delete_window(&mut self, window: Entity) {
        let window_id = self.entity_to_window.get(&window).unwrap();

        self.window_to_entity.remove(window_id);
        self.winit_windows.remove(window_id);
        self.entity_to_window.remove(&window);
    }

    pub fn create_winit_window(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window: &mut Window,
        entity: Entity,
    ) {
        let mut window_attributes = WinitWindow::default_attributes();

        let window_resolution = window.resolution();
        let window_resize_limit = window.resize_limit();

        let mut window_buttons = WindowButtons::empty();
        {
            let buttons = window.titlebar_buttons();
            if buttons.minimize() {
                window_buttons.insert(WindowButtons::MINIMIZE);
            }
            if buttons.maximize() {
                window_buttons.insert(WindowButtons::MAXIMIZE);
            }
            if buttons.close() {
                window_buttons.insert(WindowButtons::CLOSE);
            }
        }

        window_attributes = window_attributes
            .with_title(window.title())
            .with_inner_size(PhysicalSize::new(
                window_resolution.width(),
                window_resolution.height(),
            ))
            .with_resizable(window.resizable())
            .with_min_inner_size(PhysicalSize::new(
                window_resize_limit.min_width(),
                window_resize_limit.min_height(),
            ))
            .with_max_inner_size(PhysicalSize::new(
                window_resize_limit.max_width(),
                window_resize_limit.max_height(),
            ))
            .with_fullscreen(match window.mode() {
                WindowMode::Windowed => None,
                WindowMode::BorderlessFullscreen(monitor) => {
                    Some(Fullscreen::Borderless(match monitor {
                        Monitor::Current => None,
                        Monitor::Primary => event_loop.primary_monitor(),
                        Monitor::Index(id) => event_loop.available_monitors().nth(id),
                    }))
                }
                WindowMode::Fullscreen(monitor, video_mode) => {
                    Some(Fullscreen::Exclusive(match monitor {
                        Monitor::Current => match video_mode {
                            VideoMode::Current => {
                                let monitor = event_loop.available_monitors().nth(0).unwrap();
                                monitor
                                    .video_modes()
                                    .filter(|mode| {
                                        mode.size() == monitor.size()
                                            && Some(mode.refresh_rate_millihertz())
                                                == monitor.refresh_rate_millihertz()
                                    })
                                    .max_by_key(VideoModeHandle::bit_depth)
                                    .unwrap()
                            }
                            VideoMode::Specific {
                                size,
                                bit_depth,
                                refresh_rate_millihertz,
                            } => event_loop
                                .available_monitors()
                                .nth(0)
                                .unwrap()
                                .video_modes()
                                .find(|mode| {
                                    mode.size().width == size.x
                                        && mode.size().height == size.y
                                        && mode.refresh_rate_millihertz() == refresh_rate_millihertz
                                        && mode.bit_depth() == bit_depth
                                })
                                .unwrap(),
                        },
                        Monitor::Primary => match video_mode {
                            VideoMode::Current => {
                                let monitor = event_loop.primary_monitor().unwrap();
                                monitor
                                    .video_modes()
                                    .filter(|mode| {
                                        mode.size() == monitor.size()
                                            && Some(mode.refresh_rate_millihertz())
                                                == monitor.refresh_rate_millihertz()
                                    })
                                    .max_by_key(VideoModeHandle::bit_depth)
                                    .unwrap()
                            }
                            VideoMode::Specific {
                                size,
                                bit_depth,
                                refresh_rate_millihertz,
                            } => event_loop
                                .primary_monitor()
                                .unwrap()
                                .video_modes()
                                .find(|mode| {
                                    mode.size().width == size.x
                                        && mode.size().height == size.y
                                        && mode.refresh_rate_millihertz() == refresh_rate_millihertz
                                        && mode.bit_depth() == bit_depth
                                })
                                .unwrap(),
                        },
                        Monitor::Index(id) => match video_mode {
                            VideoMode::Current => {
                                let monitor = event_loop.available_monitors().nth(id).unwrap();
                                monitor
                                    .video_modes()
                                    .filter(|mode| {
                                        mode.size() == monitor.size()
                                            && Some(mode.refresh_rate_millihertz())
                                                == monitor.refresh_rate_millihertz()
                                    })
                                    .max_by_key(VideoModeHandle::bit_depth)
                                    .unwrap()
                            }
                            VideoMode::Specific {
                                size,
                                bit_depth,
                                refresh_rate_millihertz,
                            } => event_loop
                                .available_monitors()
                                .nth(id)
                                .unwrap()
                                .video_modes()
                                .find(|mode| {
                                    mode.size().width == size.x
                                        && mode.size().height == size.y
                                        && mode.refresh_rate_millihertz() == refresh_rate_millihertz
                                        && mode.bit_depth() == bit_depth
                                })
                                .unwrap(),
                        },
                    }))
                }
            })
            .with_decorations(window.decorations())
            .with_enabled_buttons(window_buttons)
            .with_transparent(window.transparent())
            .with_theme(match window.window_theme() {
                WindowTheme::Auto => None,
                WindowTheme::Light => Some(winit::window::Theme::Light),
                WindowTheme::Dark => Some(winit::window::Theme::Dark),
            });

        let winit_window = event_loop.create_window(window_attributes).unwrap();

        // winit_window.set_outer_position(match window.position() {
        //     WindowPosition::Auto => {
        //         let position = winit_window.outer_position().unwrap();
        //         window.set_position(WindowPosition::Pos(IVec2 {
        //             x: position.x,
        //             y: position.y,
        //         }));
        //         position
        //     }
        //     WindowPosition::Center => {
        //         let monitor_size = winit_window.current_monitor().unwrap().size();
        //         let x =
        //             (monitor_size.width as i32 / 2) - (winit_window.outer_size().width as i32 / 2);
        //         let y = (monitor_size.height as i32 / 2)
        //             - (winit_window.outer_size().height as i32 / 2);
        //         window.set_position(WindowPosition::Pos(IVec2 { x, y }));
        //         PhysicalPosition::new(x, y)
        //     }
        //     WindowPosition::Pos(vec) => PhysicalPosition::new(vec.x, vec.y),
        // });

        winit_window
            .set_cursor_grab(match window.cursor_mode() {
                magma_window::window::CursorMode::Free => winit::window::CursorGrabMode::None,
                magma_window::window::CursorMode::Confined => {
                    winit::window::CursorGrabMode::Confined
                }
                magma_window::window::CursorMode::Locked => winit::window::CursorGrabMode::Locked,
            })
            .or_else(|_| {
                // setting cursor mode to confined if locked failed
                window.set_cursor_mode(magma_window::window::CursorMode::Confined);
                winit_window.set_cursor_grab(CursorGrabMode::Confined)
            })
            .or_else(|_| {
                // setting cursor mode to locked if confined failed
                window.set_cursor_mode(magma_window::window::CursorMode::Locked);
                winit_window.set_cursor_grab(CursorGrabMode::Locked)
            })
            .unwrap();

        if window.focused() {
            winit_window.focus_window();
        }

        winit_window.set_cursor_visible(window.cursor_visible());

        window.set_resolution(WindowResolution::new(
            winit_window.inner_size().width,
            winit_window.inner_size().height,
        ));

        // add window to self
        let window_id = winit_window.id();

        self.winit_windows.insert(window_id, winit_window);
        self.window_to_entity.insert(window_id, entity);
        self.entity_to_window.insert(entity, window_id);

        window.has_window = true;
    }

    pub fn update_winit_window(&mut self, window: &mut Window, entity: Entity) {
        let winit_window = self
            .winit_windows
            .get(&self.entity_to_window.get(&entity).unwrap())
            .unwrap();

        winit_window.set_title(&window.title());
        winit_window
            .request_inner_size(PhysicalSize::new(
                window.resolution().width(),
                window.resolution().height(),
            ))
            .or(Some(PhysicalSize {
                width: 0,
                height: 0,
            }));
        winit_window.set_outer_position(match window.position() {
            WindowPosition::Auto => {
                let position = winit_window.outer_position().unwrap();
                window.set_position(WindowPosition::Pos(IVec2 {
                    x: position.x,
                    y: position.y,
                }));
                position
            }
            WindowPosition::Center => {
                let monitor_size = winit_window.current_monitor().unwrap().size();
                let x = (monitor_size.width as i32 / 2) - (window.resolution().width() as i32 / 2);
                let y =
                    (monitor_size.height as i32 / 2) - (window.resolution().height() as i32 / 2);
                window.set_position(WindowPosition::Pos(IVec2 { x, y }));
                PhysicalPosition::new(x, y)
            }
            WindowPosition::Pos(vec) => PhysicalPosition::new(vec.x, vec.y),
        });
        winit_window.set_resizable(window.resizable());
        winit_window.set_min_inner_size(Some(PhysicalSize::new(
            window.resize_limit().min_width(),
            window.resize_limit().min_height(),
        )));
        winit_window.set_max_inner_size(Some(PhysicalSize::new(
            window.resize_limit().max_width(),
            window.resize_limit().max_height(),
        )));

        match window.mode() {
            WindowMode::Windowed => winit_window.set_fullscreen(None),
            WindowMode::BorderlessFullscreen(monitor) => {
                winit_window.set_fullscreen(Some(Fullscreen::Borderless(match monitor {
                    Monitor::Current => winit_window.current_monitor(),
                    Monitor::Primary => winit_window.primary_monitor(),
                    Monitor::Index(id) => winit_window.available_monitors().nth(id),
                })));
            }
            WindowMode::Fullscreen(monitor, video_mode) => {
                winit_window.set_fullscreen(Some(Fullscreen::Exclusive(match monitor {
                    Monitor::Current => match video_mode {
                        VideoMode::Current => {
                            let monitor = winit_window.current_monitor().unwrap();
                            monitor
                                .video_modes()
                                .filter(|mode| {
                                    mode.size() == monitor.size()
                                        && Some(mode.refresh_rate_millihertz())
                                            == monitor.refresh_rate_millihertz()
                                })
                                .max_by_key(VideoModeHandle::bit_depth)
                                .unwrap()
                        }
                        VideoMode::Specific {
                            size,
                            bit_depth,
                            refresh_rate_millihertz,
                        } => winit_window
                            .current_monitor()
                            .unwrap()
                            .video_modes()
                            .find(|mode| {
                                mode.size().width == size.x
                                    && mode.size().height == size.y
                                    && mode.refresh_rate_millihertz() == refresh_rate_millihertz
                                    && mode.bit_depth() == bit_depth
                            })
                            .unwrap(),
                    },
                    Monitor::Primary => match video_mode {
                        VideoMode::Current => {
                            let monitor = winit_window.primary_monitor().unwrap();
                            monitor
                                .video_modes()
                                .filter(|mode| {
                                    mode.size() == monitor.size()
                                        && Some(mode.refresh_rate_millihertz())
                                            == monitor.refresh_rate_millihertz()
                                })
                                .max_by_key(VideoModeHandle::bit_depth)
                                .unwrap()
                        }
                        VideoMode::Specific {
                            size,
                            bit_depth,
                            refresh_rate_millihertz,
                        } => winit_window
                            .primary_monitor()
                            .unwrap()
                            .video_modes()
                            .find(|mode| {
                                mode.size().width == size.x
                                    && mode.size().height == size.y
                                    && mode.refresh_rate_millihertz() == refresh_rate_millihertz
                                    && mode.bit_depth() == bit_depth
                            })
                            .unwrap(),
                    },
                    Monitor::Index(id) => match video_mode {
                        VideoMode::Current => {
                            let monitor = winit_window.available_monitors().nth(id).unwrap();
                            monitor
                                .video_modes()
                                .filter(|mode| {
                                    mode.size() == monitor.size()
                                        && Some(mode.refresh_rate_millihertz())
                                            == monitor.refresh_rate_millihertz()
                                })
                                .max_by_key(VideoModeHandle::bit_depth)
                                .unwrap()
                        }
                        VideoMode::Specific {
                            size,
                            bit_depth,
                            refresh_rate_millihertz,
                        } => winit_window
                            .available_monitors()
                            .nth(id)
                            .unwrap()
                            .video_modes()
                            .find(|mode| {
                                mode.size().width == size.x
                                    && mode.size().height == size.y
                                    && mode.refresh_rate_millihertz() == refresh_rate_millihertz
                                    && mode.bit_depth() == bit_depth
                            })
                            .unwrap(),
                    },
                })));

                winit_window
                    .set_cursor_grab(match window.cursor_mode() {
                        magma_window::window::CursorMode::Free => {
                            winit::window::CursorGrabMode::None
                        }
                        magma_window::window::CursorMode::Confined => {
                            winit::window::CursorGrabMode::Confined
                        }
                        magma_window::window::CursorMode::Locked => {
                            winit::window::CursorGrabMode::Locked
                        }
                    })
                    .or_else(|_| {
                        // setting cursor mode to confined if locked failed
                        window.set_cursor_mode(magma_window::window::CursorMode::Confined);
                        winit_window.set_cursor_grab(CursorGrabMode::Confined)
                    })
                    .or_else(|_| {
                        // setting cursor mode to locked if confined failed
                        window.set_cursor_mode(magma_window::window::CursorMode::Locked);
                        winit_window.set_cursor_grab(CursorGrabMode::Locked)
                    })
                    .unwrap();

                winit_window.set_cursor_visible(window.cursor_visible());
                winit_window.set_decorations(window.decorations());

                let mut window_buttons = WindowButtons::empty();
                {
                    let buttons = window.titlebar_buttons();
                    if buttons.minimize() {
                        window_buttons.insert(WindowButtons::MINIMIZE);
                    }
                    if buttons.maximize() {
                        window_buttons.insert(WindowButtons::MAXIMIZE);
                    }
                    if buttons.close() {
                        window_buttons.insert(WindowButtons::CLOSE);
                    }
                }
                winit_window.set_enabled_buttons(window_buttons);

                if window.focused() {
                    winit_window.focus_window();
                }

                winit_window.set_theme(match window.window_theme() {
                    WindowTheme::Auto => None,
                    WindowTheme::Light => Some(winit::window::Theme::Light),
                    WindowTheme::Dark => Some(winit::window::Theme::Dark),
                });
            }
        }
    }
}
