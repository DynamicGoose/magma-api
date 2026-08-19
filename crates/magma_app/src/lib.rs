use std::{any::TypeId, fmt::Debug};

use magma_ecs::World;

use crate::schedule::{Schedule, ScheduleLabel, Schedules};

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

    pub fn add_schedule(&mut self, schedule: impl ScheduleLabel + 'static) {
        self.schedules.insert(schedule, Schedule::new());
    }
}

fn default_runner(app: App) {}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
