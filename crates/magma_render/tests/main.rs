use magma_app::{App, World, module::Module};
use magma_render::RenderModule;
use magma_windowing::Window;

fn main() {
    let mut app = App::new();
    app.add_module(RenderModule);
    app.add_module(TestModule);
    for _ in 0..3 {
        app.world
            .create_entity((Window::new().with_title("Test Window"),))
            .unwrap();
    }
    app.run();
}

struct TestModule;

impl Module for TestModule {
    fn setup(self, app: &mut App) {
        app.world.add_resource(0_u32).unwrap();
        app.add_systems(magma_app::SystemType::Update, &[(count_exit, "exit", &[])]);
    }
}

fn count_exit(world: &World) {
    let mut counter = world.get_resource_mut::<u32>().unwrap();

    if *counter >= 1000 {
        world
            .query::<(Window,)>()
            .unwrap()
            .iter()
            .for_each(|w| w.delete());
    } else {
        *counter += 1;
    }
}
