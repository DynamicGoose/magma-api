use std::marker::PhantomData;

use feufeu::RenderState;
use magma_app::{World, module::Module};

use crate::sync_module::{EntityRenderEntityMap, SyncComponent, SyncSystems, SyncToRenderWorld};

pub struct SyncComponentModule<C: SyncComponent>(PhantomData<C>);

impl<C: SyncComponent + 'static> Module for SyncComponentModule<C> {
    fn setup(self, app: &mut magma_app::App) {
        app.world
            .get_resource_mut::<RenderState>()
            .unwrap()
            .render_world
            .register_component::<C::Out>();

        app.world
            .get_resource_mut::<SyncSystems>()
            .unwrap()
            .push(sync_component_to_render_world_system::<C>);
    }
}

fn sync_component_to_render_world_system<C: SyncComponent + 'static>(world: &World) {
    world
        .query::<(SyncToRenderWorld, C)>()
        .unwrap()
        .iter()
        .for_each(|entity| {
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
}
