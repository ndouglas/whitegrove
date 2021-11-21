use specs::prelude::*;

use crate::ecs::components::*;
use crate::map::Map;

pub struct TileEntityIndex {}

impl<'a> System<'a> for TileEntityIndex {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, HasPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, has_position_storage) = data;
        map.tile_entities.clear();
        for (entity, has_position) in (&entities, &has_position_storage).join() {
            map.tile_entities.add_at_position(has_position.position, entity);
        }
    }
}
