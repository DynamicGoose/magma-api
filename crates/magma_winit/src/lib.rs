/*!
This crate integrates [`winit`] into the Magma API in order to manage application windows.

# Example

```
# use std::error::Error;
# use magma_app::{magma_ecs::entities::Entity, App, SystemType, World};
# use magma_windowing::Window;
# use magma_winit::WinitModule;
fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    app.add_module(WinitModule);
    // Add the system to close created windows.
    // Windows should not be closed in a startup system, bc it might cause the app to hang.
    app.add_systems(SystemType::Update, &[(close_windows, "close_windows", &[])]);
    // create a window
    // The winit module will create a single window on startup. That means there will now be two.
    app.world.create_entity((Window::new().with_title("test"),))?;
    app.run();
    Ok(())
}

// system for closing the opened windows
fn close_windows(world: &World) {
    // close windows
    world
        .query::<(Window,)>()
        .unwrap()
        .iter()
        .for_each(|window| window.delete());
}
```
*/

use magma_app::{App, events::Events, module::Module};
use magma_math::{IVec2, UVec2};
use magma_windowing::monitor::VideoMode;
use magma_windowing::window::WindowTheme;
use magma_windowing::{ClosingWindow, Monitor, PrimaryMonitor, window_event::*};
use magma_windowing::{Window, WindowModule};
use windows::Windows;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
};

mod systems;
mod windows;

/**
The [`WinitModule`] adds winit as a backend for [magma_windowing](https://crates.io/crates/magma_windowing). It also automatically creates one window on application start.
*/
pub struct WinitModule;

impl Module for WinitModule {
    fn setup(self, app: &mut magma_app::App) {
        app.set_runner(winit_event_loop);
        app.add_module(WindowModule);

        // default event handling
        app.add_event_systems::<WindowCloseRequested>(&[(
            systems::mark_closed_windows,
            "winit_mark_closed",
            &[],
        )])
        .unwrap();
        app.add_event_systems::<WindowResized>(&[(systems::resized, "winit_resized", &[])])
            .unwrap();
        app.add_event_systems::<WindowMoved>(&[(systems::moved, "winit_moved", &[])])
            .unwrap();
        app.add_event_systems::<WindowFocused>(&[(systems::focused, "winit_focused", &[])])
            .unwrap();
    }
}

struct WrappedApp {
    app: App,
    windows: Windows,
}

impl ApplicationHandler for WrappedApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let primary_monitor = event_loop.primary_monitor();
        for (id, winit_monitor) in event_loop.available_monitors().enumerate() {
            let monitor = Monitor {
                name: winit_monitor.name(),
                height: winit_monitor.size().height,
                width: winit_monitor.size().width,
                position: IVec2::new(winit_monitor.position().x, winit_monitor.position().y),
                refresh_rate: winit_monitor.refresh_rate_millihertz(),
                scale_factor: winit_monitor.scale_factor(),
                video_modes: winit_monitor
                    .video_modes()
                    .map(|video_mode_handle| VideoMode {
                        size: UVec2::new(
                            video_mode_handle.size().width,
                            video_mode_handle.size().height,
                        ),
                        bit_depth: video_mode_handle.bit_depth(),
                        refresh_rate: video_mode_handle.refresh_rate_millihertz(),
                    })
                    .collect(),
                id,
            };
            if primary_monitor.as_ref() == Some(&winit_monitor) {
                self.app
                    .world
                    .create_entity((monitor, PrimaryMonitor))
                    .unwrap();
            } else {
                self.app.world.create_entity((monitor,)).unwrap();
            }
        }
    }

    fn window_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // convert winit events to app events
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
                .push_event(WindowDestroyed)
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
            WindowEvent::Focused(focus) => self
                .app
                .world
                .get_resource_mut::<Events>()
                .unwrap()
                .push_event(WindowFocused {
                    window: *self.windows.window_to_entity.get(&window_id).unwrap(),
                    focus,
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
                .push_event(RedrawRequested)
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
        // create winit windows for new window components
        self.app
            .world
            .query::<(Window,)>()
            .unwrap()
            .iter()
            .for_each(|window_entity| {
                let mut window_component = window_entity.get_component_mut::<Window>().unwrap();
                if !window_component.has_window {
                    self.windows.create_winit_window(
                        &self.app.world,
                        event_loop,
                        &mut window_component,
                        window_entity.into(),
                    );
                } else if window_component.changed_attr {
                    self.windows.update_winit_window(
                        &mut window_component,
                        window_entity.into(),
                        &self.app.world,
                    );
                }
                window_component.changed_attr = false;
                self.app
                    .world
                    .get_resource_mut::<Events>()
                    .unwrap()
                    .push_event(WindowCreated {
                        window: window_entity.into(),
                    })
                    .unwrap();
            });

        let windows = self.app.world.query::<(Window,)>().unwrap();

        // exit if no windows are present
        if windows.is_empty() {
            event_loop.exit();
        } else if windows.len() < self.windows.winit_windows.len() {
            // drop winit windows without an entity
            let windows_to_drop = self
                .windows
                .window_to_entity
                .iter()
                .filter_map(|(_, entity)| {
                    if windows
                        .iter()
                        .find(|query_entity| entity.id() == query_entity.id())
                        .is_none()
                    {
                        Some(*entity)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            for window in windows_to_drop {
                self.windows.delete_window(window);
            }
        }

        // Delete window entities which have a pending close request.
        // Their winit windows will be destroyd before the next update.
        // TODO: this could be refactored as a system, when scheduling systems at "start" or "end" is supported
        self.app
            .world
            .query::<(ClosingWindow, Window)>()
            .unwrap()
            .iter()
            .for_each(|closing_window| {
                closing_window.delete();
                self.app
                    .world
                    .get_resource_mut::<Events>()
                    .unwrap()
                    .push_event(WindowClosed {
                        window: closing_window.into(),
                    })
                    .unwrap();
            });

        // update the app
        self.app.update();
    }
}

fn winit_event_loop(app: App) {
    // create primary window
    app.world.create_entity((Window::new(),)).unwrap();
    // set up winit event loop
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = WrappedApp {
        app,
        windows: Windows::new(),
    };
    event_loop.run_app(&mut app).unwrap();
}
