use magma_ecs::World;
use module::Module;

pub mod module;

pub struct App<'a> {
    pub world: World,
    startup_systems: Vec<&'a dyn Fn(&mut World)>,
    update_systems: Vec<&'a dyn Fn(&mut World)>,
}

impl<'a> App<'a> {
    pub fn add_module(&mut self, module: impl Module) {
        module.setup(self);
    }

    pub fn add_systems(&mut self, systemtype: SystemType, systems: Vec<&'a dyn Fn(&mut World)>) {
        match systemtype {
            SystemType::Startup => systems
                .iter()
                .for_each(|system| self.startup_systems.push(*system)),
            SystemType::Update => systems
                .iter()
                .for_each(|system| self.update_systems.push(*system)),
        }
    }

    pub fn run(&mut self) {
        self.world.startup(vec![], self.startup_systems.clone());
        self.world.update(
            &Self::update_condition,
            vec![],
            self.startup_systems.clone(),
        );
    }

    fn update_condition(_: &World) -> bool {
        true
    }
}

pub enum SystemType {
    Startup,
    Update,
}

#[cfg(test)]
mod tests {}
