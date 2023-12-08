use magma_app::{App, SystemType, World};
use magma_winit::{window::Window, WinitModule};

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.world.spawn().with_component(Window::new()).unwrap();
    app.add_systems(
        SystemType::Update,
        (vec![], vec![&open_windows, &close_windows]),
    );
    app.run();
}

fn open_windows(world: &mut World) {
    world.spawn().with_component(Window::new()).unwrap();
}

fn close_windows(world: &mut World) {
    let mut query = world.query();
    let windows = query.with_component::<Window>().unwrap().run();
    if windows.indexes.len() == 4 {
        for index in windows.indexes {
            world.despawn(index).unwrap();
        }
    }
}
