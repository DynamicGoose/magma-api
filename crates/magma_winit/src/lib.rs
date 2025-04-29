/*!
This crate integrates [`winit`] into the Magma API in order to manage application windows.

# Example

```
use magma_app::{App, SystemType, World};
use magma_winit::{windows::Windows, WinitModule};

let mut app = App::new();
app.add_module(WinitModule);
// spawn a window before running the app
app.world
    .get_resource_mut::<Windows>().unwrap().spawn(1);
app.add_systems(SystemType::Update, &[(close_window, "close_window", &[])]);
app.run();

// close the window while the app is running
fn close_window(world: &World) {
    let mut windows = world.get_resource_mut::<Windows>().unwrap();
    windows.despawn(0);
    windows.despawn(1);
}
```
*/

use magma_app::{App, events::Events, module::Module};
use magma_math::IVec2;
use magma_window::window::{Monitor, VideoMode, WindowMode, WindowPosition, WindowTheme};
use magma_window::window_event::*;
use magma_window::{Window, WindowModule};
use windows::Windows;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::monitor::VideoModeHandle;
use winit::window::{CursorGrabMode, Fullscreen, Window as WinitWindow, WindowButtons};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
};

pub use winit;

/// The [`Windows`] resource
pub mod windows;

/**
Adding the [`WinitModule`] to an [`App`] adds functionality for creating and managing windows. It also automatically spawns one window on application start.
*/
pub struct WinitModule;

impl Module for WinitModule {
    fn setup(self, app: &mut magma_app::App) {
        app.set_runner(winit_event_loop);
        app.add_module(WindowModule);
    }
}

struct WrappedApp {
    app: App,
    windows: Windows,
}

impl ApplicationHandler for WrappedApp {
    fn resumed(&mut self, _: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(physical_size) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowResized {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    width: physical_size.width,
                    height: physical_size.height,
                })
                .unwrap(),
            WindowEvent::Moved(physical_position) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowMoved {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    position: IVec2 {
                        x: physical_position.x,
                        y: physical_position.y,
                    },
                })
                .unwrap(),
            WindowEvent::CloseRequested => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowCloseRequested {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::Destroyed => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowDestroyed {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::DroppedFile(path_buf) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(FileDragDrop::Dropped {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    path: path_buf,
                })
                .unwrap(),
            WindowEvent::HoveredFile(path_buf) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(FileDragDrop::Hovered {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    path: path_buf,
                })
                .unwrap(),
            WindowEvent::HoveredFileCancelled => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(FileDragDrop::HoverCanceled {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::Focused(_) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowFocused {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::CursorMoved { position, .. } => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(CursorMoved {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    position: IVec2 {
                        x: position.x as i32,
                        y: position.y as i32,
                    },
                })
                .unwrap(),
            WindowEvent::CursorEntered { .. } => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(CursorEntered {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::CursorLeft { .. } => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(CursorLeft {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                })
                .unwrap(),
            WindowEvent::ThemeChanged(theme) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowThemeChanged {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    theme: match theme {
                        winit::window::Theme::Light => WindowTheme::Light,
                        winit::window::Theme::Dark => WindowTheme::Dark,
                    },
                })
                .unwrap(),
            WindowEvent::Occluded(occlusion) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(match occlusion {
                    true => WindowOcclusion::Occluded {
                        window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    },
                    false => WindowOcclusion::NotOccluded {
                        window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    },
                })
                .unwrap(),
            WindowEvent::RedrawRequested => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(RequestRedraw)
                .unwrap(),
            _ => (), // expand when input system is being implemented
        }
    }

    // fn device_event(
    //     &mut self,
    //     _: &winit::event_loop::ActiveEventLoop,
    //     _device_id: winit::event::DeviceId,
    //     event: winit::event::DeviceEvent,
    // ) {
    //     todo!()
    // }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // update the app
        self.app.update();
        // create winit windows for new window components
        self.app
            .world
            .query::<(Window,)>()
            .unwrap()
            .iter()
            .for_each(|window_entity| {
                let mut window_component = window_entity.get_component_mut::<Window>().unwrap();
                if !window_component.has_window {
                    let window = create_winit_window(event_loop, &mut window_component);
                    let window_id = window.id();

                    self.windows.winit_windows.insert(window_id, window);
                    self.windows
                        .window_to_entity
                        .insert(window_id, window_entity.id());
                    self.windows
                        .entity_to_window
                        .insert(window_entity.id(), window_id);
                    window_component.has_window = true;
                } else if window_component.changed_attr {
                    let winit_window = self
                        .windows
                        .winit_windows
                        .get(
                            &self
                                .windows
                                .entity_to_window
                                .get(&window_entity.id())
                                .unwrap(),
                        )
                        .unwrap();

                    // update window attributes
                    winit_window.set_title(&window_component.title())
                    // match window_component.position() {
                    //     magma_window::window::WindowPosition::Auto => ,
                    //     magma_window::window::WindowPosition::Center => todo!(),
                    //     magma_window::window::WindowPosition::Pos(ivec2) => todo!(),
                    // }
                }
            });
    }
}

fn winit_event_loop(app: App) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = WrappedApp {
        app,
        windows: Windows::new(),
    };
    event_loop.run_app(&mut app).unwrap();
}

fn create_winit_window(
    event_loop: &winit::event_loop::ActiveEventLoop,
    window: &mut Window,
) -> WinitWindow {
    let mut window_attributes = WinitWindow::default_attributes();

    let window_resolution = window.resolution();
    let window_resize_limit = window.resize_limit();

    let mut window_buttons = WindowButtons::empty();
    {
        let buttons = window.titlebar_buttons();
        if buttons.minimize {
            window_buttons.insert(WindowButtons::MINIMIZE);
        }
        if buttons.maximize {
            window_buttons.insert(WindowButtons::MAXIMIZE);
        }
        if buttons.close {
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

    window_attributes = match window.position() {
        WindowPosition::Center => {
            window_attributes.with_position(PhysicalPosition::new(0, 0)) // TODO: calculate screen center
        }
        WindowPosition::Pos(vec) => {
            window_attributes.with_position(PhysicalPosition::new(vec.x, vec.y))
        }
        _ => window_attributes,
    };

    let winit_window = event_loop.create_window(window_attributes).unwrap();

    winit_window
        .set_cursor_grab(match window.cursor_mode() {
            magma_window::window::CursorMode::Free => winit::window::CursorGrabMode::None,
            magma_window::window::CursorMode::Confined => winit::window::CursorGrabMode::Confined,
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

    winit_window
}
