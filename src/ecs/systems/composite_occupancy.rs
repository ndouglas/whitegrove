use specs::prelude::*;

use crate::ecs::components::*;
use crate::map::Map;

pub struct CompositeOccupancy {}

impl<'a> System<'a> for CompositeOccupancy {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, HasPosition>,
        ReadStorage<'a, OccupiesTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, has_position_storage, occupies_tile_storage) = data;
        let positions = (&has_position_storage, &occupies_tile_storage)
            .join()
            .into_iter()
            .map(|(has_position, _occupies_tile)| &has_position.position)
            .collect();
        map.set_occupied_tiles_from_positions(positions);
    }
}
