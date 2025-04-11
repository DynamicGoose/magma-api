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
use magma_window::Window;
use windows::Windows;
use winit::window::Window as WinitWindow;
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
        app.register_event::<winit::event::DeviceEvent>();
        app.register_event::<winit::event::WindowEvent>();
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
            WindowEvent::CloseRequested => {
                // let mut windows = self.app.world.get_resource_mut::<Windows>().unwrap();
                // let index = windows.windows.iter().position(|window| {
                //     window
                //         .as_ref()
                //         .is_some_and(|window| window.id() == window_id)
                // });
                // if let Some(index) = index {
                //     windows.despawn(index);
                // }
            }
            _ => {
                self.app
                    .world
                    .get_resource_mut::<Events>()
                    .unwrap()
                    .push_event(event)
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
            .get_resource_mut::<Events>()
            .unwrap()
            .push_event(event)
            .unwrap();
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.update();
        self.app
            .world
            .query::<(Window,)>()
            .unwrap()
            .iter()
            .for_each(|window_entity| {
                let mut window_component = window_entity.get_component_mut::<Window>().unwrap();
                if !window_component.has_winit_window() {
                    let window = event_loop
                        .create_window(WinitWindow::default_attributes())
                        .unwrap();
                    let window_id = window.id();

                    self.windows.winit_windows.insert(window_id, window);
                    self.windows
                        .window_to_entity
                        .insert(window_id, window_entity.id());
                    window_component.winit_window = true;
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
