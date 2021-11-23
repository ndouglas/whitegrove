use specs::prelude::*;

use crate::ecs::components::*;
use crate::spatial_index::TILE_OCCUPANTS;

pub struct IndexTileOccupants {}

impl<'a> System<'a> for IndexTileOccupants {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, HasPosition>,
        ReadStorage<'a, OccupiesTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, has_position_storage, occupies_tile_storage) = data;
        let positions_vector = (&entities, &has_position_storage, &occupies_tile_storage)
            .join()
            .into_iter()
            .map(|(entity, has_position, _occupies_tile)| (entity, &has_position.position))
            .collect();
        let mut tile_occupants = TILE_OCCUPANTS.lock().unwrap();
        tile_occupants.clear();
        tile_occupants.set_at_positions(&positions_vector);
    }
}
