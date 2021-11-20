use specs::prelude::*;

pub mod movement;
use movement::*;

pub fn run_systems(ecs: &mut World) {
    let mut movement = Movement {};
    movement.run_now(ecs);
}
