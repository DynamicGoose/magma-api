use magma_app::{App, SystemType, World};
use magma_winit::{windows::Windows, WinitModule};

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.world
        .resources_write()
        .get_mut::<Windows>()
        .unwrap()
        .spawn(1);
    app.add_systems(SystemType::Update, vec![open_windows, close_windows]);
    app.run();
}

fn open_windows(world: &World) {
    println!("open");
    world
        .resources_write()
        .get_mut::<Windows>()
        .unwrap()
        .spawn(1);
    println!("open_end")
}

fn close_windows(world: &World) {
    println!("close");
    let mut resources = world.resources_write();
    println!("write_close");
    let window_resource = resources.get_mut::<Windows>().unwrap();
    if window_resource.windows.len() == 4 {
        for i in 0..4 {
            window_resource.despawn(i);
        }
    }
}
