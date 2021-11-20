use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::ecs::components::*;
use crate::map::*;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let map_width;
    let map_height;
    {
        let map = ecs.fetch::<Map>();
        map_width = map.width;
        map_height = map.height;
    }
    let mut has_position_storage = ecs.write_storage::<HasPosition>();
    let mut is_player_storage = ecs.write_storage::<IsPlayer>();

    for (_player, pos) in (&mut is_player_storage, &mut has_position_storage).join() {
        pos.position.x = min(
            map_width - 1,
            max(0, pos.position.x as i32 + delta_x).try_into().unwrap(),
        );
        pos.position.y = min(
            map_height - 1,
            max(0, pos.position.y as i32 + delta_y).try_into().unwrap(),
        );
    }
}

pub fn player_input(ecs: &mut World, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => {
                let player_entity = ecs.fetch::<Entity>();
                ecs.write_storage::<WantsToMoveWest>()
                    .insert(*player_entity, WantsToMoveWest {});
            } //try_move_player(-1, 0, ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, ecs),
            _ => {}
        },
    }
}
