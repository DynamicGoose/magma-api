use std::time::Instant;

use magma_app::{App, SystemType, World};

#[test]
fn add_systems() {
    let mut app = App::new();
    app.world.register_component::<u32>();
    app.world.register_component::<Transform>();
    app.world.register_component::<Position>();
    app.world.register_component::<Rotation>();
    app.world.register_component::<Velocity>();

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

    let time = Instant::now();
    world
        .create_entity_batch(
            (
                Transform([
                    [10, 10, 10, 10],
                    [10, 10, 10, 10],
                    [10, 10, 10, 10],
                    [10, 10, 10, 10],
                ]),
                Position((10, 10, 10)),
                Rotation((10, 10, 10)),
                Velocity((10, 10, 10)),
            ),
            10000,
        )
        .unwrap();
    let elapsed = time.elapsed();
    println!("{}", elapsed.as_micros());
}

fn update_resource(world: &World) {
    world.resource_mut(|res: &mut u32| *res += 1).unwrap();
}

fn test_runner(app: App) {
    for _ in 0..10 {
        app.update();
    }
}

#[allow(dead_code)]
struct Transform([[i32; 4]; 4]);
#[allow(dead_code)]
struct Position((i32, i32, i32));
#[allow(dead_code)]
struct Rotation((i32, i32, i32));
#[allow(dead_code)]
struct Velocity((i32, i32, i32));
