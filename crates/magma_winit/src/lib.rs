use magma_app::{module::Module, App};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct WinitModule;

impl WinitModule {
    pub fn create_window() {
        let event_loop = EventLoop::new().unwrap();
        Window::new(&event_loop).unwrap();
    }
}

impl Module for WinitModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.runner = &winit_event_loop;
    }
}

fn winit_event_loop(mut app: App) {
    let event_loop = EventLoop::new().unwrap();
    WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    elwt.exit();
                }
                Event::AboutToWait => {
                    app.world
                        .startup(app.update_systems.0.clone(), app.update_systems.1.clone());
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Redraw the application.
                    //
                    // It's preferable for applications that do not render continuously to render in
                    // this event rather than in AboutToWait, since rendering in here allows
                    // the program to gracefully handle redraws requested by the OS.
                }
                _ => (),
            }
        })
        .unwrap();
}
