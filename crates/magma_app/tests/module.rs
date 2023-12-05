use magma_app::{module::Module, App};
use magma_ecs::World;

#[test]
fn add_module() {
    let mut app = App::new();
    app.add_module(TestModule);
}

pub struct TestModule;

impl Module for TestModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.add_systems(magma_app::SystemType::Startup, (vec![], vec![&test_system]));
    }
}

fn test_system(world: &mut World) {
    world.add_resource(10_u32);
    assert_eq!(*world.get_resource::<u32>().unwrap(), 10);
}
