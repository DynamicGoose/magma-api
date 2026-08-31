use std::ops::{Deref, DerefMut};

use magma_app::{
    Module,
    magma_ecs::{Component, query::Query, resource::ResMut, world::UnsafeWorldMut},
};

use crate::{RenderSync, render_state::RenderState};

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct RenderEntity(usize);

impl RenderEntity {
    pub const fn id(&self) -> usize {
        self.0
    }
}

impl Deref for RenderEntity {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RenderEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct MainEntity(usize);

impl MainEntity {
    pub const fn id(&self) -> usize {
        self.0
    }
}

impl Deref for MainEntity {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MainEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct SyncToRenderWorld;

fn sync_entities(
    query: Query<(&SyncToRenderWorld, &mut RenderEntity)>,
    mut render_state: ResMut<RenderState>,
) {
    for (index, main_entity) in
        unsafe { UnsafeWorldMut::new(&mut render_state.render_world).get_ref() }
            .component_store
            .get_components_ref::<MainEntity>()
            .unwrap()
    {
        if query.data.0.get(main_entity.id()).is_none() {
            render_state
                .render_world
                .component_store
                .delete_entity(index);
        };
    }

    for (index, _) in query.data.0 {
        if query.data.1.get(index).is_none() {
            let render_entity = render_state
                .render_world
                .component_store
                .create_entity(MainEntity(index))
                .unwrap();
            query.data.1.insert(index, RenderEntity(render_entity));
        }
    }
}

pub struct SyncEntityModule;

impl Module for SyncEntityModule {
    fn init(self, app: &mut magma_app::App) {
        app.world
            .resource_store
            .get_mut::<RenderState>()
            .expect("Register Render Module!")
            .render_world
            .component_store
            .register_component::<MainEntity>();
        app.add_system(RenderSync, sync_entities).unwrap();
    }
}
