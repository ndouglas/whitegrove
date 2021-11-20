use specs::prelude::*;

use crate::ecs::components::*;
use crate::model::CompassDirection;

pub fn move_compass_direction(ecs: &mut World, entity: Entity, dir: CompassDirection) {
    ecs.write_storage::<WantsToMove>()
        .insert(entity, WantsToMove {
            compass_direction: dir,
        });
}
