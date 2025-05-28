use magma_app::{App, SystemType, World};
use magma_window::Window;
use magma_winit::WinitModule;

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.add_systems(
        SystemType::Update,
        &[
            (open_windows, "open_windows", &[]),
            (close_windows, "close_windows", &["open_windows"]),
        ],
    );
    app.world.create_entity((Window::new(),)).unwrap();
    app.run();
}

fn open_windows(world: &World) {
    world.create_entity((Window::new(),)).unwrap();
}

fn close_windows(world: &World) {
    let query = world.query::<(Window,)>().unwrap();
    if query.len() == 4 {
        query.iter().for_each(|w| w.delete());
    }
}
