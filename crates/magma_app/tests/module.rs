use magma_app::{App, module::Module, schedule::Startup};
use magma_ecs::resources::Res;

#[test]
fn add_module() {
    let mut app = App::new();
    app.add_module(TestModule);
    app.run_schedule::<Startup>().unwrap();
}

pub struct TestModule;

impl Module for TestModule {
    fn setup(self, app: &mut magma_app::App) {
        app.world.add_resource(10_u32).unwrap();
        app.add_system(Startup, test_system, vec![]).unwrap();
    }
}

fn test_system(res: Res<u32>) {
    assert_eq!(*res, 10)
}
