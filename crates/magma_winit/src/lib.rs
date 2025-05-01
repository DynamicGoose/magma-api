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

use magma_app::entities::Entity;
use magma_app::{App, events::Events, module::Module};
use magma_math::IVec2;
use magma_window::window::WindowTheme;
use magma_window::{ClosingWindow, window_event::*};
use magma_window::{Window, WindowModule};
use windows::Windows;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
};

pub use winit;

/// The [`Windows`] resource
pub mod windows;

mod systems;

/**
Adding the [`WinitModule`] to an [`App`] adds functionality for creating and managing windows. It also automatically spawns one window on application start.
*/
pub struct WinitModule;

impl Module for WinitModule {
    fn setup(self, app: &mut magma_app::App) {
        app.set_runner(winit_event_loop);
        app.add_module(WindowModule);

        app.add_event_systems::<WindowCloseRequested>(&[(
            systems::mark_closed_windows,
            "mark_closed_windows",
            &[],
        )])
        .unwrap();
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
                    self.windows.create_winit_window(
                        event_loop,
                        &mut window_component,
                        Entity(window_entity.id()),
                    );
                } else if window_component.changed_attr {
                    self.windows
                        .update_winit_window(&mut window_component, Entity(window_entity.id()));
                }
                window_component.changed_attr = false;
                self.app
                    .world
                    .get_resource_mut::<Events>()
                    .unwrap()
                    .push_event(WindowCreated {
                        window: Entity(window_entity.id()),
                    })
                    .unwrap();
            });

        // close windows which have a pending close request
        self.app
            .world
            .query::<(ClosingWindow, Window)>()
            .unwrap()
            .iter()
            .for_each(|window_entity| {
                self.windows.delete_window(Entity(window_entity.id()));
                window_entity.delete_component::<Window>().unwrap();
                window_entity.delete_component::<ClosingWindow>().unwrap();
                self.app
                    .world
                    .get_resource_mut::<Events>()
                    .unwrap()
                    .push_event(WindowClosed {
                        window: Entity(window_entity.id()),
                    })
                    .unwrap();
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
