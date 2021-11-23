use rltk::Rltk;
use specs::prelude::*;

use crate::ecs::components::*;

pub fn collect_garbage(ecs: &mut World, _ctx: &Rltk) {
    let mut dead: Vec<Entity> = Vec::new();
    {
        let has_hit_points_storage = ecs.read_storage::<HasHitPoints>();
        let has_name_storage = ecs.read_storage::<HasName>();
        let entities = ecs.entities();
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
        ecs.delete_entity(entity).expect("Unable to delete");
    }
}

