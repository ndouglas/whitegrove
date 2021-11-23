use specs::prelude::*;

use crate::ecs::components::*;
use crate::model::Position;

pub fn get_entity_position(ecs: &World, target: Entity) -> Option<Position> {
    if let Some(has_position) = ecs.read_storage::<HasPosition>().get(target) {
        return Some(has_position.position);
    }
    None
}
