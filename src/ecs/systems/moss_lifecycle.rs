use specs::prelude::*;

use crate::ecs::components::*;
use crate::ecs::resources::*;

pub struct MossLifecycle {}

impl<'a> System<'a> for MossLifecycle {
    type SystemData = (
        ReadExpect<'a, Tick>,
        Entities<'a>,
        ReadStorage<'a, HasMossLifecycle>,
        ReadStorage<'a, HasPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (tick_resource, entities, has_moss_lifecycle_storage, has_position_storage) = data;
        if tick_resource.0 % 600 != 0 {
            return;
        }
        for (entity, has_moss_lifecycle, has_position) in (
            &entities,
            &has_moss_lifecycle_storage,
            &has_position_storage,
        )
            .join()
        {
            has_moss_lifecycle
                .moss_lifecycle
                .tick(entity, &has_position.position);
        }
    }
}
