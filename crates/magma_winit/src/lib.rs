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

use magma_app::{App, module::Module};
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
        app.world.get_resource_mut::<Windows>().unwrap().spawn(1);
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
                let mut windows = self.app.world.get_resource_mut::<Windows>().unwrap();
                let index = windows.windows.iter().position(|window| {
                    window
                        .as_ref()
                        .is_some_and(|window| window.id() == window_id)
                });
                if let Some(index) = index {
                    windows.despawn(index);
                }
            }
            _ => {
                self.app
                    .world
                    .get_resource_mut::<Windows>()
                    .unwrap()
                    .window_events
                    .push(event);
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
            .get_resource_mut::<Windows>()
            .unwrap()
            .device_events
            .push(event);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        {
            let mut windows = self.app.world.get_resource_mut::<Windows>().unwrap();
            for _ in 0..windows.spawn {
                let window = event_loop
                    .create_window(Window::default_attributes())
                    .unwrap();
                if let Some(none) = windows.windows.iter_mut().find(|w| w.is_none()) {
                    *none = Some(window);
                } else {
                    windows.windows.push(Some(window));
                }
            }
            windows.spawn = 0;
        }
        if !update(&self.app) {
            event_loop.exit();
        }
        let mut windows = self.app.world.get_resource_mut::<Windows>().unwrap();
        windows.window_events = vec![];
        windows.device_events = vec![];
    }
}

fn winit_event_loop(app: App) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = WrappedApp { app };
    event_loop.run_app(&mut app).unwrap();
}

fn update(app: &App) -> bool {
    if app
        .world
        .get_resource::<Windows>()
        .unwrap()
        .windows
        .iter()
        .filter(|w| w.is_some())
        .collect::<Vec<_>>()
        .is_empty()
    {
        return false;
    }
    app.update();
    true
}
