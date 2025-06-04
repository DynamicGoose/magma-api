use magma_app::{App, SystemType, World};
use magma_windowing::{Monitor, Window};
use magma_winit::WinitModule;

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.add_systems(
        SystemType::Update,
        &[
            (
                close_windows,
                "close_windows",
                &["open_windows", "print_monitors"],
            ),
            (open_windows, "open_windows", &[]),
            (print_monitors, "print_monitors", &[]),
        ],
    );
    app.run();
}

fn open_windows(world: &World) {
    for _ in 0..2 {
        world.create_entity((Window::new(),)).unwrap();
    }
}

fn close_windows(world: &World) {
    world.query::<(Window,)>().unwrap().iter().for_each(|w| {
        println!("closed window: {}", w.id());
        w.delete();
    });
}

fn print_monitors(world: &World) {
    world
        .query::<(Monitor,)>()
        .unwrap()
        .iter()
        .for_each(|monitor| println!("monitor: {:?}", monitor.get_component::<Monitor>().unwrap()));
}
