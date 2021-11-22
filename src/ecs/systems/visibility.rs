use specs::prelude::*;

use crate::ecs::components::*;
use crate::map::Map;

pub struct Visibility {}

impl<'a> System<'a> for Visibility {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, HasViewshed>,
        WriteStorage<'a, HasPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mut has_viewshed_storage, has_position_storage) = data;
        for (has_viewshed, has_position) in (&mut has_viewshed_storage, &has_position_storage)
            .join()
            .filter(|(has_viewshed, _has_position)| has_viewshed.viewshed.is_dirty)
        {
            (&mut has_viewshed.viewshed).recalculate(&has_position.position, &*map);
        }
    }
}
