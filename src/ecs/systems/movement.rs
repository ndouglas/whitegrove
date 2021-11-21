use specs::prelude::*;

use crate::ecs::components::{HasPosition, HasViewshed, IsPlayer, WantsToMove};
use crate::map::tile::TileTrait;
use crate::map::Map;
use crate::model::CompassDirection;

pub struct Movement {}

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, HasPosition>,
        WriteStorage<'a, HasViewshed>,
        ReadStorage<'a, IsPlayer>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            entities,
            mut wants_to_move_storage,
            mut has_position_storage,
            mut has_viewshed_storage,
            is_player_storage,
        ) = data;
        let map_width = map.width;
        let map_height = map.height;
        let mut satisfied = vec![];
        let mut set_move_compass_direction = vec![];
        let mut set_move_randomly = vec![];
        for (entity, wants_to_move, has_position) in (
            &entities,
            &mut wants_to_move_storage,
            &mut has_position_storage,
        )
            .join()
        {
            let mut position = &mut has_position.position;
            match wants_to_move {
                WantsToMove::CompassDirection { compass_direction } => {
                    if let Ok(dest) = position.get_safe_to_compass_direction(
                        (map_width, map_height),
                        *compass_direction,
                    ) {
                        if map.get_tiletype_at_position(dest).is_walkable() {
                            position.x = dest.x;
                            position.y = dest.y;
                            let mut has_viewshed_option: Option<&mut HasViewshed> =
                                has_viewshed_storage.get_mut(entity);
                            if let Some(has_viewshed) = &mut has_viewshed_option {
                                has_viewshed.viewshed.is_dirty = true;
                            }
                        } else {
                            set_move_randomly.push(entity);
                        }
                    }
                },
                WantsToMove::Randomly { ref mut duration } => {
                    if *duration == 0 as usize {
                        set_move_compass_direction.push(entity);
                    }
                    else {
                        *duration -= 1;
                        if let Ok(dest) = position.get_safe_to_compass_direction(
                            (map_width, map_height),
                            CompassDirection::get_random(),
                        ) {
                            if map.get_tiletype_at_position(dest).is_walkable() {
                                position.x = dest.x;
                                position.y = dest.y;
                                let mut has_viewshed_option: Option<&mut HasViewshed> =
                                    has_viewshed_storage.get_mut(entity);
                                if let Some(has_viewshed) = &mut has_viewshed_option {
                                    has_viewshed.viewshed.is_dirty = true;
                                }
                            } else {
                                set_move_randomly.push(entity);
                            }
                        }
                    }
                },
            }
            let is_player_option: Option<&IsPlayer> = is_player_storage.get(entity);
            if let Some(_is_player) = is_player_option {
                satisfied.push(entity);
            }

        }
        for entity in satisfied.iter() {
            wants_to_move_storage.remove(*entity);
        }
        for entity in set_move_randomly.iter() {
            wants_to_move_storage
                .insert(
                    *entity,
                    WantsToMove::Randomly { duration: 5 },
                )
                .expect("Unable to insert movement.");
        }
        for entity in set_move_compass_direction.iter() {
            wants_to_move_storage
                .insert(
                    *entity,
                    WantsToMove::CompassDirection { compass_direction:CompassDirection::get_random() },
                )
                .expect("Unable to insert movement.");
        }
    }
}
