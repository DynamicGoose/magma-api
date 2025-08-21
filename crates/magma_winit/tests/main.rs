use magma_app::{App, World, schedule::Update};
use magma_windowing::{Monitor, Window};
use magma_winit::WinitModule;

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.add_systems::<Update>(&[
        (close_windows, "close_windows", &["open_windows"]),
        (open_windows, "open_windows", &[]),
        (print_monitors, "print_monitors", &["close_windows"]),
    ])
    .unwrap();
    app.run();
}

fn open_windows(world: &World) {
    world.create_entity((Window::new(),)).unwrap();
}

fn close_windows(world: &World) {
    let windows = world.query::<(Window,)>().unwrap();
    if windows.len() >= 4 {
        println!("window test...");
        windows.iter().for_each(|w| {
            println!("closed window: {}", w.id());
            w.delete();
        });
    }
}

fn print_monitors(world: &World) {
    if world.query::<(Window,)>().unwrap().is_empty() {
        println!("monitor test...");
        world
            .query::<(Monitor,)>()
            .unwrap()
            .iter()
            .for_each(|monitor| println!("{:?}", monitor.get_component::<Monitor>().unwrap()));
    }
}
