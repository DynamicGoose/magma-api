use std::marker::PhantomData;

use magma_app::{
    Module,
    magma_ecs::{Component, component::ComponentsRef, resource::ResMut, world::UnsafeWorldMut},
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
    components: ComponentsRef<C>,
    render_entities: ComponentsRef<RenderEntity>,
    mut render_state: ResMut<RenderState>,
) {
    // SAFETY: Not borrowing the same data
    let main_entities = unsafe { UnsafeWorldMut::new(&mut render_state.render_world).get_ref() }
        .component_store
        .get_components_ref::<MainEntity>()
        .unwrap();
    let render_components = render_state
        .render_world
        .component_store
        .get_components_mut::<C::Out>()
        .unwrap();
    for (index, main_entity) in main_entities {
        if components.get(main_entity.id()).is_none() {
            render_components.remove(index);
        }
    }

    for (index, c) in components.iter() {
        match render_entities.get(index) {
            Some(render_entity) => {
                render_components.insert(render_entity.id(), c.extract(index, &()));
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
