use magma_app::{module::Module, App};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct WinitModule;

impl WinitModule {}

impl Module for WinitModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.set_runner(&winit_event_loop);
    }
}

fn winit_event_loop(mut app: App) {
    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            Event::AboutToWait => {
                app.update();
            }
            _ => (),
        })
        .unwrap();
}
