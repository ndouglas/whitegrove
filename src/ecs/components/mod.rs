use specs::prelude::*;
// use specs::saveload::SimpleMarker;

pub mod has_position;
pub use has_position::*;
pub mod has_renderable;
pub use has_renderable::*;

pub fn register_components(ecs: &mut World) {
    ecs.register::<HasPosition>();
    ecs.register::<HasRenderable>();
}
