use std::ops::{Deref, DerefMut};

pub struct RenderMaterial {
    pub name: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct RenderMaterialId(usize);

impl RenderMaterialId {
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    pub const fn id(&self) -> usize {
        self.0
    }
}

impl Deref for RenderMaterialId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RenderMaterialId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
