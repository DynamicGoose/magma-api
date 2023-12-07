use std::ops::Deref;

use magma_app::{module::Module, App};
use window::{Window, Windows};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod window;

/// Adding the `WinitModule` to an `App` adds functionality for creating and managing windows.
pub struct WinitModule;

impl Module for WinitModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.world.register_component::<Window>();
        app.set_runner(&winit_event_loop);
    }
}

fn winit_event_loop(mut app: App) {
    let event_loop = EventLoop::new().unwrap();
    {
        let mut window_query = app.world.query();

        let mut windows = window_query
            .with_component::<Window>()
            .unwrap()
            .run_entity();

        for window in windows {
            let window = &mut window.get_component::<Window>().unwrap();
            window.0 = Some(WindowBuilder::new().build(&event_loop).unwrap());
        }
    }

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(|event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                println!("The close button was pressed; stopping");
                let windows = &mut app.world.get_resource_mut::<Windows>().unwrap().0;
                let indexes = 0..windows.len();
                for window_index in indexes {
                    let window = windows[window_index].0.as_ref();
                    if window.is_some_and(|window| window.id() == window_id) {
                        windows[window_index].0 = None;
                    }
                }
            }
            Event::AboutToWait => {
                app.update();
                if app
                    .world
                    .get_resource::<Windows>()
                    .unwrap()
                    .0
                    .iter()
                    .filter(|window| window.0.is_some())
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    elwt.exit()
                }
            }
            _ => (),
        })
        .unwrap();
}
