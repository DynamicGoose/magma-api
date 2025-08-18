use magma_app::App;
use magma_render::RenderModule;

fn main() {
    let mut app = App::new();
    app.add_module(RenderModule);
    app.run();
}
