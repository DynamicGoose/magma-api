use magma_app::{module::Module, World};
use ui::UI;

pub use egui;

pub mod ui;

pub struct UIModule;

impl Module for UIModule {
    fn setup(&self, app: &mut magma_app::App) {
        app.add_systems(
            magma_app::SystemType::Update,
            (vec![], vec![&init_ui, &update_ui]),
        )
    }
}

fn init_ui(world: &mut World) {
    todo!()
}
fn update_ui(_: &mut World) {
    todo!()
}
