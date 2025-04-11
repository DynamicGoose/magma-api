use magma_app::{App, SystemType, World};
use magma_winit::{WinitModule, windows::Windows};

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.world.get_resource_mut::<Windows>().unwrap().spawn(1);
    app.add_systems(
        SystemType::Update,
        &[
            (open_windows, "open_windows", &[]),
            (close_windows, "close_windows", &[]),
        ],
    );
    app.run();
}

fn open_windows(world: &World) {
    let mut windows = world.get_resource_mut::<Windows>().unwrap();
    if windows.windows.len() < 4 {
        windows.spawn(1);
    }
}

fn close_windows(world: &World) {
    let mut windows = world.get_resource_mut::<Windows>().unwrap();
    if windows.windows.len() == 4 {
        for i in 0..4 {
            windows.despawn(i);
        }
    }
}
