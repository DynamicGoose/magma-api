/*!
This crate provides basic functionality for creating and running an [`App`].
A [`Module`] trait is also provided for implementing additional functionality.
*/
use std::any::{Any, TypeId};

pub use magma_ecs::World;
use module::Module;

/// Support for adding [`Module`]s
pub mod module;

type Systems = Vec<fn(&World)>;

/// The [`App`] struct holds all the apps data and defines the necessary functions and methods to operate on it.
pub struct App {
    pub world: World,
    runner: fn(App),
    modules: Vec<TypeId>,
    startup_systems: Systems,
    update_systems: Systems,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: Default::default(),
            runner: default_runner,
            modules: vec![],
            startup_systems: Default::default(),
            update_systems: Default::default(),
        }
    }
}

impl App {
    /// Create a new [`App`]
    pub fn new() -> Self {
        Self::default()
    }

    /**
    Add a [`Module`] to the [`App`].
    ```
    use magma_app::{module::Module, App};

    let mut app = App::new();
    app.add_module(ExampleModule);

    struct ExampleModule;

    impl Module for ExampleModule {
        fn setup(&self, app: &mut App) {
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
    Add systems to the [`App`]'s [`World`]. Systems must take either an immutable or a mutable reference to [`World`].
    ```
    use magma_app::{App, SystemType, World};

    let mut app = App::new();
    app.add_systems(SystemType::Startup, vec![example_system]);

    fn example_system(_world: &World) {
        // E.g. change something in the World
    }
    ```
    */
    pub fn add_systems(&mut self, systemtype: SystemType, mut systems: Systems) {
        match systemtype {
            SystemType::Startup => self.startup_systems.append(&mut systems),
            SystemType::Update => self.update_systems.append(&mut systems),
        }
    }

    /// Set the runner of the [`App`]
    pub fn set_runner(&mut self, runner: fn(App)) {
        self.runner = runner;
    }

    /// update the [`App`] once
    pub fn update(&self) {
        self.world.update(&self.update_systems);
    }

    /// Run the Application
    pub fn run(self) {
        self.world.update(&self.startup_systems);
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
