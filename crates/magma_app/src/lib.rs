pub use magma_ecs::World;
use module::Module;

pub mod module;

type Systems<'a> = (Vec<&'a dyn Fn(&World)>, Vec<&'a dyn Fn(&mut World)>);

/// The `App` struct holds all the apps data and defines the necessary functions and methods to operate on it.
pub struct App<'a> {
    /// The `World` holds all the data of the entity-component-system
    pub world: World,
    /// The runner of the `App`
    pub runner: &'a dyn Fn(App),
    pub startup_systems: Systems<'a>,
    pub update_systems: Systems<'a>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            world: Default::default(),
            runner: &default_runner,
            startup_systems: Default::default(),
            update_systems: Default::default(),
        }
    }
}

impl<'a> App<'a> {
    /// Create a new `App`
    pub fn new() -> Self {
        Self::default()
    }

    /**
    Add a `Module` to the `App`.
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
    pub fn add_module(&mut self, module: impl Module) {
        module.setup(self);
    }

    /**
    Add systems to the `App`'s `World`. Systems must take either an immutable or a mutable reference to `World`.
    ```
    use magma_app::{App, SystemType, World};

    let mut app = App::new();
    app.add_systems(SystemType::Startup, (vec![&example_system_ref], vec![&example_system_mut]));

    fn example_system_ref(_world: &World) {
        // Do something
    }

    fn example_system_mut(_world: &mut World) {
        // E.g. change something in the World
    }
    ```
    */
    pub fn add_systems(&mut self, systemtype: SystemType, systems: Systems<'a>) {
        match systemtype {
            SystemType::Startup => {
                systems
                    .0
                    .iter()
                    .for_each(|system| self.startup_systems.0.push(*system));
                systems
                    .1
                    .iter()
                    .for_each(|system| self.startup_systems.1.push(*system));
            }
            SystemType::Update => {
                systems
                    .0
                    .iter()
                    .for_each(|system| self.update_systems.0.push(*system));
                systems
                    .1
                    .iter()
                    .for_each(|system| self.update_systems.1.push(*system));
            }
        }
    }

    /// Run the Application
    pub fn run(self) {
        (self.runner)(self);
    }
}

pub fn default_runner(mut app: App) {
    app.world
        .startup(app.startup_systems.0.clone(), app.startup_systems.1.clone());
    app.world.update(
        &default_update_condition,
        app.update_systems.0.clone(),
        app.update_systems.1.clone(),
    );
}

fn default_update_condition(_: &World) -> bool {
    true
}

/// Used to specify if systems should be added to the `startup` or `update` loop of the `World`
pub enum SystemType {
    Startup,
    Update,
}

#[cfg(test)]
mod tests {}