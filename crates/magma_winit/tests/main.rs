use magma_app::App;
use magma_winit::WinitModule;

fn main() {
    WinitModule::create_window();
    let mut app = App::new();
    app.add_module(WinitModule);
    app.run();
}
