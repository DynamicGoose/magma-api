use magma_app::{App, SystemType, World};
use magma_window::Window;
use magma_winit::WinitModule;

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.add_systems(
        SystemType::Update,
        &[
            (close_windows, "close_windows", &["open_windows"]),
            (open_windows, "open_windows", &[]),
        ],
    );
    app.run();
}

fn open_windows(world: &World) {
    world.create_entity((Window::new(),)).unwrap();
}

fn close_windows(world: &World) {
    let windows = world.query::<(Window,)>().unwrap();
    if windows.len() >= 4 {
        windows.iter().for_each(|w| {
            println!("closed window: {}", w.id());
            w.delete();
        });
    }
}
