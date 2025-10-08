use magma_app::{
    entities::Entity,
    magma_ecs::{
        events::EventSender,
        query::{Changed, Query, Removed},
    },
    resources::ResMut,
};
use magma_windowing::{
    Monitor, Window,
    window::{CursorMode, VideoModeSelection, WindowMode, WindowPosition, WindowTheme},
    window_event::WindowClosed,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    monitor::VideoModeHandle,
    window::{CursorGrabMode, Fullscreen},
};

use crate::{
    CachedWindow, WindowsToDrop,
    windows::{Windows, convert_titlebar_buttons},
};

pub fn winit_update_changed_windows(
    query: Query<(Entity, &Window, &mut CachedWindow), Changed<Window>>,
    monitors: Query<(Entity, &Monitor)>,
    mut windows: ResMut<Windows>,
) {
    let monitors = monitors.into_iter().collect::<Vec<_>>();
    for (entity, window, mut cached_window) in query {
        let winit_window = windows.get_window_mut(&entity).unwrap();
        if window.title != cached_window.window.title {
            winit_window.set_title(&window.title);
        }
        if window.position != cached_window.window.position {
            match window.position {
                WindowPosition::Pos(pos) => {
                    let should_set = match winit_window.outer_position() {
                        Ok(current_pos) => current_pos != PhysicalPosition::new(pos.x, pos.y),
                        Err(_) => false,
                    };

                    if should_set {
                        winit_window.set_outer_position(PhysicalPosition::new(pos.x, pos.y));
                    }
                }
                _ => (),
            }
        }
        if window.resolution != cached_window.window.resolution {
            let _ = winit_window.request_inner_size(PhysicalSize::new(
                window.resolution.width(),
                window.resolution.height(),
            ));
        }
        if window.resizable != cached_window.window.resizable {
            winit_window.set_resizable(window.resizable);
        }
        if window.resize_limit != cached_window.window.resize_limit {
            winit_window.set_min_inner_size(Some(PhysicalSize::new(
                window.resize_limit.min_width(),
                window.resize_limit.min_height(),
            )));
            winit_window.set_max_inner_size(Some(PhysicalSize::new(
                window.resize_limit.max_width(),
                window.resize_limit.max_height(),
            )));
        }
        if window.mode != cached_window.window.mode {
            let monitor = match window.mode {
                WindowMode::Windowed => None,
                WindowMode::BorderlessFullscreen(monitor_selection) => match monitor_selection {
                    magma_windowing::window::MonitorSelection::Current => {
                        winit_window.current_monitor()
                    }
                    magma_windowing::window::MonitorSelection::Primary => {
                        winit_window.primary_monitor()
                    }
                    magma_windowing::window::MonitorSelection::Entity(entity) => {
                        match monitors.iter().find(|(e, _)| *e == entity) {
                            Some((_, monitor)) => winit_window.available_monitors().nth(monitor.id),
                            None => None,
                        }
                    }
                },
                WindowMode::Fullscreen(monitor_selection, _) => match monitor_selection {
                    magma_windowing::window::MonitorSelection::Current => {
                        winit_window.current_monitor()
                    }
                    magma_windowing::window::MonitorSelection::Primary => {
                        winit_window.primary_monitor()
                    }
                    magma_windowing::window::MonitorSelection::Entity(entity) => match monitors
                        .iter()
                        .find(|(e, _)| *e == entity)
                        .as_ref()
                        .copied()
                    {
                        Some((_, monitor)) => winit_window.available_monitors().nth(monitor.id),
                        None => None,
                    },
                },
            };

            match window.mode {
                WindowMode::Windowed => winit_window.set_fullscreen(None),
                WindowMode::BorderlessFullscreen(_) => {
                    winit_window.set_fullscreen(Some(Fullscreen::Borderless(monitor)))
                }
                WindowMode::Fullscreen(_, video_mode_selection) => match monitor {
                    Some(monitor) => match video_mode_selection {
                        VideoModeSelection::Current => {
                            winit_window.set_fullscreen(Some(Fullscreen::Exclusive(
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
                        } => winit_window.set_fullscreen(Some(Fullscreen::Exclusive(
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
                            let monitor = winit_window
                                .primary_monitor()
                                .expect("Failed to get monitor handle");
                            winit_window.set_fullscreen(Some(Fullscreen::Exclusive(
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
                            let monitor = winit_window
                                .primary_monitor()
                                .expect("Failed to get monitor handle");
                            winit_window.set_fullscreen(Some(Fullscreen::Exclusive(
                                monitor
                                    .video_modes()
                                    .find(|mode| {
                                        mode.size() == PhysicalSize::new(size.x, size.y)
                                            && mode.refresh_rate_millihertz()
                                                == refresh_rate_millihertz
                                            && mode.bit_depth() == bit_depth
                                    })
                                    .expect("coudn't get specified video mode"),
                            )))
                        }
                    },
                },
            }
        }
        if window.cursor_mode != cached_window.window.cursor_mode {
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
        }
        if window.cursor_position != cached_window.window.cursor_position {
            winit_window
                .set_cursor_position(PhysicalPosition::new(
                    window.cursor_position.x,
                    window.cursor_position.y,
                ))
                .or::<()>(Ok(()))
                .unwrap();
        }
        if window.cursor_visible != cached_window.window.cursor_visible {
            winit_window.set_cursor_visible(window.cursor_visible);
        }
        if window.decorations != cached_window.window.decorations {
            winit_window.set_decorations(window.decorations);
        }
        if window.titlebar_buttons != cached_window.window.titlebar_buttons {
            winit_window.set_enabled_buttons(convert_titlebar_buttons(window.titlebar_buttons));
        }
        if window.transparent != cached_window.window.transparent {
            winit_window.set_transparent(window.transparent);
        }
        if window.window_theme != cached_window.window.window_theme {
            match window.window_theme {
                WindowTheme::Auto => winit_window.set_theme(None),
                WindowTheme::Light => winit_window.set_theme(Some(winit::window::Theme::Light)),
                WindowTheme::Dark => winit_window.set_theme(Some(winit::window::Theme::Dark)),
            }
        }

        cached_window.window = window.clone();
    }
}

pub fn winit_drop_windows(
    closed_windows: Query<Entity, Removed<Window>>,
    mut windows: ResMut<Windows>,
    mut windows_to_drop: ResMut<WindowsToDrop>,
    mut events: EventSender<WindowClosed>,
) {
    windows_to_drop.0.clear();
    for entity in closed_windows {
        if let Some(winit_window) = windows.remove_window(entity) {
            windows_to_drop.0.push(winit_window);
            events.send(WindowClosed { window: entity });
        }
    }
}
