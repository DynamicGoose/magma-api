use std::any::type_name_of_val;

use magma_app::{
    App, World,
    magma_ecs::query::{Query, QueryMut, With},
    schedule::Update,
};
use magma_windowing::{Monitor, Window};
use magma_winit::WinitModule;

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.add_system(Update, open_windows, vec![]).unwrap();
    app.add_system(
        Update,
        close_windows,
        vec![type_name_of_val(&open_windows).to_string()],
    )
    .unwrap();
    app.add_system(
        Update,
        print_monitors,
        vec![type_name_of_val(&close_windows).to_string()],
    )
    .unwrap();
    app.run();
}

fn open_windows(world: &mut World) {
    world.create_entity((Window::new(),)).unwrap();
}

fn close_windows(windows: QueryMut<With<Window>>) {
    if windows.len() >= 4 {
        println!("window test...");
        windows.iter().for_each(|w| {
            println!("closed window: {}", w.id());
            w.delete();
        });
    }
}

fn print_monitors(windows: Query<&Window>, monitors: Query<&Monitor>) {
    if windows.len() == 0 {
        println!("monitor test...");
        for monitor in monitors {
            println!("{:?}", monitor);
        }
    }
}
