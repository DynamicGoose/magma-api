use std::collections::HashMap;

use feufeu::wgpu::Surface;
use magma_app::{entities::Entity, rayon::iter::IntoParallelRefIterator};
use magma_windowing::raw_handle::RawHandleWrapper;

pub struct ExtractedWindow;

pub struct ExtractedWindows<'a> {
    entity_to_render_entity: HashMap<Entity, Entity>,
    render_entity_to_entity: HashMap<Entity, Entity>,
    render_entity_to_window: HashMap<Entity, (RawHandleWrapper, Surface<'a>)>,
}

impl<'a> ExtractedWindows<'a> {
    pub fn new() -> Self {
        Self {
            entity_to_render_entity: HashMap::new(),
            render_entity_to_entity: HashMap::new(),
            render_entity_to_window: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        entity: Entity,
        render_entity: Entity,
        raw_handle: RawHandleWrapper,
        surface: Surface<'a>,
    ) {
        self.entity_to_render_entity.insert(entity, render_entity);
        self.render_entity_to_entity.insert(render_entity, entity);
        self.render_entity_to_window
            .insert(render_entity, (raw_handle, surface));
    }

    pub fn remove_through_entity(&mut self, entity: &Entity) {
        let render_entity = *self.entity_to_render_entity.get(entity).unwrap();

        self.entity_to_render_entity.remove(entity).unwrap();
        self.render_entity_to_entity.remove(&render_entity).unwrap();
        self.render_entity_to_window.remove(&render_entity).unwrap();
    }

    pub fn remove_through_render_entity(&mut self, render_entity: &Entity) {
        let entity = *self.render_entity_to_entity.get(render_entity).unwrap();

        self.entity_to_render_entity.remove(&entity).unwrap();
        self.render_entity_to_entity.remove(render_entity).unwrap();
        self.render_entity_to_window.remove(render_entity).unwrap();
    }

    pub fn get_through_entity(&self, entity: &Entity) -> &(RawHandleWrapper, Surface) {
        self.render_entity_to_window
            .get(self.entity_to_render_entity.get(entity).unwrap())
            .unwrap()
    }

    pub fn get_through_render_entity(
        &self,
        render_entity: &Entity,
    ) -> &(RawHandleWrapper, Surface) {
        self.render_entity_to_window.get(render_entity).unwrap()
    }

    pub fn get_mut_through_entity(
        &'a mut self,
        entity: &Entity,
    ) -> &'a mut (RawHandleWrapper, Surface<'a>) {
        self.render_entity_to_window
            .get_mut(self.entity_to_render_entity.get(entity).unwrap())
            .unwrap()
    }

    pub fn get_mut_through_render_entity(
        &'a mut self,
        render_entity: &Entity,
    ) -> &'a mut (RawHandleWrapper, Surface<'a>) {
        self.render_entity_to_window.get_mut(render_entity).unwrap()
    }

    pub fn get_render_entity(&self, entity: &Entity) -> Entity {
        *self.entity_to_render_entity.get(entity).unwrap()
    }

    pub fn iter_windows(
        &self,
    ) -> std::collections::hash_map::Values<'_, Entity, (RawHandleWrapper, Surface<'a>)> {
        self.render_entity_to_window.values()
    }
}
