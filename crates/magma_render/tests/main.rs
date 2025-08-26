use magma_app::{App, World, module::Module, schedule::Update};
use magma_render::RenderModule;
use magma_render::sync_entity::SyncToRenderWorld;
use magma_windowing::Window;

fn main() {
    env_logger::init();

    let mut app = App::new();
    app.add_module(RenderModule);
    app.add_module(TestModule);
    for _ in 0..2 {
        app.world
            .create_entity((Window::new(), SyncToRenderWorld))
            .unwrap();
    }
    app.run();
}

struct TestModule;

impl Module for TestModule {
    fn setup(self, app: &mut App) {
        app.world.add_resource(0_u32).unwrap();
        app.add_systems::<Update>(vec![(count_exit, "exit".to_string(), vec![])])
            .unwrap();
    }
}

fn count_exit(world: &World) {
    let mut counter = world.get_resource_mut::<u32>().unwrap();

    if *counter >= 500 {
        world
            .query::<(Window,)>()
            .unwrap()
            .iter()
            .for_each(|w| w.delete());
    } else {
        *counter += 1;
    }
}
