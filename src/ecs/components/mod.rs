use specs::prelude::*;
// use specs::saveload::SimpleMarker;

pub mod has_position;
pub use has_position::*;
pub mod has_renderable;
pub use has_renderable::*;
pub mod wants_to_move_west;
pub use wants_to_move_west::*;

pub fn register_components(ecs: &mut World) {
    ecs.register::<HasPosition>();
    ecs.register::<HasRenderable>();
    ecs.register::<WantsToMoveWest>();
}
