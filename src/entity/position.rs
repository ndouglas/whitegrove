use specs::prelude::*;

use crate::ecs::components::*;
use crate::model::Position;
use crate::spatial_index::TILE_ENTITIES;

pub fn get_entity_position(ecs: &World, target: Entity) -> Option<Position> {
    if let Some(has_position) = ecs.read_storage::<HasPosition>().get(target) {
        return Some(has_position.position);
    }
    None
}

pub fn get_entities_at_position(_ecs: &World, position: &Position) -> Vec<Entity> {
    TILE_ENTITIES
        .lock()
        .unwrap()
        .get_at_position(position)
        .to_vec()
}
