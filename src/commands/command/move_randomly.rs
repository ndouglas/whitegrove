use specs::prelude::*;

use crate::ecs::components::*;

pub fn move_randomly(ecs: &mut World, entity: Entity, duration: usize) {
    if !ecs.entities().is_alive(entity) {
        error!("Entity {:?} cannot move because it is no longer alive.", entity);
        return;
    }
    ecs.write_storage::<WantsToMove>()
        .insert(
            entity,
            WantsToMove::Randomly {
                duration,
            },
        )
        .expect(format!("Could not move {:?} randomly.", entity).as_str());
}
