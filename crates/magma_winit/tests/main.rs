use magma_app::{App, SystemType, World};
use magma_winit::{window::Windows, WinitModule};

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.add_systems(SystemType::Update, (vec![], vec![&close_windows]));
    let windows = app.world.get_resource_mut::<Windows>().unwrap();
    windows.add_window();
    windows.add_window();
    app.run();
}

fn close_windows(world: &mut World) {
    world.get_resource_mut::<Windows>().unwrap().0.pop();
}
