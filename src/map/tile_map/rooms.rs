use std::cmp::min;

use crate::map::TileType;
use crate::model::{get_dim_distance, xy_to_idx, Rectangle};
use crate::random;

pub fn get_filled_tile_map(width: usize, height: usize, tile: TileType) -> Vec<TileType> {
    let length = width * height;
    vec![tile; length]
}

pub fn get_rooms_and_corridors_tile_map(width: usize, height: usize) -> (Vec<TileType>, Vec<Rectangle>) {
    let mut tile_map = get_filled_tile_map(width, height, TileType::Wall);
    let mut rooms: Vec<Rectangle> = Vec::new();
    const MAX_ROOMS: i32 = 20;
    const MIN_SIZE: usize = 20;
    const MAX_SIZE: usize = 25;
    for _ in 0..MAX_ROOMS {
        let w = random::range(MIN_SIZE, MAX_SIZE);
        let h = random::range(MIN_SIZE, MAX_SIZE);
        let x = (random::roll_dice(1, (width - w - 1) as i32) - 1) as usize;
        let y = (random::roll_dice(1, (height - h - 1) as i32) - 1) as usize;
        let new_room = Rectangle::with_size((x, y), (w, h));
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersects_rectangle(other_room) {
                ok = false;
                break;
            }
        }
        if ok {
            fill_rectangle_of_tile_map(&new_room, &mut tile_map, width, TileType::Floor);

            if !rooms.is_empty() {
                let r1c = new_room.get_center_xy();
                let r2c = rooms[rooms.len() - 1].get_center_xy();
                create_corridor(&mut tile_map, width, r1c, r2c);
            }

            rooms.push(new_room);
        }
    }
    (tile_map, rooms)
}

pub fn fill_rectangle_of_tile_map(
    rect: &Rectangle,
    tiles: &mut [TileType],
    width: usize,
    tile: TileType,
) {
    for y in rect.y1 + 1..=rect.y2 {
        for x in rect.x1 + 1..=rect.x2 {
            tiles[xy_to_idx(width, x.try_into().unwrap(), y.try_into().unwrap())] = tile;
        }
    }
}

pub fn create_corridor(
    tiles: &mut [TileType],
    width: usize,
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
) {
    fill_vertical_line_of_tile_map(tiles, x1, y1, y2, width, TileType::Floor);
    fill_horizontal_line_of_tile_map(tiles, x1, x2, y2, width, TileType::Floor);
}

pub fn fill_horizontal_line_of_tile_map(
    tiles: &mut [TileType],
    x1: usize,
    x2: usize,
    y: usize,
    width: usize,
    tile: TileType,
) {
    let length = get_dim_distance(x1, x2);
    let rectangle = Rectangle::from_horizontal_line((min(x1, x2), y), length + 1, 1);
    fill_rectangle_of_tile_map(&rectangle, tiles, width, tile);
}

pub fn fill_vertical_line_of_tile_map(
    tiles: &mut [TileType],
    x: usize,
    y1: usize,
    y2: usize,
    width: usize,
    tile: TileType,
) {
    let length = get_dim_distance(y1, y2);
    let rectangle = Rectangle::from_vertical_line((x, min(y1, y2)), length, 1);
    fill_rectangle_of_tile_map(&rectangle, tiles, width, tile);
}
