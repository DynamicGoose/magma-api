/*!
This crate provides basic functionality for creating and running an [`App`].
A [`Module`] trait is also provided for implementing additional functionality.
*/
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use magma_ecs::{
    error::EventError,
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
    systems::{Systems, dispatcher::Dispatcher},
};
use module::Module;

pub use magma_ecs;
pub use magma_ecs::{World, entities, rayon, resources, systems};
pub use schedule::AppSchedule;

use crate::{
    error::ScheduleError,
    schedule::{PostUpdate, PreUpdate, Startup, Update},
};

pub mod error;
/// Support for adding [`Module`]s
pub mod module;
/// The [`AppSchedule`] trait and default schedules.
pub mod schedule;

type SystemSlice = &'static [(fn(&World), &'static str, &'static [&'static str])];

/// The [`App`] struct holds all the apps data and defines the necessary functions and methods to operate on it.
pub struct App {
    pub world: World,
    runner: fn(App),
    modules: Vec<TypeId>,
    systems: HashMap<TypeId, (Systems, Dispatcher)>,
    event_systems: HashMap<TypeId, (Systems, Dispatcher)>,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            world: Default::default(),
            runner: default_runner,
            modules: vec![],
            systems: Default::default(),
            event_systems: Default::default(),
        };

        app.register_schedule::<Startup>();
        app.register_schedule::<PreUpdate>();
        app.register_schedule::<Update>();
        app.register_schedule::<PostUpdate>();

        app
    }
}

impl App {
    /// Create a new [`App`]
    pub fn new() -> Self {
        Self::default()
    }

    /**
    Add a [`Module`] to the [`App`]. If it already exists, nothing happens.

    # Example

    ```
    use magma_app::{module::Module, App};

    let mut app = App::new();
    app.add_module(ExampleModule);

    struct ExampleModule;

    impl Module for ExampleModule {
        fn setup(self, app: &mut App) {
            // Setup the module
            // E.g. register components to the World or add resources
        }
    }
    ```
    */
    pub fn add_module(&mut self, module: impl Module + 'static) {
        let type_id = module.type_id();
        if !self.modules.contains(&type_id) {
            self.modules.push(type_id);
            module.setup(self);
        }
    }

    /// Register an [`AppSchedule`].
    pub fn register_schedule<S: AppSchedule + 'static>(&mut self) {
        self.systems.insert(TypeId::of::<S>(), Default::default());
    }

    /// Run an [`AppSchedule`].
    pub fn run_schedule<S: AppSchedule + 'static>(&self) -> Result<(), ScheduleError> {
        self.systems
            .get(&TypeId::of::<S>())
            .ok_or(ScheduleError::ScheduleNotRegistered)?
            .1
            .dispatch(&self.world);
        Ok(())
    }

    /**
    Add systems to the [`App`]'s [`World`]. Systems must take an immutable reference to [`World`].

    # Errors

    Returns an error, when the schedule isn't registered.

    # Example

    ```
    use magma_app::{App, World};
    use magma_app::schedule::Startup;

    let mut app = App::new();
    app.add_systems::<Startup>(&[(example_system, "example_system", &[])]).unwrap();

    fn example_system(_world: &World) {
        // E.g. change something in the World
    }
    ```
    */
    pub fn add_systems<S: AppSchedule + 'static>(
        &mut self,
        systems: SystemSlice,
    ) -> Result<(), ScheduleError> {
        let schedule = self
            .systems
            .get_mut(&TypeId::of::<S>())
            .ok_or(ScheduleError::ScheduleNotRegistered)?;
        for (run, name, deps) in systems {
            schedule.0.add(*run, name, deps);
        }

        schedule.1 = schedule.0.to_owned().build_dispatcher();
        Ok(())
    }

    pub fn register_event<E: Any + Send + Sync + Clone>(&mut self) {
        self.world.register_event::<E>();
        self.event_systems
            .insert(TypeId::of::<E>(), (Systems::new(), Dispatcher::default()));
    }

    pub fn add_event_systems<E: Any + Send + Sync + Clone>(
        &mut self,
        systems: SystemSlice,
    ) -> Result<(), EventError> {
        let event_systems = self
            .event_systems
            .get_mut(&TypeId::of::<E>())
            .ok_or(EventError::EventNotRegistered)?;
        for (run, name, deps) in systems {
            event_systems.0.add(*run, name, deps);
        }
        event_systems.1 = event_systems.0.to_owned().build_dispatcher();
        Ok(())
    }

    /// Set the runner of the [`App`]
    pub fn set_runner(&mut self, runner: fn(App)) {
        self.runner = runner;
    }

    /// Process pending events.
    pub fn process_events(&self) {
        let events = self.world.get_pending_events();
        // dispatch systems for events
        events.par_iter().for_each(|type_id| {
            self.event_systems
                .get(type_id)
                .unwrap()
                .1
                .dispatch(&self.world);
        });

        self.world.clear_events();
    }

    /// Run the Application
    pub fn run(self) {
        (self.runner)(self);
    }
}

fn default_runner(app: App) {
    app.run_schedule::<Startup>().unwrap();
    loop {
        app.run_schedule::<PreUpdate>().unwrap();
        app.run_schedule::<Update>().unwrap();
        app.run_schedule::<PostUpdate>().unwrap();
        app.process_events();
    }
}
