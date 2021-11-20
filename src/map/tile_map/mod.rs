
use crate::model::xy_to_idx;

use super::TileType;

pub fn get_random_tile_map(width: usize, height: usize) -> Vec<TileType> {
    let length = width * height;
    let mut result = vec![TileType::Floor; length];
    for x in 0..width {
        result[xy_to_idx(width, x, 0)] = TileType::Wall;
        result[xy_to_idx(width, x, height - 1)] = TileType::Wall;
    }
    for y in 0..height {
        result[xy_to_idx(width, 0, y)] = TileType::Wall;
        result[xy_to_idx(width, width - 1, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..length / 10 {
        let x = rng.roll_dice(1, (width - 1).try_into().unwrap()) as usize;
        let y = rng.roll_dice(1, (height - 1).try_into().unwrap()) as usize;
        let idx = xy_to_idx(width, x, y);
        if idx != xy_to_idx(width, width / 2, height / 2) {
            result[idx] = TileType::Wall;
        }
    }
    result
}
