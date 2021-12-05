use specs::prelude::*;

use crate::effects::{Effect, Spawner};

pub fn kill_moss(ecs: &mut World, spawner: &Spawner, target: Entity) {
    if let Effect::KillMoss = spawner.effect {
        ecs.delete_entity(target).expect("Unable to kill moss");
    }
}
