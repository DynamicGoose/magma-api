use magma_app::{
    App, World,
    magma_ecs::{ComponentStore, query::Query, systems::IntoSystem},
    schedule::Update,
};
use magma_windowing::{Monitor, Window};
use magma_winit::WinitModule;

fn main() {
    let mut app = App::new();
    app.add_module(WinitModule);
    app.add_system(Update, open_windows).unwrap();
    app.add_system(Update, close_windows.after(open_windows))
        .unwrap();
    app.add_system(Update, print_monitors.after(close_windows))
        .unwrap();
    app.run();
}

fn open_windows(world: &mut World) {
    println!(
        "created window: {}",
        world.component_store.create_entity(Window::new()).unwrap()
    );
}

fn close_windows(component_store: &mut ComponentStore) {
    let windows = component_store
        .get_components_ref::<Window>()
        .unwrap()
        .iter()
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    if windows.len() >= 4 {
        println!("closing windows...");
        windows.into_iter().for_each(|w| {
            println!("closed window: {}", w);
            component_store.delete_entity(w);
        });
    }
}

fn print_monitors(windows: Query<&Window>, monitors: Query<&Monitor>) {
    if windows.data.len() == 0 {
        println!("monitor test...");
        for (_, monitor) in monitors.data {
            println!("{:?}", monitor);
        }
    }
}
