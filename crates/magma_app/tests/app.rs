use magma_app::{App, SystemType, World};

#[test]
fn add_systems() {
    let mut app = App::new();
    app.world.register_component::<u32>();
    app.add_systems(
        SystemType::Startup,
        &[(system_startup, "system_startup", &[])],
    );
    app.add_systems(
        SystemType::Update,
        &[(update_resource, "update_resource", &[])],
    );
    app.set_runner(test_runner);
    app.run();
}

fn system_startup(world: &World) {
    world.add_resource(0_u32).unwrap();
    world.create_entity().with_component(10_u32).unwrap();
}

fn update_resource(world: &World) {
    world.resource_mut(|res: &mut u32| *res += 1).unwrap();
}

fn test_runner(app: App) {
    for _ in 0..10 {
        app.update();
    }
}
