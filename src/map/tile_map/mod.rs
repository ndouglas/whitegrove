use rltk::RandomNumberGenerator;
use std::cmp::{max, min};

use crate::model::{get_dim_distance, xy_to_idx, Rectangle};

use super::TileType;

pub fn add_borders_to_tile_map(tiles: &mut Vec<TileType>, width: usize, height: usize) {
    for x in 0..width {
        tiles[xy_to_idx(width, x, 0)] = TileType::Wall;
        tiles[xy_to_idx(width, x, height - 1)] = TileType::Wall;
    }
    for y in 0..height {
        tiles[xy_to_idx(width, 0, y)] = TileType::Wall;
        tiles[xy_to_idx(width, width - 1, y)] = TileType::Wall;
    }
}

pub fn add_random_walls_to_tile_map(
    tiles: &mut Vec<TileType>,
    width: usize,
    height: usize,
    count: usize,
) {
    let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 0..count {
        let x = rng.roll_dice(1, (width - 1).try_into().unwrap()) as usize;
        let y = rng.roll_dice(1, (height - 1).try_into().unwrap()) as usize;
        let idx = xy_to_idx(width, x, y);
        if idx != xy_to_idx(width, width / 2, height / 2) {
            tiles[idx] = TileType::Wall;
        }
    }
}

pub fn get_random_tile_map(width: usize, height: usize) -> Vec<TileType> {
    let length = width * height;
    let mut result = vec![TileType::Floor; length];
    add_borders_to_tile_map(&mut result, width, height);
    add_random_walls_to_tile_map(&mut result, width, height, length / 10);
    result
}

pub fn get_filled_tile_map(width: usize, height: usize, tile: TileType) -> Vec<TileType> {
    let length = width * height;
    vec![tile; length]
}

pub fn get_rooms_and_corridors_tile_map(width: usize, height: usize) -> Vec<TileType> {
    let mut result = get_filled_tile_map(width, height, TileType::Wall);
    let mut rooms: Vec<Rectangle> = Vec::new();
    let mut rng = RandomNumberGenerator::new();
    const MAX_ROOMS: i32 = 20;
    const MIN_SIZE: usize = 20;
    const MAX_SIZE: usize = 25;
    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = (rng.roll_dice(1, (width - w - 1) as i32) - 1) as usize;
        let y = (rng.roll_dice(1, (height - h - 1) as i32) - 1) as usize;
        let new_room = Rectangle::with_size((x, y), (w, h));
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersects_rectangle(other_room) {
                ok = false;
                break;
            }
        }
        if ok {
            fill_rectangle_of_tile_map(&new_room, &mut result, width, TileType::Floor);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.get_center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].get_center();
                if rng.range(0, 2) == 1 {
                    fill_horizontal_line_of_tile_map(
                        &mut result,
                        prev_x,
                        new_x,
                        prev_y,
                        width,
                        TileType::Floor,
                    );
                    fill_vertical_line_of_tile_map(
                        &mut result,
                        new_x,
                        prev_y,
                        new_y,
                        width,
                        TileType::Floor,
                    );
                } else {
                    fill_vertical_line_of_tile_map(
                        &mut result,
                        prev_x,
                        prev_y,
                        new_y,
                        width,
                        TileType::Floor,
                    );
                    fill_horizontal_line_of_tile_map(
                        &mut result,
                        prev_x,
                        new_x,
                        new_y,
                        width,
                        TileType::Floor,
                    );
                }
            }

            rooms.push(new_room);
        }
    }
    result
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

pub fn fill_horizontal_line_of_tile_map(
    tiles: &mut [TileType],
    x1: usize,
    x2: usize,
    y: usize,
    width: usize,
    tile: TileType,
) {
    let length = get_dim_distance(x1, x2);
    let rectangle = Rectangle::from_horizontal_line((min(x1, x2), y), length, 1);
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
