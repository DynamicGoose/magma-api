use magma_app::App;
use magma_render::RenderModule;
use magma_windowing::Window;

fn main() {
    let mut app = App::new();
    app.add_module(RenderModule);
    for _ in 0..80 {
        app.world
            .create_entity((Window::new().with_title("Second Window"),))
            .unwrap();
    }
    app.run();
}
