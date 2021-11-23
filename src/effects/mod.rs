pub mod effect;
pub use effect::*;
pub mod queue;
pub use queue::*;
pub mod spawner;
pub use spawner::*;
pub mod target;
pub use target::*;

use specs::prelude::*;

use crate::ecs::components::*;
use crate::map::Map;
use crate::model::Position;

pub fn get_entity_position(ecs: &World, target: Entity) -> Option<Position> {
    if let Some(has_position) = ecs.read_storage::<HasPosition>().get(target) {
        return Some(has_position.position.clone());
    }
    None
}

pub fn get_entity_xy(ecs: &World, target: Entity) -> Option<(usize, usize)> {
    if let Some(position) = get_entity_position(ecs, target) {
        return Some((position.x, position.y));
    }
    None
}

pub fn get_entity_idx(ecs: &World, target: Entity) -> Option<usize> {
    if let Some(xy) = get_entity_xy(ecs, target) {
        let map = ecs.fetch::<Map>();
        return Some(map.get_xy_as_idx(xy));
    }
    None
}
