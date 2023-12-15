use std::time::Duration;

use magma_app::{module::Module, App, World};
use windows::Windows;
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    platform::pump_events::EventLoopExtPumpEvents,
};

pub use winit::event::Event as WinitEvent;
pub use winit::*;

pub mod windows;

/// Adding the `WinitModule` to an `App` adds functionality for creating and managing windows. It also automatically adds one window on application start.
pub struct WinitModule;

impl Module for WinitModule {
    fn setup(&self, app: &mut magma_app::App) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        app.world.add_resource(Windows::new(event_loop));
        app.world.add_resource(WinitEvent::<()>::AboutToWait);
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
        // Use pump_events or run_on_demand to allow access to event_loop from outside the event_loop
        let windows = app.world.get_resource_mut::<Windows>().unwrap();
        windows
            .event_loop
            .pump_events(Some(Duration::ZERO), |event, _| {
                windows.event = event;
            });
        update(&mut app);
        println!("updated");
    }
}

fn update(app: &mut App) {
    let windows = app.world.get_resource_mut::<Windows>().unwrap();
    if windows
        .windows
        .iter()
        .filter(|window| window.is_some())
        .collect::<Vec<_>>()
        .is_empty()
    {
        windows.event_loop.exit()
    }
    app.update();
}

fn handle_close_request(world: &mut World) {
    let windows = world.get_resource_mut::<Windows>().unwrap();
    if let WinitEvent::WindowEvent {
        window_id,
        event: WindowEvent::CloseRequested,
    } = windows.event
    {
        let _ = windows
            .windows
            .iter_mut()
            .filter(|window| {
                window
                    .as_ref()
                    .is_some_and(|window| window.id() == window_id)
            })
            .map(|window| *window = None);
    }
}
