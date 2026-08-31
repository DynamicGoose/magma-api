use std::ops::{Deref, DerefMut};

pub struct SimulationMaterial {
    pub name: String, // pub mechanical_properties: MechanicalProperties,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct SimulationMaterialId(usize);

impl SimulationMaterialId {
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    pub const fn id(&self) -> usize {
        self.0
    }
}

impl Deref for SimulationMaterialId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SimulationMaterialId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
