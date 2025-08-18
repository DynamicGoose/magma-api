use std::marker::PhantomData;

use feufeu::RenderState;
use magma_app::{
    World,
    entities::Entity,
    module::Module,
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
};

use crate::sync_module::{EntityRenderEntityMap, SyncComponent, SyncSystems, SyncToRenderWorld};

pub struct SyncComponentModule<C: SyncComponent>(PhantomData<C>);

struct EntityMap<C: SyncComponent>(Vec<Entity>, PhantomData<C>);

impl<C: SyncComponent + 'static> Module for SyncComponentModule<C> {
    fn setup(self, app: &mut magma_app::App) {
        app.world
            .get_resource_mut::<RenderState>()
            .unwrap()
            .render_world
            .register_component::<C::Out>();

        app.world
            .add_resource(EntityMap::<C>(vec![], PhantomData::default()))
            .unwrap();

        app.world
            .get_resource_mut::<SyncSystems>()
            .unwrap()
            .push(sync_component_to_render_world_system::<C>);
    }
}

fn sync_component_to_render_world_system<C: SyncComponent + 'static>(world: &World) {
    let mut map = world.get_resource_mut::<EntityMap<C>>().unwrap();
    world
        .query::<(SyncToRenderWorld, C)>()
        .unwrap()
        .iter()
        .for_each(|entity| {
            if !map.0.contains(&entity.into()) {
                map.0.push(entity.into());
            }
            let render_entity = world
                .get_resource_mut::<EntityRenderEntityMap>()
                .unwrap()
                .entity_to_render_entity
                .get(&entity.into())
                .unwrap()
                .to_owned();
            world
                .get_resource::<RenderState>()
                .unwrap()
                .render_world
                .assign_components(
                    (entity.get_component::<C>().unwrap().get_data(),),
                    render_entity,
                )
                .unwrap();
        });
    // sync deleted components
    if map.0.len() > world.query::<(SyncToRenderWorld, C)>().unwrap().len() {
        map.0 = map
            .0
            .par_iter()
            .filter_map(|entity| match world.get_component::<C>(*entity) {
                Ok(_) => Some(*entity),
                Err(_) => {
                    world
                        .get_resource::<RenderState>()
                        .unwrap()
                        .render_world
                        .delete_component::<C::Out>(
                            world
                                .get_resource::<EntityRenderEntityMap>()
                                .unwrap()
                                .entity_to_render_entity
                                .get(entity)
                                .unwrap()
                                .to_owned(),
                        )
                        .unwrap();
                    None
                }
            })
            .collect::<Vec<Entity>>();
    }
}
