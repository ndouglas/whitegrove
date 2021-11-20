use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::commands::*;
use crate::model::CompassDirection;

pub fn player_input(ecs: &mut World, ctx: &mut Rltk) {
    let player_entity;
    {
       player_entity = *ecs.fetch::<Entity>();
    }
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => move_compass_direction(ecs, player_entity, CompassDirection::West),
            VirtualKeyCode::Right => move_compass_direction(ecs, player_entity, CompassDirection::East),
            VirtualKeyCode::Up => move_compass_direction(ecs, player_entity, CompassDirection::North),
            VirtualKeyCode::Down => move_compass_direction(ecs, player_entity, CompassDirection::South),
            _ => {}
        },
    }
}
