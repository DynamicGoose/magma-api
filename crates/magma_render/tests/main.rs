use magma_app::magma_ecs::ComponentStore;
use magma_app::magma_ecs::resource::ResMut;
use magma_app::{App, module::Module, schedule::Update};
use magma_render::RenderModule;
use magma_render::sync_entities::SyncToRenderWorld;
use magma_windowing::Window;

fn main() {
    env_logger::init();

    let mut app = App::new();
    app.add_module(RenderModule);
    app.add_module(TestModule);
    for _ in 0..400 {
        app.world
            .component_store
            .create_entity((Window::new(), SyncToRenderWorld))
            .unwrap();
    }
    app.run();
}

struct TestModule;

impl Module for TestModule {
    fn init(self, app: &mut App) {
        app.world.resource_store.insert(0_u32);
        // app.world
        //     .resource_store
        //     .get_mut::<RenderState>()
        //     .unwrap()
        //     .render_world
        //     .component_store
        //     .register_component::<TestRenderComp>();
        // app.add_system(
        //     RenderSync,
        //     check_render_windows.after(magma_render::sync_windows::sync_window_changes),
        // )
        // .unwrap();
        // app.add_system(RenderSync, get_test_comp).unwrap();
        // app.add_system(Update, count_exit).unwrap();
    }
}

// #[derive(Component, Debug)]
// struct TestRenderComp;

// fn check_render_windows(query: Query<(&Window, &RenderEntity)>, state: Res<RenderState>) {
//     for (index, _) in query.data.0 {
//         match query.data.1.get(index) {
//             Some(render_entity) => {
//                 match state
//                     .render_world
//                     .component_store
//                     .get_components_ref::<RenderWindow>()
//                     .unwrap()
//                     .get(render_entity.id())
//                 {
//                     Some(render_window) => println!(
//                         "Window with render id {} has render window {:?}",
//                         render_entity.id(),
//                         render_window
//                     ),
//                     None => println!(
//                         "Window has following render id {}, but no window",
//                         render_entity.id()
//                     ),
//                 }
//             }
//             None => println!("Window does not have render id"),
//         }
//     }
// }

// fn get_test_comp(state: Res<RenderState>) {
//     println!(
//         "{:?}",
//         state
//             .render_world
//             .component_store
//             .get_components_ref::<TestRenderComp>()
//             .unwrap()
//     );
// }

fn count_exit(store: &mut ComponentStore, mut counter: ResMut<u32>) {
    if *counter >= 500 {
        for index in store
            .get_components_ref::<Window>()
            .unwrap()
            .iter()
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
        {
            store.delete_entity(index);
        }
    } else {
        *counter += 1;
    }
}
