/*!
This crates integrates [`winit`] into the Magma3D engine, in order to manage application windows.
Here is a basic usage example:
```
use magma_app::{App, SystemType, World};
use magma_winit::{windows::Windows, WinitModule};

let mut app = App::new();
app.add_module(WinitModule);
// spawn a window before running the app
app.world.resources_write().get_mut::<Windows>().unwrap().spawn();
app.add_systems(SystemType::Update, vec![open_windows]);
app.run();

// open a new window every update and close them, once there's 4
fn open_windows(world: &World) {
    world.resources_write().get_mut::<Windows>().unwrap().spawn();
    let mut resources = world.resources_write();
    let window_resource = resources.get_mut::<Windows>().unwrap();
    if window_resource.windows.len() == 4 {
        for i in 0..4 {
            window_resource.despawn(i);
        }
    }
    println!("lel");
}
```
*/

use std::time::Duration;

use magma_app::{module::Module, App, World};
use windows::Windows;
use winit::{
    application::ApplicationHandler,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::pump_events::EventLoopExtPumpEvents,
    window::Window,
};

pub use winit;

/// The [`Windows`] resource
pub mod windows;

/**
Adding the [`WinitModule`] to an [`App`] adds functionality for creating and managing windows. It also automatically adds one window on application start.
```
use magma_app::{App, SystemType, World};
use magma_winit::{windows::Windows, WinitModule};

let mut app = App::new();
app.add_module(WinitModule);
```
*/
pub struct WinitModule;

impl Module for WinitModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.world.add_resource(Windows::new());
        app.set_runner(winit_event_loop);
        app.add_systems(magma_app::SystemType::Update, vec![handle_close_request]);
    }
}

struct WrappedApp {
    app: App,
}

impl ApplicationHandler for WrappedApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                let resources = self.app.world.resources_read();
                let windows = resources.get_ref::<Windows>().unwrap();
                let mut index = None;

                index = windows.windows.iter().position(|window| {
                    window
                        .as_ref()
                        .is_some_and(|window| window.id() == *window_id)
                });

                let mut resources = world.resources_write();
                if let Some(index) = index {
                    resources.get_mut::<Windows>().unwrap().despawn(index);
                }
            }
            _ => {
                let mut resources = app.world.resources_write();
                let windows = resources.get_mut::<Windows>().unwrap();
                windows.events.push(event);
            }
        }
    }
}

fn winit_event_loop(app: App) {
    let mut event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = WrappedApp { app };
    event_loop.run_app(&mut app);
    loop {
        {
            let mut resources = app.world.resources_write();
            let windows = resources.get_mut::<Windows>().unwrap();
            if windows.spawn {
                #[allow(deprecated)]
                let window = event_loop
                    .create_window(Window::default_attributes())
                    .unwrap();
                if let Some(none) = windows.windows.iter_mut().find(|window| window.is_none()) {
                    *none = Some(window);
                } else {
                    windows.windows.push(Some(window));
                }
                windows.spawn = false;
            }
            #[allow(deprecated)]
            event_loop.pump_events(Some(Duration::ZERO), |event, _| {
                windows.events.push(event);
            });
        }
        if !update(&app) {
            break;
        }
        app.world
            .resources_write()
            .get_mut::<Windows>()
            .unwrap()
            .events = vec![];
    }
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

fn handle_close_request(world: &World) {
    let resources = world.resources_read();
    let windows = resources.get_ref::<Windows>().unwrap();
    let mut index = None;
    for event in &windows.events {
        if let Event::WindowEvent {
            window_id,
            event: WindowEvent::CloseRequested,
        } = event
        {
            println!("closing window");
            index = windows.windows.iter().position(|window| {
                window
                    .as_ref()
                    .is_some_and(|window| window.id() == *window_id)
            });
        }
    }

    let mut resources = world.resources_write();
    if let Some(index) = index {
        resources.get_mut::<Windows>().unwrap().despawn(index);
    }
}
