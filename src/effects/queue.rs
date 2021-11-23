use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

use super::Effect;
use super::Spawner;
use super::Target;

lazy_static! {
    pub static ref EFFECT_QUEUE: Mutex<VecDeque<Spawner>> = Mutex::new(VecDeque::new());
}

pub fn enqueue_effect(creator: Option<Entity>, effect: Effect, target: Target) {
    EFFECT_QUEUE.lock().unwrap().push_back(Spawner {
        creator,
        effect,
        target,
    });
}

pub fn run_effects_queue(ecs: &mut World) {
    loop {
        let spawner: Option<Spawner> = EFFECT_QUEUE.lock().unwrap().pop_front();
        if let Some(spawner) = spawner {
            spawner.target.apply(ecs, &spawner);
        } else {
            break;
        }
    }
}
