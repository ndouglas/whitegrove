use specs::prelude::*;

use crate::ecs::components::*;
use crate::model::CompassDirection;

pub struct Ai {}

impl<'a> System<'a> for Ai {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, HasViewshed>,
        ReadStorage<'a, HasPosition>,
        ReadStorage<'a, HasAi>,
        WriteStorage<'a, WantsToMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _has_viewshed_storage,
            has_position_storage,
            has_ai_storage,
            mut wants_to_move_storage,
        ) = data;
        let non_moving_entities: Vec<Entity> = (
            &entities,
            &has_position_storage,
            &has_ai_storage,
            !&wants_to_move_storage,
        )
            .join()
            .map(|(entity, _has_position, _has_ai, ())| entity)
            .collect();
        for entity in non_moving_entities.iter() {
            wants_to_move_storage
                .insert(
                    *entity,
                    WantsToMove::CompassDirection {
                        compass_direction: CompassDirection::get_random(),
                    },
                )
                .expect("Unable to insert movement.");
        }
    }
}
