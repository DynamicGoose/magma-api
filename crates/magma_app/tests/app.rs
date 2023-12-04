use magma_app::{App, SystemType, World};

#[test]
fn add_systems() {
    let mut app = App::new();
    app.add_systems(
        SystemType::Startup,
        (vec![&ref_system_startup], vec![&mut_system_startup]),
    );
    app.add_systems(SystemType::Update, (vec![], vec![&update_resource]));
    app.runner = &test_runner;
    app.run();
}

fn ref_system_startup(_: &World) {
    println!("startup_ref");
}

fn mut_system_startup(world: &mut World) {
    world.add_resource(0_u32);
    world.register_component::<u32>();
    world.spawn().with_component(10_u32).unwrap();
}

fn update_resource(world: &mut World) {
    *world.get_resource_mut::<u32>().unwrap() += 1;
    println!("updated");
}

fn test_runner(mut app: App) {
    app.world
        .startup(app.startup_systems.0, app.startup_systems.1);
    app.world
        .update(&condition, app.update_systems.0, app.update_systems.1);
}

fn condition(world: &World) -> bool {
    *world.get_resource::<u32>().unwrap() < 10000
}
