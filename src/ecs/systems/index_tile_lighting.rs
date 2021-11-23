use specs::prelude::*;

use crate::ecs::components::*;
use crate::spatial_index::TILE_LIGHTING;
use crate::spatial_index::tile_lighting::set_backgrounds as set_tile_lighting_backgrounds;

pub struct IndexTileLighting {}

impl<'a> System<'a> for IndexTileLighting {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, HasViewshed>,
        ReadStorage<'a, HasPosition>,
        ReadStorage<'a, HasLightSource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        //set_tile_lighting_backgrounds();
        let mut tile_lighting = TILE_LIGHTING.lock().unwrap();
        tile_lighting.clear();
        let (has_viewshed_storage, has_position_storage, has_light_source_storage) = data;
        for (has_viewshed, has_position, has_light_source) in (&has_viewshed_storage, &has_position_storage, &has_light_source_storage).join() {
            let position = &has_position.position;
            let viewshed = &has_viewshed.viewshed;
            let light_source = &has_light_source.light_source;
            for lit_position in viewshed.visible_positions.iter() {
                let current_lighting = tile_lighting.get_at_position(lit_position);
                let altered_lighting = light_source.transform_color_at(current_lighting, position, lit_position);
                tile_lighting.set_at_position(lit_position, altered_lighting);
            }
        }
    }
}
