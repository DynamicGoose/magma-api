/*!
This crate integrates [`winit`] into the Magma API in order to manage application windows.
Here is a basic usage example:
```
use magma_app::{App, SystemType, World};
use magma_winit::{windows::Windows, WinitModule};

let mut app = App::new();
app.add_module(WinitModule);
// spawn a window before running the app
app.world
    .resource_mut(|windows: &mut Windows| windows.spawn(1))
    .unwrap();
app.add_systems(SystemType::Update, &[(close_window, "close_window", &[])]);
app.run();

// close the window while the app is running
fn close_window(world: &World) {
    world.resource_mut(|windows: &mut Windows| {
        windows.despawn(0);
        windows.despawn(1);
    }).unwrap();
}
```
*/

use magma_app::{module::Module, App};
use windows::Windows;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
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
        app.world.add_resource(Windows::new()).unwrap();
        app.world
            .resource_mut(|windows: &mut Windows| windows.spawn(1))
            .unwrap();
        app.set_runner(winit_event_loop);
    }
}

struct WrappedApp {
    app: App,
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
            WindowEvent::CloseRequested => {
                self.app
                    .world
                    .resource_mut(|windows: &mut Windows| {
                        let index = windows.windows.iter().position(|window| {
                            window
                                .as_ref()
                                .is_some_and(|window| window.id() == window_id)
                        });
                        if let Some(index) = index {
                            windows.despawn(index);
                        }
                    })
                    .unwrap();
            }
            _ => {
                self.app
                    .world
                    .resource_mut(|windows: &mut Windows| windows.window_events.push(event))
                    .unwrap();
            }
        }
    }

    fn device_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.app
            .world
            .resource_mut(|windows: &mut Windows| windows.device_events.push(event))
            .unwrap();
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        {
            self.app
                .world
                .resource_mut(|windows: &mut Windows| {
                    if windows.spawn > 0 {
                        for _ in 0..windows.spawn {
                            let window = event_loop
                                .create_window(Window::default_attributes())
                                .unwrap();
                            if let Some(none) =
                                windows.windows.iter_mut().find(|window| window.is_none())
                            {
                                *none = Some(window);
                            } else {
                                windows.windows.push(Some(window));
                            }
                        }
                        windows.spawn = 0;
                    }
                })
                .unwrap();
        }
        if !update(&self.app) {
            event_loop.exit();
        }
        self.app
            .world
            .resource_mut(|windows: &mut Windows| {
                windows.window_events = vec![];
                windows.device_events = vec![];
            })
            .unwrap();
    }
}

fn winit_event_loop(app: App) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = WrappedApp { app };
    event_loop.run_app(&mut app).unwrap();
}

fn update(app: &App) -> bool {
    let mut result = true;
    app.world
        .resource_ref(|windows: &Windows| {
            if windows
                .windows
                .iter()
                .filter(|window| window.is_some())
                .collect::<Vec<_>>()
                .is_empty()
            {
                result = false;
            }
        })
        .unwrap();
    app.update();
    result
}
