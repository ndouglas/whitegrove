use rltk::Rltk;
use specs::prelude::*;

use crate::ecs::components::*;

pub fn get_dead_entities(ecs: &World) -> Vec<Entity> {
    let has_hit_points_storage = ecs.read_storage::<HasHitPoints>();
    let entities = ecs.entities();
    (&entities, &has_hit_points_storage)
        .join()
        .filter(|(_entity, has_hit_points)| has_hit_points.hit_points.current < 1)
        .map(|(entity, _has_hit_points)| entity)
        .collect()
}

pub fn collect_garbage(ecs: &mut World, _ctx: &Rltk) {
    for entity in get_dead_entities(ecs) {
        debug!("Deleting entity {:?}.", entity);
        ecs.delete_entity(entity).expect("Unable to delete");
    }
}
