use magma_app::Module;
pub use material_table::MaterialTable;
pub use render_material::{RenderMaterial, RenderMaterialId};
pub use simulation_material::{SimulationMaterial, SimulationMaterialId};

pub mod material_table;
pub mod render_material;
pub mod simulation_material;

pub struct MaterialModule;

impl Module for MaterialModule {
    fn init(self, app: &mut magma_app::App) {
        app.world.resource_store.insert(MaterialTable::new());
    }
}
