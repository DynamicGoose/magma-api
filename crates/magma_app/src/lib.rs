pub use magma_ecs::World;
use module::Module;

pub mod module;

type Systems<'a> = (Vec<&'a dyn Fn(&World)>, Vec<&'a dyn Fn(&mut World)>);

/// The `App` struct holds all the apps data and defines the necessary functions and methods to operate on it.
#[derive(Default)]
pub struct App<'a> {
    pub world: World,
    startup_systems: Systems<'a>,
    update_systems: Systems<'a>,
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

    /// Run the application
    pub fn run(&mut self, update_condition: &dyn Fn(&World) -> bool) {
        self.world.startup(
            self.startup_systems.0.clone(),
            self.startup_systems.1.clone(),
        );
        self.world.update(
            update_condition,
            self.update_systems.0.clone(),
            self.update_systems.1.clone(),
        );
    }
}

/// Used to specify if systems should be added to the `startup` or `update` loop of the `World`
pub enum SystemType {
    Startup,
    Update,
}

#[cfg(test)]
mod tests {}
