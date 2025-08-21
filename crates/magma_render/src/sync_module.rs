use std::collections::HashMap;

use feufeu::RenderState;
use magma_app::{
    World,
    entities::Entity,
    module::Module,
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
};

use crate::SyncSchedule;

/// Module for syncing entities to the render world.
pub struct SyncModule;

impl Module for SyncModule {
    fn setup(self, app: &mut magma_app::App) {
        // add to new sync schedule, when that's ready
        app.add_systems::<SyncSchedule>(&[
            (sync_entities, "render_sync_entities", &[]),
            (
                sync_systems,
                "render_sync_systems",
                &["render_sync_entities"],
            ),
        ])
        .unwrap();
        app.world.register_component::<SyncToRenderWorld>();
        app.world
            .add_resource(EntityRenderEntityMap::new())
            .unwrap();
        app.world.add_resource(SyncSystems::new()).unwrap();

        app.world
            .get_resource_mut::<RenderState>()
            .unwrap()
            .render_world
            .register_component::<RenderEntity>();
    }
}

fn sync_entities(world: &World) {
    let mut map = world.get_resource_mut::<EntityRenderEntityMap>().unwrap();
    let render_state = world.get_resource::<RenderState>().unwrap();

    // Sync new entities to render world.
    world
        .query::<(SyncToRenderWorld,)>()
        .unwrap()
        .iter()
        .for_each(|entity| {
            if !map.entity_to_render_entity.contains_key(&entity.into()) {
                let render_entity = render_state
                    .render_world
                    .create_entity((RenderEntity,))
                    .unwrap();

                map.insert(entity.into(), render_entity);
            }
        });

    // Sync deleted entities to render world and map.
    render_state
        .render_world
        .query::<(RenderEntity,)>()
        .unwrap()
        .iter()
        .for_each(|render_entity| {
            let entity = map
                .render_entity_to_entity
                .get(&render_entity.into())
                .unwrap()
                .to_owned();
            if world.get_component::<SyncToRenderWorld>(entity).is_err() {
                map.delete_through_render_entity(&render_entity.into());
                render_entity.delete();
            }
        });
}

fn sync_systems(world: &World) {
    world
        .get_resource::<SyncSystems>()
        .unwrap()
        .systems
        .par_iter()
        .for_each(|f| f(world));
}

pub(crate) struct SyncSystems {
    systems: Vec<fn(&World)>,
}

impl SyncSystems {
    pub fn new() -> Self {
        Self { systems: vec![] }
    }

    pub fn push(&mut self, system: fn(&World)) {
        self.systems.push(system);
    }
}

pub struct RenderEntity;

#[derive(Debug)]
pub(crate) struct EntityRenderEntityMap {
    pub entity_to_render_entity: HashMap<Entity, Entity>,
    pub render_entity_to_entity: HashMap<Entity, Entity>,
}

impl EntityRenderEntityMap {
    fn new() -> Self {
        Self {
            entity_to_render_entity: HashMap::new(),
            render_entity_to_entity: HashMap::new(),
        }
    }

    pub fn delete_through_render_entity(&mut self, render_entity: &Entity) {
        let entity = self.render_entity_to_entity.get(render_entity).unwrap();

        self.entity_to_render_entity.remove(entity);
        self.render_entity_to_entity.remove(render_entity);
    }

    pub fn insert(&mut self, entity: Entity, render_entity: Entity) {
        self.entity_to_render_entity.insert(entity, render_entity);
        self.render_entity_to_entity.insert(render_entity, entity);
    }
}

/// Sync this entity to the render world.
pub struct SyncToRenderWorld;

/// A component, that can be synced to the render world.
pub trait SyncComponent: Send + Sync {
    type Out: Send + Sync;

    fn get_data(&self) -> Self::Out;
}
