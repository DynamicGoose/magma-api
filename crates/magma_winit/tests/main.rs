use magma_app::{App, SystemType, World};
use magma_winit::{windows::Windows, WinitModule};

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.world
        .resource_mut(|windows: &mut Windows| windows.spawn(1))
        .unwrap();
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
    world
        .resource_mut(|windows: &mut Windows| {
            if windows.windows.len() < 4 {
                windows.spawn(1);
            }
        })
        .unwrap();
}

fn close_windows(world: &World) {
    world
        .resource_mut(|windows: &mut Windows| {
            if windows.windows.len() == 4 {
                for i in 0..4 {
                    windows.despawn(i);
                }
            }
        })
        .unwrap();
}
