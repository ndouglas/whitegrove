use specs::prelude::*;

use crate::ecs::components::*;
use crate::lifecycle::Moss;
use crate::spatial_index::MOSS_TILES;

pub struct IndexMossTiles {}

impl<'a> System<'a> for IndexMossTiles {
    type SystemData = (
        ReadStorage<'a, HasMossLifecycle>,
        ReadStorage<'a, HasPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (has_moss_lifecycle_storage, has_position_storage) = data;
        let moss_positions = (&has_moss_lifecycle_storage, &has_position_storage)
            .join()
            .filter(|(has_moss_lifecycle, _has_position)| {
                has_moss_lifecycle.moss_lifecycle == Moss::Moss
            })
            .map(|(_has_moss_lifecycle, has_position)| &has_position.position)
            .collect();
        let mut moss_tiles = MOSS_TILES.lock().unwrap();
        moss_tiles.clear();
        moss_tiles.set_at_positions(&moss_positions, true);
    }
}
