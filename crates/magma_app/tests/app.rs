use std::time::Instant;

use magma_app::{
    App, World,
    schedule::{Startup, Update},
};

#[test]
fn add_systems() {
    let mut app = App::new();
    app.world.register_component::<u32>();
    app.world.register_component::<Transform>();
    app.world.register_component::<Position>();
    app.world.register_component::<Rotation>();
    app.world.register_component::<Velocity>();

    app.world.add_resource(10_u32).unwrap();

    app.add_systems::<Startup>(&[(system_startup, "system_startup", &[])])
        .unwrap();
    app.add_systems::<Update>(&[(update_resource, "update_resource", &[])])
        .unwrap();
    app.set_runner(test_runner);
    app.run();
}

#[test]
fn event_systems() {
    let mut app = App::new();
    app.register_event::<Event>();
    app.add_event_systems::<Event>(&[(update_resource, "update_resource", &[])])
        .unwrap();
    app.world.add_resource(10_u32).unwrap();

    app.add_systems::<Update>(&[(event_system, "event_system", &[])])
        .unwrap();

    app.set_runner(test_runner);
    app.run();
}

fn system_startup(world: &World) {
    let time = Instant::now();

    for _ in 0..1000 {
        world
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

fn update_resource(world: &World) {
    let mut res = world.get_resource_mut::<u32>().unwrap();
    *res += 1;
}

fn event_system(world: &World) {
    world.send_event(Event).unwrap();
}

fn test_runner(app: App) {
    app.run_schedule::<Startup>().unwrap();
    for _ in 0..10 {
        app.run_schedule::<Update>().unwrap();
        app.process_events();
    }
    assert_eq!(20, *app.world.get_resource::<u32>().unwrap());
}

#[allow(dead_code)]
struct Transform([[i32; 4]; 4]);
#[allow(dead_code)]
struct Position((i32, i32, i32));
#[allow(dead_code)]
struct Rotation((i32, i32, i32));
#[allow(dead_code)]
struct Velocity((i32, i32, i32));

#[derive(Clone)]
struct Event;
