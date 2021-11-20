use specs::prelude::*;

use crate::ecs::components::*;
use crate::ecs::resources::composite_viewshed::CompositeViewshed as CompositeViewshedResource;
use crate::map::viewshed::Viewshed;
use crate::map::Map;

pub struct CompositeViewshed {}

impl<'a> System<'a> for CompositeViewshed {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteExpect<'a, CompositeViewshedResource>,
        ReadStorage<'a, HasViewshed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, mut composite_viewshed_resource, has_viewshed_storage) = data;
        let viewsheds: Vec<&Viewshed> = (&has_viewshed_storage)
            .join()
            .into_iter()
            .map(|has_viewshed| &has_viewshed.viewshed)
            .collect();
        composite_viewshed_resource.composite_viewsheds(viewsheds);
        let positions = (&composite_viewshed_resource.visible_positions)
            .into_iter()
            .collect();
        map.set_revealed_tiles_from_positions(positions);
    }
}
