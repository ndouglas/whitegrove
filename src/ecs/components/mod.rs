use specs::prelude::*;
// use specs::saveload::SimpleMarker;

pub mod has_position;
pub use has_position::*;
pub mod has_renderable;
pub use has_renderable::*;
pub mod has_viewshed;
pub use has_viewshed::*;
pub mod is_player;
pub use is_player::*;
pub mod wants_to_move;
pub use wants_to_move::*;

pub fn register_components(ecs: &mut World) {
    ecs.register::<HasPosition>();
    ecs.register::<HasRenderable>();
    ecs.register::<HasViewshed>();
    ecs.register::<IsPlayer>();
    ecs.register::<WantsToMove>();
}
