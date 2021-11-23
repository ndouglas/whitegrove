use specs::prelude::*;

use crate::ecs::components::*;
use crate::model::*;

pub fn move_compass_direction(ecs: &mut World, entity: Entity, dir: CompassDirection) {
    if !ecs.entities().is_alive(entity) {
        error!(
            "Entity {:?} cannot move because it is no longer alive.",
            entity
        );
        return;
    }
    ecs.write_storage::<WantsToMove>()
        .insert(
            entity,
            WantsToMove::CompassDirection {
                compass_direction: dir,
            },
        )
        .expect(format!("Could not move {:?} {}", entity, dir).as_str());
}
