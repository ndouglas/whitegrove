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

pub fn get_entity_position(ecs: &World, target: Entity) -> Option<usize> {
    if let Some(has_position) = ecs.read_storage::<HasPosition>().get(target) {
        let position = &has_position.position;
        let map = ecs.fetch::<Map>();
        return Some(map.get_xy_as_idx((position.x, position.y)));
    }
    None
}
