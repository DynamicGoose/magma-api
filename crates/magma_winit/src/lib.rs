use magma_app::{module::Module, App};
use window::Window;
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
    let mut window_query = app.world.query();

    let windows = window_query
        .with_component::<Window>()
        .unwrap()
        .run_entity();

    for window in windows {
        let mut window = window.get_component_mut::<Window>().unwrap();
        window.0 = Some(WindowBuilder::new().build(&event_loop).unwrap());
    }

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(|event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                println!("The close button was pressed; stopping");
                let mut ids: Vec<usize> = vec![];
                for window in app
                    .world
                    .query()
                    .with_component::<Window>()
                    .unwrap()
                    .run_entity()
                {
                    let mut window_component = window.get_component_mut::<Window>().unwrap();
                    if window_component
                        .0
                        .as_ref()
                        .is_some_and(|window| window.id() == window_id)
                    {
                        window_component.0 = None;
                        ids.push(window.id);
                    }
                }
                for id in ids {
                    app.world.despawn(id).unwrap();
                }
            }
            Event::AboutToWait => {
                if app
                    .world
                    .query()
                    .with_component::<Window>()
                    .unwrap()
                    .run_entity()
                    .is_empty()
                {
                    elwt.exit();
                }
                app.update();
            }
            _ => (),
        })
        .unwrap();
}
