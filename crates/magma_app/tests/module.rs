use magma_app::{module::Module, App};
use magma_ecs::World;

#[test]
fn add_module() {
    let mut app = App::new();
    app.add_module(TestModule);

    assert_eq!(*app.world.get_resource::<u32>().unwrap(), 10);
}

pub struct TestModule;

impl Module for TestModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.world.add_resource(10_u32);
        app.add_systems(magma_app::SystemType::Startup, (vec![&test_system], vec![]));
    }
}

fn test_system(_: &World) {
    println!("test");
}
