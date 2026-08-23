use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

use magma_ecs::{
    errors::SystemError,
    systems::{IntoSystem, system_params::SystemParam},
};

use crate::schedule::{PostUpdate, PreUpdate, Schedule, ScheduleLabel, Schedules, Startup, Update};

pub use magma_ecs;
pub use magma_ecs::World;
pub use module::Module;

pub mod module;
pub mod schedule;

pub struct App {
    pub world: World,
    runner: fn(App),
    modules: Vec<TypeId>,
    schedules: Schedules,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: Default::default(),
            runner: default_runner,
            modules: Default::default(),
            schedules: Default::default(),
        }
    }
}

impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("world", &"World { ... }")
            .field("runner", &self.runner)
            .field("modules", &self.modules)
            .field("schedules", &self.schedules)
            .finish()
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_module(&mut self, module: impl Module + 'static) {
        let id = module.type_id();
        if !self.modules.contains(&id) {
            module.init(self);
            self.modules.push(id);
        }
    }

    pub fn register_schedule(&mut self, label: impl ScheduleLabel + 'static) {
        self.schedules.insert(label, Schedule::new());
    }

    pub fn run_schedule(&mut self, label: impl ScheduleLabel + 'static) {
        self.schedules.get_mut(label).unwrap().run(&mut self.world);
    }

    pub fn add_system<In: SystemParam, Marker>(
        &mut self,
        schedule: impl ScheduleLabel + 'static,
        system: impl IntoSystem<In, Marker>,
    ) -> Result<(), SystemError> {
        match self.schedules.get_mut(schedule) {
            Some(sched) => {
                sched.add_system(system)?;
                sched.init(&mut self.world)
            }
            None => {
                self.register_schedule(schedule);
                self.add_system(schedule, system)
            }
        }
    }

    pub fn set_runner(&mut self, runner: fn(Self)) {
        self.runner = runner;
    }

    pub fn run(self) {
        (self.runner)(self);
    }
}

// default app runner
fn default_runner(mut app: App) {
    app.run_schedule(Startup);
    loop {
        app.run_schedule(PreUpdate);
        app.run_schedule(Update);
        app.run_schedule(PostUpdate);
        app.world.event_manager.clear();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
