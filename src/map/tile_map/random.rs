use crate::map::TileType;
use crate::model::xy_to_idx;
use crate::random;

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
    for _i in 0..count {
        let x = random::roll_dice(1, (width - 1).try_into().unwrap()) as usize;
        let y = random::roll_dice(1, (height - 1).try_into().unwrap()) as usize;
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
