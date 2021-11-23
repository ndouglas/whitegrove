use specs::prelude::*;

use crate::ecs::components::*;
use crate::spatial_index::TILE_ENTITIES;

pub struct IndexTileEntities {}

impl<'a> System<'a> for IndexTileEntities {
    type SystemData = (Entities<'a>, ReadStorage<'a, HasPosition>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, has_position_storage) = data;
        let positions_vector = (&entities, &has_position_storage)
            .join()
            .into_iter()
            .map(|(entity, has_position)| (entity, &has_position.position))
            .collect();
        let mut tile_entities = TILE_ENTITIES.lock().unwrap();
        tile_entities.clear();
        tile_entities.add_at_positions(&positions_vector);
    }
}
