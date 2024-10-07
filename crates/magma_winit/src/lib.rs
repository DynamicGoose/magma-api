/*!
This crate integrates [`winit`] into the Magma API in order to manage application windows.
Here is a basic usage example:
```
use magma_app::{App, SystemType, World};
use magma_winit::{windows::Windows, WinitModule};

let mut app = App::new();
app.add_module(WinitModule);
// spawn a window before running the app
app.world.resources_write().get_mut::<Windows>().unwrap().spawn(1);
app.add_systems(SystemType::Update, vec![close_window]);
app.run();

// close the window while the app is running
fn close_window(world: &World) {
    let mut resources = world.resources_write();
    let window_resource = resources.get_mut::<Windows>().unwrap();
    window_resource.despawn(0);
    // We have to despawn two windows, because the WinitModule spawns one at startup.
    window_resource.despawn(1);
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
    fn setup(&self, app: &mut magma_app::App) {
        app.world.add_resource(Windows::new());
        app.world
            .resources_write()
            .get_mut::<Windows>()
            .unwrap()
            .spawn(1);
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
                let resources = self.app.world.resources_read();
                let windows = resources.get_ref::<Windows>().unwrap();
                let index = windows.windows.iter().position(|window| {
                    window
                        .as_ref()
                        .is_some_and(|window| window.id() == window_id)
                });

                let mut resources = self.app.world.resources_write();
                if let Some(index) = index {
                    resources.get_mut::<Windows>().unwrap().despawn(index);
                }
            }
            _ => {
                let mut resources = self.app.world.resources_write();
                let windows = resources.get_mut::<Windows>().unwrap();
                windows.window_events.push(event);
            }
        }
    }

    fn device_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let mut resources = self.app.world.resources_write();
        let windows = resources.get_mut::<Windows>().unwrap();
        windows.device_events.push(event);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        {
            let mut resources = self.app.world.resources_write();
            let windows = resources.get_mut::<Windows>().unwrap();
            if windows.spawn > 0 {
                for _ in 0..windows.spawn {
                    let window = event_loop
                        .create_window(Window::default_attributes())
                        .unwrap();
                    if let Some(none) = windows.windows.iter_mut().find(|window| window.is_none()) {
                        *none = Some(window);
                    } else {
                        windows.windows.push(Some(window));
                    }
                }
                windows.spawn = 0;
            }
        }
        if !update(&self.app) {
            event_loop.exit();
        }
        self.app
            .world
            .resources_write()
            .get_mut::<Windows>()
            .unwrap()
            .window_events = vec![];
        self.app
            .world
            .resources_write()
            .get_mut::<Windows>()
            .unwrap()
            .device_events = vec![];
    }
}

fn winit_event_loop(app: App) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = WrappedApp { app };
    event_loop.run_app(&mut app).unwrap();
}

fn update(app: &App) -> bool {
    {
        let resources = app.world.resources_read();
        let windows = resources.get_ref::<Windows>().unwrap();
        if windows
            .windows
            .iter()
            .filter(|window| window.is_some())
            .collect::<Vec<_>>()
            .is_empty()
        {
            return false;
        }
    }
    app.update();
    true
}
