use magma_app::{App, World};
use magma_render::RenderModule;
use magma_windowing::Window;

fn main() {
    let mut app = App::new();
    app.add_module(RenderModule);
    app.add_systems(
        magma_app::SystemType::Update,
        &[(exit_system, "exit_system", &[])],
    );
    app.run();
}

fn exit_system(world: &World) {
    world
        .query::<(Window,)>()
        .unwrap()
        .iter()
        .for_each(|w| w.delete());
}
