use feufeu::RenderState;
use magma_app::{World, entities::Entity, magma_ecs::component::Component, module::Module};

use crate::SyncSchedule;

/// A component that is added to main world entities synced to the render world.
/// It contains the corresponding render world entity.
#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct RenderEntity(Entity);

impl RenderEntity {
    /// Get the render world entity's id.
    pub const fn id(&self) -> usize {
        self.0.id()
    }

    /// Get the render world entity
    pub const fn entity(&self) -> Entity {
        self.0
    }
}

/// A component that is added to render world entities synced with the main world.
/// It contains the corresponding main world entity.
#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct MainEntity(Entity);

impl MainEntity {
    /// Get the main world entity's id.
    pub const fn id(&self) -> usize {
        self.0.id()
    }

    /// Get the main world entity.
    pub const fn entity(&self) -> Entity {
        self.0
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct SyncToRenderWorld;

fn sync_entities(world: &World) {
    let render_state = world.get_resource::<RenderState>().unwrap();
    world
        .query_added::<(SyncToRenderWorld,)>()
        .unwrap()
        .iter()
        .for_each(|main_entity| {
            let render_entity = render_state
                .render_world
                .create_entity((MainEntity(main_entity.into()),))
                .unwrap();
            main_entity
                .assign_components((RenderEntity(render_entity),))
                .unwrap();
        });

    world
        .query_removed::<(SyncToRenderWorld,)>()
        .unwrap()
        .iter()
        .for_each(|main_entity| {
            render_state
                .render_world
                .purge_entity(
                    main_entity
                        .get_component::<RenderEntity>()
                        .unwrap()
                        .entity(),
                )
                .unwrap();
            main_entity.delete_component::<RenderEntity>().unwrap();
        });
}

/// This [`Module`](magma_app::module::Module) is responsible for synchronizing entities from the main world to their render world counterparts.
/// Add a [`SyncToRenderWorld`] component to the main world entity to sync it to the render world automatically.
pub struct SyncEntityModule;

impl Module for SyncEntityModule {
    fn setup(self, app: &mut magma_app::App) {
        app.add_systems::<SyncSchedule>(vec![(sync_entities, "sync_entities".to_string(), vec![])])
            .unwrap();

        app.world.register_component::<RenderEntity>();
        app.world.register_component::<SyncToRenderWorld>();
        app.world
            .get_resource_mut::<RenderState>()
            .unwrap()
            .render_world
            .register_component::<MainEntity>();
    }
}
