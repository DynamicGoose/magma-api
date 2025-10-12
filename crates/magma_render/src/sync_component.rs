use std::{
    any::TypeId,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
};

use feufeu::RenderState;
use magma_app::{World, entities::Entity, magma_ecs::component::Component, module::Module};

use crate::{SyncSchedule, sync_entity::RenderEntity};

/// A component, that can be synced to the render world.
pub trait SyncComponent: Component {
    type Out: Send + Sync;

    fn get_data(entity: Entity, world: &World) -> Self::Out;
}

fn sync_component<C: SyncComponent + 'static>(world: &World) {
    let render_state = world.get_resource::<RenderState>().unwrap();

    world
        .query_added::<(C,)>()
        .unwrap()
        .iter()
        .for_each(|entity| match entity.get_component::<RenderEntity>() {
            Ok(render_entity) => render_state
                .render_world
                .assign_components((C::get_data(entity.into(), world),), render_entity.entity())
                .unwrap(),
            Err(_) => (),
        });

    world
        .query_changed::<(C,)>()
        .unwrap()
        .iter()
        .for_each(|entity| match entity.get_component::<RenderEntity>() {
            Ok(render_entity) => {
                *render_state
                    .render_world
                    .get_component_mut::<C::Out>(render_entity.entity())
                    .unwrap() = C::get_data(entity.into(), world)
            }
            Err(_) => (),
        });

    world
        .query_removed::<(C,)>()
        .unwrap()
        .iter()
        .for_each(|entity| match entity.get_component::<RenderEntity>() {
            Ok(render_entity) => render_state
                .render_world
                .purge_component::<C::Out>(render_entity.entity())
                .unwrap(),
            Err(_) => (),
        });
}

#[derive(Default)]
pub struct SyncComponentModule<C: SyncComponent + 'static>(PhantomData<C>);

impl<C: SyncComponent + 'static> Module for SyncComponentModule<C> {
    fn setup(self, app: &mut magma_app::App) {
        app.world
            .get_resource_mut::<RenderState>()
            .unwrap()
            .render_world
            .register_component::<C::Out>();

        let mut hasher = DefaultHasher::new();
        TypeId::of::<C>().hash(&mut hasher);

        let system_name = "sync_component_".to_string() + &hasher.finish().to_string();

        app.add_systems::<SyncSchedule>(vec![(
            sync_component::<C>,
            system_name,
            vec!["sync_entities".to_string()],
        )])
        .unwrap();
    }
}
