use magma_app::{module::Module, World};
use magma_winit::{window::Window, WinitModule};
use ui::UI;

pub mod ui;

pub struct UIModule;

impl Module for UIModule {
    fn setup(&self, app: &mut magma_app::App) {
        tracing_subscriber::fmt::init();
        app.add_module(WinitModule);
        app.add_systems(
            magma_app::SystemType::Update,
            (vec![], vec![&init_ui, &update_ui]),
        )
    }
}

fn init_ui(world: &mut World) {
    let mut query = world.query();
    let ui_entities = query
        .with_component::<UI>()
        .unwrap()
        .with_component::<Window>()
        .unwrap()
        .run_entity();
    for ui in ui_entities {
        let mut ui = ui.get_component_mut::<UI>().unwrap();
        if ui.0 != 1 {
            ui.0 = 1;
        }
    }
}
fn update_ui(_: &mut World) {
    todo!()
}
