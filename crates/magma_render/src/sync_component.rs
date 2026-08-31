use std::marker::PhantomData;

use magma_app::{
    Module,
    magma_ecs::{Component, query::Query, resource::ResMut, world::UnsafeWorldMut},
};

use crate::{
    RenderSync,
    render_state::RenderState,
    sync_entities::{MainEntity, RenderEntity},
};

/// A component, that can be synced to the render world.
pub trait SyncComponent: Component {
    type Out: Component;
    type ExtraData;

    fn extract(&self, index: usize, data: &Self::ExtraData) -> Self::Out;
}

fn sync_component<C: SyncComponent<ExtraData = ()> + 'static>(
    query: Query<(&C, &RenderEntity)>,
    mut render_state: ResMut<RenderState>,
) {
    let render_query = Query::<(&mut C::Out, &MainEntity)>::new(UnsafeWorldMut::new(
        &mut render_state.render_world,
    ));
    for (index, main_entity) in render_query.data.1 {
        if query.data.0.get(main_entity.id()).is_none() {
            render_query.data.0.remove(index);
        }
    }

    for (index, c) in query.data.0 {
        match query.data.1.get(index) {
            Some(render_entity) => {
                render_query
                    .data
                    .0
                    .insert(render_entity.id(), c.extract(index, &()));
            }
            None => (),
        }
    }
}

#[derive(Default)]
pub struct SyncComponentModule<C: SyncComponent + 'static>(PhantomData<C>);

impl<C: SyncComponent<ExtraData = ()> + 'static> Module for SyncComponentModule<C> {
    fn init(self, app: &mut magma_app::App) {
        app.world
            .resource_store
            .get_mut::<RenderState>()
            .expect("Register Render Module!")
            .render_world
            .component_store
            .register_component::<C::Out>();
        app.add_system(RenderSync, sync_component::<C>).unwrap()
    }
}
