use crate::{RenderMaterial, RenderMaterialId, SimulationMaterial, SimulationMaterialId};

#[derive(Default)]
pub struct MaterialTable {
    render_mats: Vec<RenderMaterial>,
    simulation_mats: Vec<SimulationMaterial>,
}

impl MaterialTable {
    pub const fn new() -> Self {
        Self {
            render_mats: vec![],
            simulation_mats: vec![],
        }
    }

    pub fn push_render_material(&mut self, mat: RenderMaterial) -> RenderMaterialId {
        self.render_mats.push(mat);
        RenderMaterialId::new(self.render_mats.len() - 1)
    }

    pub fn push_simulation_material(&mut self, mat: SimulationMaterial) -> SimulationMaterialId {
        self.simulation_mats.push(mat);
        SimulationMaterialId::new(self.simulation_mats.len() - 1)
    }

    pub fn get_render_material(&self, id: RenderMaterialId) -> Option<&RenderMaterial> {
        self.render_mats.get(id.id())
    }

    pub fn get_simulation_material(&self, id: SimulationMaterialId) -> Option<&SimulationMaterial> {
        self.simulation_mats.get(id.id())
    }

    pub fn get_render_material_mut(&mut self, id: RenderMaterialId) -> Option<&mut RenderMaterial> {
        self.render_mats.get_mut(id.id())
    }

    pub fn get_simulation_material_mut(
        &mut self,
        id: SimulationMaterialId,
    ) -> Option<&mut SimulationMaterial> {
        self.simulation_mats.get_mut(id.id())
    }

    pub fn find_render_material(&self, name: &str) -> Option<RenderMaterialId> {
        self.render_mats
            .iter()
            .position(|mat| mat.name == name)
            .map(|index| RenderMaterialId::new(index))
    }

    pub fn find_simulation_material(&self, name: &str) -> Option<SimulationMaterialId> {
        self.simulation_mats
            .iter()
            .position(|mat| mat.name == name)
            .map(|index| SimulationMaterialId::new(index))
    }
}

#[cfg(test)]
mod tests {
    use crate::MaterialTable;

    #[test]
    fn get_material() {
        let mut mat_table = MaterialTable::new();
        let id = mat_table.push_simulation_material(crate::SimulationMaterial {
            name: "Material 1".to_owned(),
        });
        assert_eq!(
            "Material 1",
            mat_table.get_simulation_material(id).unwrap().name
        );
    }
}
