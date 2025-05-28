/*!
This crate provides basic functionality for creating and running an [`App`].
A [`Module`] trait is also provided for implementing additional functionality.
*/
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use error::EventError;
use events::Events;
use magma_ecs::{
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
    systems::{Systems, dispatcher::Dispatcher},
};
use module::Module;

pub use magma_ecs;
pub use magma_ecs::{World, entities, rayon, resources, systems};

pub mod error;
pub mod events;
/// Support for adding [`Module`]s
pub mod module;

type SystemSlice = &'static [(fn(&World), &'static str, &'static [&'static str])];

/// The [`App`] struct holds all the apps data and defines the necessary functions and methods to operate on it.
pub struct App {
    pub world: World,
    runner: fn(App),
    modules: Vec<TypeId>,
    startup_systems: (Systems, Dispatcher),
    update_systems: (Systems, Dispatcher),
    event_systems: HashMap<TypeId, (Systems, Dispatcher)>,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            world: Default::default(),
            runner: default_runner,
            modules: vec![],
            startup_systems: Default::default(),
            update_systems: Default::default(),
            event_systems: Default::default(),
        };

        app.world.add_resource(Events::default()).unwrap();
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

    /**
    Add systems to the [`App`]'s [`World`]. Systems must take an immutable reference to [`World`].

    # Example

    ```
    use magma_app::{App, SystemType, World};

    let mut app = App::new();
    app.add_systems(SystemType::Startup, &[(example_system, "example_system", &[])]);

    fn example_system(_world: &World) {
        // E.g. change something in the World
    }
    ```
    */
    pub fn add_systems(&mut self, systemtype: SystemType, systems: SystemSlice) {
        match systemtype {
            SystemType::Startup => {
                for (run, name, deps) in systems {
                    self.startup_systems.0.add(*run, name, deps);
                }
                self.startup_systems.1 = self.startup_systems.0.to_owned().build_dispatcher();
            }
            SystemType::Update => {
                for (run, name, deps) in systems {
                    self.update_systems.0.add(*run, name, deps);
                }
                self.update_systems.1 = self.update_systems.0.to_owned().build_dispatcher();
            }
        }
    }

    pub fn register_event<E: Any + Send + Sync>(&mut self) {
        self.world
            .get_resource_mut::<Events>()
            .unwrap()
            .0
            .insert(TypeId::of::<E>(), vec![]);
        self.event_systems
            .insert(TypeId::of::<E>(), (Systems::new(), Dispatcher::default()));
    }

    pub fn add_event_systems<E: Any + Send + Sync>(
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

    /// update the [`App`] once
    pub fn update(&self) {
        self.update_systems.1.dispatch(&self.world);

        // get which events occured
        let events = self
            .world
            .get_resource::<Events>()
            .unwrap()
            .0
            .par_iter()
            .filter_map(|(type_id, events)| {
                if !events.is_empty() {
                    Some(*type_id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // dispatch systems for events
        events.par_iter().for_each(|type_id| {
            self.event_systems
                .get(type_id)
                .unwrap()
                .1
                .dispatch(&self.world);
        });

        // clear events
        self.world
            .get_resource_mut::<Events>()
            .unwrap()
            .clear_events();
    }

    /// Run the Application
    pub fn run(self) {
        self.startup_systems.1.dispatch(&self.world);
        (self.runner)(self);
    }
}

/// Used to specify if systems should be added to the `startup` or `update` loop of the [`App`]
pub enum SystemType {
    Startup,
    Update,
}

fn default_runner(app: App) {
    loop {
        app.update();
    }
}
