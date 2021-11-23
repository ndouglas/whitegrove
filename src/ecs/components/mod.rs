use specs::prelude::*;
// use specs::saveload::SimpleMarker;

pub mod has_ai;
pub use has_ai::*;
pub mod has_hit_points;
pub use has_hit_points::*;
pub mod has_melee_target;
pub use has_melee_target::*;
pub mod has_melee_weapon;
pub use has_melee_weapon::*;
pub mod has_name;
pub use has_name::*;
pub mod has_position;
pub use has_position::*;
pub mod has_renderable;
pub use has_renderable::*;
pub mod has_viewshed;
pub use has_viewshed::*;
pub mod is_player;
pub use is_player::*;
pub mod occupies_tile;
pub use occupies_tile::*;
pub mod wants_to_move;
pub use wants_to_move::*;

pub fn register_components(ecs: &mut World) {
    ecs.register::<HasAi>();
    ecs.register::<HasHitPoints>();
    ecs.register::<HasMeleeTarget>();
    ecs.register::<HasMeleeWeapon>();
    ecs.register::<HasName>();
    ecs.register::<HasPosition>();
    ecs.register::<HasRenderable>();
    ecs.register::<HasViewshed>();
    ecs.register::<IsPlayer>();
    ecs.register::<OccupiesTile>();
    ecs.register::<WantsToMove>();
}
