use std::time::Instant;

use magma_app::{
    App, World,
    schedule::{Startup, Update},
};
use magma_ecs::{component::Component, events::EventSender, resources::ResMut};

#[test]
fn add_systems() {
    let mut app = App::new();
    app.world.register_component::<u32>();
    app.world.register_component::<Transform>();
    app.world.register_component::<Position>();
    app.world.register_component::<Rotation>();
    app.world.register_component::<Velocity>();

    app.add_system(Startup, system_startup, vec![]).unwrap();
    app.add_system(Update, update_resource, vec![]).unwrap();
    app.set_runner(test_runner);
    app.run();
}

#[test]
fn event_systems() {
    let mut app = App::new();
    app.register_event::<Event>();
    app.add_event_system(Event, update_resource, vec![])
        .unwrap();
    app.add_system(Update, event_system, vec![]).unwrap();

    app.set_runner(test_runner);
    app.run();
}

fn system_startup(world: &mut World) {
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

fn update_resource(mut res: ResMut<u32>) {
    *res += 1;
}

fn event_system(mut events: EventSender<Event>) {
    events.send(Event);
}

fn test_runner(mut app: App) {
    app.run_schedule::<Startup>().unwrap();
    for _ in 0..10 {
        app.run_schedule::<Update>().unwrap();
        app.process_events();
    }
    assert_eq!(10, *app.world.get_resource::<u32>().unwrap());
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

#[derive(Clone)]
struct Event;
