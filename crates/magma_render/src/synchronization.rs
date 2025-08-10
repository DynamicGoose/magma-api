use std::marker::PhantomData;

use feufeu::RenderState;
use magma_app::{
    World,
    entities::Entity,
    module::Module,
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
};

/// Functions that extract components for syncing to the render world.
pub struct SyncSystems {
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

/// Mapping from sync component data to a corresponding entity.
pub struct SyncComponentEntityMapping<C: SyncComponent>(pub C::Out, pub Entity);

/// Components of type `C` with their corresponding entity to be synced to the render world.
pub struct SyncComponents<C: SyncComponent> {
    pub components: Vec<SyncComponentEntityMapping<C>>,
}

/// Sync this entity to the render world.
pub struct SyncToRenderWorld;

/// A component, that can be synced to the render world.
pub trait SyncComponent: Send + Sync {
    type Out: Send + Sync;

    fn get_data(&self) -> Self::Out;
}

/// Module for syncing entities to the render world.
pub struct SyncModule;

impl Module for SyncModule {
    fn setup(self, app: &mut magma_app::App) {
        // add to new sync schedule, when that's ready
        app.add_systems(
            magma_app::SystemType::Update,
            &[(sync_systems, "render_sync_systems", &[])],
        );
        app.world.register_component::<SyncToRenderWorld>();
        app.world.add_resource(SyncSystems::new()).unwrap();
    }
}

pub struct SyncComponentModule<C: 'static>(PhantomData<C>);

impl<C: SyncComponent> Module for SyncComponentModule<C> {
    fn setup(self, app: &mut magma_app::App) {
        app.world
            .get_resource_mut::<SyncSystems>()
            .unwrap()
            .systems
            .push(sync_component::<C>);
        app.world
            .get_resource_mut::<RenderState>()
            .unwrap()
            .render_world
            .register_component::<C::Out>();
    }
}

fn sync_component<C: SyncComponent + 'static>(world: &World) {
    if let Ok(query) = world.query::<(SyncToRenderWorld, C)>() {
        query.iter().for_each(|entity| {
            world
                .get_resource_mut::<SyncComponents<C>>()
                .unwrap()
                .components
                .push(SyncComponentEntityMapping::<C>(
                    entity.get_component::<C>().unwrap().get_data(),
                    entity.into(),
                ))
        });
    }
}

fn sync_systems(world: &World) {
    world
        .get_resource::<SyncSystems>()
        .unwrap()
        .systems
        .par_iter()
        .for_each(|f| f(world));
}

// TODO: - [ ] sync directly in one step to the RenderState resource.
//       - [ ] remove SyncSystems
//       - [ ] render world entity to main world entity mappings.
