use specs::prelude::*;

pub mod composite_viewshed;
use composite_viewshed::*;
pub mod movement;
use movement::*;
pub mod visibility;
use visibility::*;

pub fn run_systems(ecs: &mut World) {
    let mut movement = Movement {};
    movement.run_now(ecs);
    let mut visibility = Visibility {};
    visibility.run_now(ecs);
    let mut composite_viewshed = CompositeViewshed {};
    composite_viewshed.run_now(ecs);
}
