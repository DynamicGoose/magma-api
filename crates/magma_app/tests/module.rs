use magma_app::{App, module::Module, schedule::Startup};
use magma_ecs::resource::Res;

#[test]
fn add_module() {
    let mut app = App::new();
    app.add_module(TestModule);
    app.run_schedule::<Startup>().unwrap();
}

pub struct TestModule;

impl Module for TestModule {
    fn setup(self, app: &mut magma_app::App) {
        app.world.resource_store.insert(10_u32);
        app.add_system(Startup, test_system).unwrap();
    }
}

fn test_system(res: Res<u32>) {
    assert_eq!(*res, 0)
}
