use specs::prelude::*;

use crate::ecs::components::*;
use crate::map::Map;
use crate::model::CompassDirection;

pub struct Movement {}

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, HasName>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, HasPosition>,
        WriteStorage<'a, HasViewshed>,
        ReadStorage<'a, IsPlayer>,
        WriteStorage<'a, HasMeleeTarget>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            entities,
            has_name_storage,
            mut wants_to_move_storage,
            mut has_position_storage,
            mut has_viewshed_storage,
            is_player_storage,
            mut has_melee_target_storage,
        ) = data;
        let map_width = map.width;
        let map_height = map.height;
        let mut satisfied = vec![];
        let mut set_move_compass_direction = vec![];
        let mut set_move_randomly = vec![];
        let mut set_move_toward_target = vec![];
        let mut actually_move = vec![];
        for (entity, has_name, wants_to_move, has_position) in (
            &entities,
            &has_name_storage,
            &mut wants_to_move_storage,
            &has_position_storage,
        )
            .join()
        {
            let position = &has_position.position;
            match wants_to_move {
                WantsToMove::CompassDirection { compass_direction } => {
                    if let Ok(destination) = position
                        .get_safe_to_compass_direction((map_width, map_height), *compass_direction)
                    {
                        if let Some(occupant) = map.tile_occupants.get_at_position(&destination) {
                            let occupant_has_name_option = &has_name_storage.get(occupant);
                            match occupant_has_name_option {
                                None => {
                                  debug!("{} ({}) wants to move {} (to {}) but it is occupied by a dying entity ({:?}).", has_name.name, position, compass_direction, destination, occupant);
                                },
                                Some(occupant_has_name) => {
                                  debug!("{} ({}) wants to move {} (to {}) but it is occupied by {}.", has_name.name, position, compass_direction, destination, occupant_has_name.name);
                                  has_melee_target_storage
                                      .insert(
                                          entity,
                                          HasMeleeTarget {
                                              melee_target: occupant,
                                          },
                                      )
                                      .expect("Unable to insert has-melee-target.");
                                  set_move_toward_target.push((entity, occupant));
                                },
                            }
                        } else if map.is_exit_valid_xy((destination.x, destination.y)) {
                            debug!("{} wants to move {} and it is possible.", has_name.name, compass_direction);
                            actually_move.push((entity, destination));
                        } else {
                            debug!("{} wants to move {} but it is not a valid exit.", has_name.name, compass_direction);
                            let is_player_option: Option<&IsPlayer> = is_player_storage.get(entity);
                            if let None = is_player_option {
                                trace!("{} wants to move randomly.", has_name.name);
                                set_move_randomly.push(entity);
                            }
                        }
                    } else {
                        debug!("{} wants to move {} but it is not okay.", has_name.name, compass_direction);
                    }
                    let is_player_option: Option<&IsPlayer> = is_player_storage.get(entity);
                    if let Some(_is_player) = is_player_option {
                        satisfied.push(entity);
                    }
                }
                WantsToMove::Randomly { ref mut duration } => {
                    if *duration == 0 as usize {
                        debug!("{} wants to stop moving randomly.", has_name.name);
                        set_move_compass_direction.push(entity);
                    } else {
                        *duration -= 1;
                        let random_direction = CompassDirection::get_random();
                        if let Ok(destination) = position.get_safe_to_compass_direction(
                            (map_width, map_height),
                            random_direction,
                        ) {
                            if map.is_exit_valid_xy((destination.x, destination.y)) {
                                debug!("{} wants to randomly move {} and it is possible.", has_name.name, random_direction);
                                actually_move.push((entity, destination));
                            } else {
                                debug!("{} wants to move randomly now.", has_name.name);
                                set_move_randomly.push(entity);
                            }
                        } else {
                            debug!("{} wants to randomly move {} but it is not okay.", has_name.name, random_direction);
                        }
                    }
                },
                WantsToMove::TowardTarget { target } => {
                    if !entities.is_alive(*target) {
                        debug!("{} has a target, but it is dead.", has_name.name);
                        set_move_randomly.push(entity);
                    } else {
                        let target_has_name_option = &has_name_storage.get(*target);
                        match target_has_name_option {
                            None => {
                                debug!("{} is hunting a dying target ({:?}).", has_name.name, target);
                            },
                            Some(target_has_name) => {
                                debug!("{} is hunting a living target ({}).", has_name.name, target_has_name.name);
                                let has_viewshed = &has_viewshed_storage.get(entity).unwrap();
                                let has_target_position_option = has_position_storage.get(*target);
                                if let Some(has_target_position) = has_target_position_option {
                                    if has_viewshed.viewshed.contains_position(&has_target_position.position) {
                                        if let Some(next_move_position) = map.get_next_astar_step_position(&position, &has_target_position.position) {
                                            debug!("{} is approaching target at {}!", has_name.name, next_move_position);
                                            actually_move.push((entity, next_move_position));
                                        } else {
                                            debug!("{} has a target, but can't see it.", has_name.name);
                                            set_move_randomly.push(entity);
                                        }
                                    }
                                }
                            },
                        }

                    }
                },
            }
        }
        for (entity, destination) in actually_move.iter() {
            let mut has_position = &mut has_position_storage.get_mut(*entity).unwrap();
            has_position.position.x = destination.x;
            has_position.position.y = destination.y;
            let mut has_viewshed_option: Option<&mut HasViewshed> =
                has_viewshed_storage.get_mut(*entity);
            if let Some(has_viewshed) = &mut has_viewshed_option {
                has_viewshed.viewshed.is_dirty = true;
            }
        }
        for entity in satisfied.iter() {
            debug!(
                "{} is satisfied with its movement.",
                has_name_storage.get(*entity).unwrap().name
            );
            wants_to_move_storage.remove(*entity);
        }
        for entity in set_move_randomly.iter() {
            debug!(
                "{} will move randomly.",
                has_name_storage.get(*entity).unwrap().name
            );
            wants_to_move_storage
                .insert(*entity, WantsToMove::Randomly { duration: 2 })
                .expect("Unable to insert movement.");
        }
        for entity in set_move_compass_direction.iter() {
            debug!(
                "{} will move in a random compass direction.",
                has_name_storage.get(*entity).unwrap().name
            );
            wants_to_move_storage
                .insert(
                    *entity,
                    WantsToMove::CompassDirection {
                        compass_direction: CompassDirection::get_random(),
                    },
                )
                .expect("Unable to insert movement.");
        }
        for (entity, target) in set_move_toward_target.iter() {
            debug!(
                "{} will pursue a target {}.",
                has_name_storage.get(*entity).unwrap().name,
                has_name_storage.get(*target).unwrap().name
            );
            wants_to_move_storage
                .insert(
                    *entity,
                    WantsToMove::TowardTarget {
                        target: *target,
                    },
                )
                .expect("Unable to insert movement.");
        }
    }
}
