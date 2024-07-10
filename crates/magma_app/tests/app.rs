use magma_app::{App, SystemType, World};

#[test]
fn add_systems() {
    let mut app = App::new();
    app.add_systems(SystemType::Startup, vec![system_startup]);
    app.add_systems(SystemType::Update, vec![update_resource]);
    app.set_runner(test_runner);
    app.run();
}

fn system_startup(world: &World) {
    world.add_resource(0_u32);
    world.register_component::<u32>();
    world
        .entities_write()
        .create_entity()
        .with_component(10_u32)
        .unwrap();
}

fn update_resource(world: &World) {
    *world.resources_write().get_mut::<u32>().unwrap() += 1;
}

fn test_runner(app: App) {
    for _ in 0..10 {
        app.update();
    }
}
