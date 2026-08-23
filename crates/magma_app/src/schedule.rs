use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use magma_ecs::{
    Dispatcher, SystemGraph, World,
    errors::SystemError,
    systems::{IntoSystem, system_params::SystemParam},
};

#[derive(Clone, Debug, Default)]
pub struct Schedule {
    dispatcher: Option<Dispatcher>,
    graph: SystemGraph,
}

impl Schedule {
    pub const fn new() -> Self {
        Self {
            dispatcher: None,
            graph: SystemGraph::new(),
        }
    }

    /// Initialize this [`Schedule`] on the provided [`World`](magma_ecs::World).
    ///
    /// # Errors
    ///
    /// This function will return an error if the systems in this [`Schedule`] could not be converted into a [`Dispatcher`](magma_ecs::Dispatcher).
    pub fn init(&mut self, world: &mut World) -> Result<(), SystemError> {
        let dispatcher = self.graph.clone().into_dispatcher(world)?;

        self.dispatcher = Some(dispatcher);
        Ok(())
    }

    /// Run the [`Schedule`] on the provided [`World`](magma_ecs::World).
    ///
    /// # Panics
    ///
    /// Panics if the [`Schedule`] has not been initialized.
    pub fn run(&mut self, world: &mut World) {
        self.dispatcher.as_mut().unwrap().dispatch(world);
    }

    pub fn add_system<In: SystemParam, Marker>(
        &mut self,
        system: impl IntoSystem<In, Marker>,
    ) -> Result<(), SystemError> {
        self.graph.add_system(system)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Schedules {
    map: HashMap<TypeId, Schedule>,
}

impl Schedules {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        label: impl ScheduleLabel + 'static,
        schedule: Schedule,
    ) -> Option<Schedule> {
        self.map.insert(label.type_id(), schedule)
    }

    pub fn remove(&mut self, label: impl ScheduleLabel + 'static) -> Option<Schedule> {
        self.map.remove(&label.type_id())
    }

    pub fn get(&self, label: impl ScheduleLabel + 'static) -> Option<&Schedule> {
        self.map.get(&label.type_id())
    }

    pub fn get_mut(&mut self, label: impl ScheduleLabel + 'static) -> Option<&mut Schedule> {
        self.map.get_mut(&label.type_id())
    }
}

pub trait ScheduleLabel: Copy {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Startup;
impl ScheduleLabel for Startup {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct PreUpdate;
impl ScheduleLabel for PreUpdate {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Update;
impl ScheduleLabel for Update {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct PostUpdate;
impl ScheduleLabel for PostUpdate {}
