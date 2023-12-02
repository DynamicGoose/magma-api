pub use magma_ecs::World;
use module::Module;

pub mod module;

type Systems<'a> = (Vec<&'a dyn Fn(&World)>, Vec<&'a dyn Fn(&mut World)>);

#[derive(Default)]
pub struct App<'a> {
    pub world: World,
    startup_systems: Systems<'a>,
    update_systems: Systems<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_module(&mut self, module: impl Module) {
        module.setup(self);
    }

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

pub enum SystemType {
    Startup,
    Update,
}

#[cfg(test)]
mod tests {}
