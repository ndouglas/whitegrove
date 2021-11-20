use specs::prelude::*;

use crate::ecs::components::{HasPosition, WantsToMoveWest};
use crate::map::Map;
use crate::map::tile::TileTrait;

pub struct MoveWest {}

impl<'a> System<'a> for MoveWest {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, WantsToMoveWest>,
        WriteStorage<'a, HasPosition>,
    );

    fn run(&mut self, (map, westy_c, mut pos_c): Self::SystemData) {
        for (_westy, pos) in (&westy_c, &mut pos_c).join() {
            let mut position = &mut pos.position;
            let dest = position.to_west();
            if map.get_tiletype_at_position(dest).is_walkable() {
                if position.x == 0 {
                    position.x = map.width;
                }
                else {
                    position.x -= 1;
                }
            }
        }
    }
}
