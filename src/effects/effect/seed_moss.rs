use specs::prelude::*;

use crate::ecs::components::*;
use crate::effects::{Effect, Spawner};
use crate::lifecycle::Moss as MossLifecycle;
use crate::map::Map;
use crate::model::Position;
use crate::spatial_index::MOSS_SEEDED_TILES;

pub fn seed_moss(ecs: &mut World, spawner: &Spawner, position: &Position) {
    if let Effect::SeedMoss = spawner.effect {
        {
            let map = ecs.fetch::<Map>();
            if !map.is_exit_valid_xy((position.x, position.y)) {
                return;
            }
        }
        let mut moss_seeded_tiles = MOSS_SEEDED_TILES.lock().unwrap();
        if moss_seeded_tiles.get_at_position(&position) {
            return;
        }
        error!("Seeding moss at {}", position);
        ecs.create_entity()
            .with(HasPosition {
                position: *position,
            })
            .with(HasMossLifecycle {
                moss_lifecycle: MossLifecycle::Seed,
            })
            .with(HasName {
                name: format!("Moss"),
            })
            .build();
        moss_seeded_tiles.set_at_position(&position, true);
    }
}
