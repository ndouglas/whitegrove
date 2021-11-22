use specs::prelude::*;

use crate::ecs::components::*;
use crate::map::Map;

pub struct IndexTileOccupants {}

impl<'a> System<'a> for IndexTileOccupants {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, HasPosition>,
        ReadStorage<'a, OccupiesTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, has_position_storage, occupies_tile_storage) = data;
        map.tile_occupants.clear();
        for (entity, has_position, _occupies_tile) in
            (&entities, &has_position_storage, &occupies_tile_storage).join()
        {
            map.tile_occupants
                .set_at_position(&has_position.position, entity);
        }
    }
}
