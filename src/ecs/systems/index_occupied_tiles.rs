use specs::prelude::*;

use crate::ecs::components::*;
use crate::spatial_index::OCCUPIED_TILES;

pub struct IndexOccupiedTiles {}

impl<'a> System<'a> for IndexOccupiedTiles {
    type SystemData = (ReadStorage<'a, HasPosition>, ReadStorage<'a, OccupiesTile>);

    fn run(&mut self, data: Self::SystemData) {
        let (has_position_storage, occupies_tile_storage) = data;
        let positions = (&has_position_storage, &occupies_tile_storage)
            .join()
            .map(|(has_position, _occupies_tile)| &has_position.position)
            .collect();
        let mut occupied_tiles = OCCUPIED_TILES.lock().unwrap();
        occupied_tiles.clear();
        occupied_tiles.set_at_positions(&positions, true);
    }
}
