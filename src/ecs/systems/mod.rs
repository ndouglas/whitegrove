use specs::prelude::*;

pub mod move_west;
use move_west::*;

pub fn run_systems(ecs: &mut World) {
    let mut move_west = MoveWest{};
    move_west.run_now(ecs);
}
