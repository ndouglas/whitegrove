use specs::prelude::*;

use crate::ecs::components::{HasPosition, WantsToMoveWest};

pub struct MoveWest {}

impl<'a> System<'a> for MoveWest {
    type SystemData = (
        ReadStorage<'a, WantsToMoveWest>,
        WriteStorage<'a, HasPosition>,
    );

    fn run(&mut self, (westy, mut pos): Self::SystemData) {
        for (_westy, pos) in (&westy, &mut pos).join() {
            pos.position.x -= 1;
            if pos.position.x < 0 {
                pos.position.x = 79;
            }
        }
    }
}
