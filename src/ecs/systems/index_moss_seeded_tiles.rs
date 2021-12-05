use specs::prelude::*;

use crate::ecs::components::*;
use crate::spatial_index::MOSS_SEEDED_TILES;

pub struct IndexMossSeededTiles {}

impl<'a> System<'a> for IndexMossSeededTiles {
    type SystemData = (
        ReadStorage<'a, HasMossLifecycle>,
        ReadStorage<'a, HasPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (has_moss_lifecycle_storage, has_position_storage) = data;
        let moss_seeded_positions = (&has_moss_lifecycle_storage, &has_position_storage)
            .join()
            .map(|(_has_moss_lifecycle, has_position)| &has_position.position)
            .collect();
        let mut moss_seeded_tiles = MOSS_SEEDED_TILES.lock().unwrap();
        moss_seeded_tiles.clear();
        moss_seeded_tiles.set_at_positions(&moss_seeded_positions, true);
    }
}
