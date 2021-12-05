use rltk::{GameState, Rltk};
use specs::prelude::*;

pub mod run_mode;
pub use run_mode::*;

use crate::ecs::components::*;
use crate::ecs::dispatcher::{get_new_dispatcher, UnifiedDispatcher};
use crate::ecs::resources::Tick;
use crate::effects::run_effects_queue;
use crate::garbage_collector::collect_garbage;

pub struct State {
    run_mode: RunMode,
    dispatcher: Box<dyn UnifiedDispatcher + 'static>,
    ecs: World,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::new();
        register_components(&mut ecs);
        State {
            ecs: ecs,
            run_mode: RunMode::Initial,
            dispatcher: get_new_dispatcher(),
        }
    }

    pub fn run_systems(&mut self, ctx: &mut Rltk) {
        if self.run_mode.should_maintain_ecs() {
            self.dispatcher.run_now(&mut self.ecs);
            self.ecs.maintain();
            collect_garbage(&mut self.ecs, ctx);
            self.ecs.maintain();
            run_effects_queue(&mut self.ecs);
            self.ecs.maintain();
            let mut tick = self.ecs.write_resource::<Tick>();
            tick.0 += 1;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        let ecs = &mut self.ecs;
        if let Some(new_mode) = self.run_mode.tick(ecs, ctx) {
            self.run_mode = new_mode;
        }
        self.run_systems(ctx);
    }
}
