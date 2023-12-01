use magma_ecs::World;
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
mod tests {
    use magma_ecs::World;

    use crate::App;
    use crate::SystemType;

    #[test]
    fn add_systems() {
        let mut app = App::new();
        app.add_systems(
            SystemType::Startup,
            (vec![&ref_system_startup], vec![&mut_system_startup]),
        );
        app.add_systems(SystemType::Update, (vec![], vec![&update_resource]));
        app.run(&update_condition);
    }

    fn ref_system_startup(_: &World) {
        println!("startup_ref");
    }

    fn mut_system_startup(world: &mut World) {
        world.add_resource(0_u32);
        world.register_component::<u32>();
        world.spawn().with_component(10_u32).unwrap();
    }

    fn update_resource(world: &mut World) {
        *world.get_resource_mut::<u32>().unwrap() += 1;
    }

    fn update_condition(world: &World) -> bool {
        *world.get_resource::<u32>().unwrap() <= 42949672
    }
}
