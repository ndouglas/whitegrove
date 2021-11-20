use specs::prelude::*;

use crate::ecs::components::{HasPosition, WantsToMoveWest};
use crate::map::tile::TileTrait;
use crate::map::Map;

pub struct MoveWest {}

impl<'a> System<'a> for MoveWest {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, WantsToMoveWest>,
        WriteStorage<'a, HasPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, entities, mut westy_c, mut pos_c) = data;
        let mut satisfied = vec![];
        for (entity, _westy, pos) in (&entities, &mut westy_c, &mut pos_c).join() {
            let mut position = &mut pos.position;
            if let Ok(dest) = position.to_west() {
                if map.get_tiletype_at_position(dest).is_walkable() {
                    if position.x == 0 {
                        position.x = map.width;
                    } else {
                        position.x -= 1;
                    }
                }
            }
            satisfied.push(entity);
        }
        for (entity) in (&entities).join() {
            westy_c.remove(entity);
        }
    }
}
