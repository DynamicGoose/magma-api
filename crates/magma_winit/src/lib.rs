/*!
This crates integrates [`winit`] into the Magma3D engine.
Here is a basic usage example:
```
use magma_app::{App, SystemType, World};
use magma_winit::{windows::Windows, WinitModule};

let mut app = App::new();
app.add_module(WinitModule);
// spawn a window before running the app
app.world.get_resource_mut::<Windows>().unwrap().spawn();
app.add_systems(
    SystemType::Update,
    (vec![], vec![&open_windows, &close_windows]),
);
app.run();

// open a new window every update
fn open_windows(world: &mut World) {
    world.get_resource_mut::<Windows>().unwrap().spawn();
}

// close all the windows when 4 have been spawned
fn close_windows(world: &mut World) {
    let window_resource = world.get_resource_mut::<Windows>().unwrap();
    if window_resource.windows.len() == 4 {
        for i in 0..4 {
            window_resource.despawn(i);
        }
    }
}
```
*/

use std::time::Duration;

use magma_app::{module::Module, App, World};
use windows::Windows;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::pump_events::EventLoopExtPumpEvents,
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
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        app.world.add_resource(Windows::new(event_loop));
        app.set_runner(&winit_event_loop);
        app.add_systems(
            magma_app::SystemType::Update,
            (vec![], vec![&handle_close_request]),
        );
        app.world.get_resource_mut::<Windows>().unwrap().spawn();
    }
}

fn winit_event_loop(mut app: App) {
    loop {
        let windows = app.world.get_resource_mut::<Windows>().unwrap();
        windows
            .event_loop
            .pump_events(Some(Duration::ZERO), |event, _| {
                windows.events.push(event);
            });
        if !update(&mut app) {
            break;
        }
        app.world.get_resource_mut::<Windows>().unwrap().events = vec![];
    }
}

fn update(app: &mut App) -> bool {
    let windows = app.world.get_resource_mut::<Windows>().unwrap();
    if windows
        .windows
        .iter()
        .filter(|window| window.is_some())
        .collect::<Vec<_>>()
        .is_empty()
    {
        return false;
    }
    app.update();
    true
}

fn handle_close_request(world: &mut World) {
    let windows = world.get_resource_mut::<Windows>().unwrap();
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
    if let Some(index) = index {
        windows.despawn(index);
    }
}
