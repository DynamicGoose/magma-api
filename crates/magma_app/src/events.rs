use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::error::EventError;

#[derive(Default)]
pub struct Events(pub(crate) HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>);

impl Events {
    pub fn push_event<E: Any + Send + Sync>(&mut self, event: E) -> Result<(), EventError> {
        self.0
            .get_mut(&event.type_id())
            .ok_or(EventError::EventNotRegistered)?
            .push(Box::new(event));
        Ok(())
    }

    pub fn get_events<E: Any + Send + Sync>(
        &self,
    ) -> Result<&Vec<Box<dyn Any + Send + Sync>>, EventError> {
        self.0
            .get(&TypeId::of::<E>())
            .ok_or(EventError::EventNotRegistered)
    }

    pub(crate) fn clear_events(&mut self) {
        self.0.values_mut().for_each(|e| e.clear());
    }
}
