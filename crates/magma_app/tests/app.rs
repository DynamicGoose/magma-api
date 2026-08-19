use std::time::Instant;

use magma_app::{
    App,
    schedule::{Startup, Update},
};
use magma_ecs::{ComponentStore, component::Component, resource::ResMut};

#[test]
fn add_systems() {
    let mut app = App::new();
    app.world.component_store.register_component::<Transform>();
    app.world.component_store.register_component::<Position>();
    app.world.component_store.register_component::<Rotation>();
    app.world.component_store.register_component::<Velocity>();

    app.add_system(Startup, system_startup).unwrap();
    app.add_system(Update, update_resource).unwrap();
    app.set_runner(test_runner);
    app.run();
}

fn system_startup(component_store: &mut ComponentStore) {
    let time = Instant::now();

    for _ in 0..1000 {
        component_store
            .create_entity((
                Transform([
                    [10, 10, 10, 10],
                    [10, 10, 10, 10],
                    [10, 10, 10, 10],
                    [10, 10, 10, 10],
                ]),
                Position((10, 10, 10)),
                Rotation((10, 10, 10)),
                Velocity((10, 10, 10)),
            ))
            .unwrap();
    }
    let elapsed = time.elapsed();
    println!("{}", elapsed.as_micros());
}

fn update_resource(mut res: ResMut<u32>) {
    *res += 1;
}

fn test_runner(mut app: App) {
    app.run_schedule::<Startup>().unwrap();
    for _ in 0..10 {
        app.run_schedule::<Update>().unwrap();
        app.world.event_manager.clear();
    }
    assert_eq!(10, *app.world.resource_store.get::<u32>().unwrap());
}

#[derive(Component)]
#[allow(dead_code)]
struct Transform([[i32; 4]; 4]);
#[derive(Component)]
#[allow(dead_code)]
struct Position((i32, i32, i32));
#[derive(Component)]
#[allow(dead_code)]
struct Rotation((i32, i32, i32));
#[derive(Component)]
#[allow(dead_code)]
struct Velocity((i32, i32, i32));
