use magma_app::{App, SystemType, World};
use magma_winit::{window::Window, WinitModule};

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    for _ in 0..2 {
        app.world.spawn().with_component(Window::new()).unwrap();
    }
    app.add_systems(SystemType::Update, (vec![], vec![&close_windows]));
    app.run();
}

fn close_windows(world: &mut World) {
    let mut query = world.query();
    let windows = query.with_component::<Window>().unwrap().run();
    if !windows.indexes.is_empty() {
        world.despawn(windows.indexes[0]).unwrap();
    }
}
