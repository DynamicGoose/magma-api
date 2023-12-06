use magma_app::{module::Module, App};
use window::{Window, Windows};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod window;

pub struct WinitModule;

impl WinitModule {}

impl Module for WinitModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.world.add_resource(Windows(vec![Window::new()]));
        app.set_runner(&winit_event_loop);
    }
}

fn winit_event_loop(mut app: App) {
    let event_loop = EventLoop::new().unwrap();
    {
        let windows = app.world.get_resource_mut::<Windows>().unwrap();

        for window in &mut windows.0 {
            window.0 = Some(WindowBuilder::new().build(&event_loop).unwrap());
        }
    }

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(|event, _elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                println!("The close button was pressed; stopping");
                let windows = &mut app.world.get_resource_mut::<Windows>().unwrap().0;
                for window in 0..windows.len() - 1 {
                    if windows[window].0.as_ref().unwrap().id() == window_id {
                        windows.remove(window);
                    }
                }
            }
            Event::AboutToWait => {
                app.update();
                for window in &app.world.get_resource::<Windows>().unwrap().0 {
                    window.0.as_ref().unwrap().request_redraw();
                }
            }
            _ => (),
        })
        .unwrap();
}
