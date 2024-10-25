use magma_app::{module::Module, App};
use magma_ecs::World;

#[test]
fn add_module() {
    let mut app = App::new();
    app.add_module(TestModule);
}

pub struct TestModule;

impl Module for TestModule {
    fn setup(self, app: &mut magma_app::App) {
        app.add_systems(
            magma_app::SystemType::Startup,
            &[(test_system, "test_system", &[])],
        );
    }
}

fn test_system(world: &World) {
    world.add_resource(10_u32).unwrap();
    world
        .resource_ref(|res: &u32| assert_eq!(*res, 10))
        .unwrap();
}
