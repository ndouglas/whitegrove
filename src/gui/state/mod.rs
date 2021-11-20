use rltk::{GameState, Rltk};
use specs::prelude::*;

pub mod run_mode;
pub use run_mode::*;

use crate::ecs::components::register_components;
use crate::ecs::systems::run_systems;

pub struct State {
    ecs: World,
    run_mode: RunMode,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::new();
        register_components(&mut ecs);
        State {
            ecs: ecs,
            run_mode: RunMode::Initial,
        }
    }

    pub fn run_systems(&mut self) {
        run_systems(&mut self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let ecs = &mut self.ecs;
        if let Some(new_mode) = self.run_mode.tick(ecs, ctx) {
            self.run_mode = new_mode;
        } else {
            self.run_systems();
        }
    }
}
