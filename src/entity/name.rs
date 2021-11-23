use specs::prelude::*;

use crate::ecs::components::*;

pub fn get_entity_name(ecs: &World, target: Entity) -> Option<String> {
    if let Some(has_name) = ecs.read_storage::<HasName>().get(target) {
        return Some(has_name.name.clone());
    }
    None
}
