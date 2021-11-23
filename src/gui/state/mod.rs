use rltk::{GameState, Rltk};
use specs::prelude::*;

pub mod run_mode;
pub use run_mode::*;

use crate::ecs::components::*;
use crate::ecs::dispatcher::{get_new_dispatcher, UnifiedDispatcher};
use crate::effects::run_effects_queue;

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

    pub fn collect_garbage(&mut self) {
        trace!("Collecting garbage.");
        let mut dead: Vec<Entity> = Vec::new();
        {
            let has_hit_points_storage = self.ecs.read_storage::<HasHitPoints>();
            let has_name_storage = self.ecs.read_storage::<HasName>();
            let entities = self.ecs.entities();
            for (entity, has_hit_points) in (&entities, &has_hit_points_storage).join() {
                debug!("Searching for dead entities...");
                if has_hit_points.hit_points.current < 1 {
                    let has_name = has_name_storage.get(entity).unwrap();
                    debug!("{} has died!", has_name.name);
                    dead.push(entity);
                }
            }
        }
        for entity in dead {
            debug!("Deleting entity {:?}.", entity);
            self.ecs.delete_entity(entity).expect("Unable to delete");
        }
    }

    pub fn run_systems(&mut self) {
        if self.run_mode.should_maintain_ecs() {
            self.dispatcher.run_now(&mut self.ecs);
            self.ecs.maintain();
            self.collect_garbage();
            self.ecs.maintain();
            run_effects_queue(&mut self.ecs);
            self.ecs.maintain();
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
        self.run_systems();
    }
}
