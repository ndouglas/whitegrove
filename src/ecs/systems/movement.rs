use specs::prelude::*;

use crate::ecs::components::{HasPosition, WantsToMove};
use crate::map::tile::TileTrait;
use crate::map::Map;

pub struct Movement {}

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, HasPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, entities, mut wants_to_move_storage, mut has_position_storage) = data;
        let map_width = map.width;
        let map_height = map.height;
        let mut satisfied = vec![];
        for (entity, wants_to_move, has_position) in (&entities, &mut wants_to_move_storage, &mut has_position_storage).join() {
            let mut position = &mut has_position.position;
            if let Ok(dest) = position.get_safe_to_compass_direction((map_width, map_height), wants_to_move.compass_direction) {
                if map.get_tiletype_at_position(dest).is_walkable() {
                    position.x = dest.x;
                    position.y = dest.y;
                }
            }
            satisfied.push(entity);
        }
        for entity in (&entities).join() {
            wants_to_move_storage.remove(entity);
        }
    }
}
