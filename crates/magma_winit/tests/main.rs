use magma_app::App;
use magma_winit::{window::Windows, WinitModule};

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    let windows = app.world.get_resource_mut::<Windows>().unwrap();
    windows.add_window();
    app.run();
}
