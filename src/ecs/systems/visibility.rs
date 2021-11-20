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
        for (has_viewshed, has_position) in
            (&mut has_viewshed_storage, &has_position_storage).join()
        {
            let position = &has_position.position;
            let viewshed = &mut has_viewshed.viewshed;
            if viewshed.is_dirty {
                viewshed.recalculate(position, &*map);
            }
        }
    }
}
