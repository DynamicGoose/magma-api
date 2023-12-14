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
    let mut ids: Vec<usize> = vec![];
    for window in world
        .query()
        .with_component::<Window>()
        .unwrap()
        .run_entity()
    {
        window.get_component_mut::<Window>().unwrap().0 = None;
        ids.push(window.id);
    }
    for id in ids {
        world.despawn(id).unwrap();
    }
}
