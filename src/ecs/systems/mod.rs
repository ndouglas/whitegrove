use specs::prelude::*;

pub mod movement;
use movement::*;
pub mod visibility;
use visibility::*;

pub fn run_systems(ecs: &mut World) {
    let mut movement = Movement {};
    movement.run_now(ecs);
    let mut visibility = Visibility {};
    visibility.run_now(ecs);
}
