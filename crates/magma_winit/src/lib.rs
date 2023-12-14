use magma_app::{module::Module, App, World};
use window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub use winit::event::Event as WinitEvent;
pub use winit::*;

pub mod window;

/// Adding the `WinitModule` to an `App` adds functionality for creating and managing windows. It also automatically adds one window on application start.
pub struct WinitModule;

impl Module for WinitModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.world.register_component::<Window>();
        app.world.add_resource(WinitEvent::<()>::AboutToWait);
        app.set_runner(&winit_event_loop);
        app.add_systems(
            magma_app::SystemType::Update,
            (vec![], vec![&handle_winit_events]),
        );
        app.world.spawn().with_component(Window::new()).unwrap();
    }
}

fn winit_event_loop(mut app: App) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(|event, elwt| match event {
            // Event::WindowEvent {
            //     event: WindowEvent::CloseRequested,
            //     window_id,
            // } => {
            //     println!("The close button was pressed; stopping");
            //     let mut ids: Vec<usize> = vec![];
            //     for window in app
            //         .world
            //         .query()
            //         .with_component::<Window>()
            //         .unwrap()
            //         .run_entity()
            //     {
            //         let mut window_component = window.get_component_mut::<Window>().unwrap();
            //         if window_component
            //             .0
            //             .as_ref()
            //             .is_some_and(|window| window.id() == window_id)
            //         {
            //             window_component.0 = None;
            //             ids.push(window.id);
            //         }
            //     }
            //     for id in ids {
            //         app.world.despawn(id).unwrap();
            //     }
            // }
            Event::AboutToWait => {
                let mut query = app.world.query();

                let windows = query.with_component::<Window>().unwrap().run_entity();
                if windows.is_empty() {
                    elwt.exit();
                }
                for window in windows {
                    let mut window = window.get_component_mut::<Window>().unwrap();
                    if window.0.is_none() {
                        window.0 = Some(WindowBuilder::new().build(elwt).unwrap());
                    }
                }
                *app.world.get_resource_mut::<WinitEvent<()>>().unwrap() = event;
                app.update();
            }
            _ => *app.world.get_resource_mut::<WinitEvent<()>>().unwrap() = event,
        })
        .unwrap();
}

fn handle_winit_events(world: &mut World) {
    match world.get_resource::<WinitEvent<()>>().unwrap() {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } => {
            println!("The close button was pressed; stopping");
            let mut ids: Vec<usize> = vec![];
            for window in world
                .query()
                .with_component::<Window>()
                .unwrap()
                .run_entity()
            {
                let mut window_component = window.get_component_mut::<Window>().unwrap();
                if window_component
                    .0
                    .as_ref()
                    .is_some_and(|window| &window.id() == window_id)
                {
                    window_component.0 = None;
                    ids.push(window.id);
                }
            }
            for id in ids {
                world.despawn(id).unwrap();
            }
        }
        _ => (),
    }
}
