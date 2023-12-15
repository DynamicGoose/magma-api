use magma_app::{App, SystemType, World};
use magma_winit::{windows::Windows, WinitModule};

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.world.get_resource_mut::<Windows>().unwrap().spawn();
    app.add_systems(
        SystemType::Update,
        (vec![], vec![&open_windows, &close_windows]),
    );
    app.run();
}

fn open_windows(world: &mut World) {
    world.get_resource_mut::<Windows>().unwrap().spawn();
}

fn close_windows(world: &mut World) {
    // println!("{:?}", world.get_resource::<Windows>().unwrap().events);
    let window_resource = world.get_resource_mut::<Windows>().unwrap();
    if window_resource.windows.len() == 4 {
        for i in 0..4 {
            window_resource.despawn(i);
        }
    }
}
