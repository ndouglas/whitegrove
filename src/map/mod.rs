use rltk::Rltk;
use serde::*;

use crate::model::{idx_to_xy, xy_to_idx, Position};

pub mod tile;
use tile::*;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub length: usize,
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let length = width * height;
        let mut tiles = vec![TileType::Floor; length];
        for x in 0..width {
            tiles[xy_to_idx(width, x, 0)] = TileType::Wall;
            tiles[xy_to_idx(width, x, height - 1)] = TileType::Wall;
        }
        for y in 0..height {
            tiles[xy_to_idx(width, 0, y)] = TileType::Wall;
            tiles[xy_to_idx(width, width - 1, y)] = TileType::Wall;
        }

        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..length / 10 {
            let x = rng.roll_dice(1, (width - 1).try_into().unwrap()) as usize;
            let y = rng.roll_dice(1, (height - 1).try_into().unwrap()) as usize;
            let idx = xy_to_idx(width, x, y);
            if idx != xy_to_idx(width, width / 2, height / 2) {
                tiles[idx] = TileType::Wall;
            }
        }

        Map {
            width: width,
            height: height,
            length: length,
            tiles: tiles,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            let renderable = tile.get_renderable();
            let (x, y) = self.get_idx_as_xy(idx);
            ctx.set(x, y, renderable.fg, renderable.bg, renderable.glyph);
        }
    }

    pub fn get_tiletype_at_idx(&self, idx: usize) -> TileType {
        self.tiles[idx]
    }

    pub fn get_tiletype_at_xy(&self, x: usize, y: usize) -> TileType {
        self.tiles[xy_to_idx(self.width, x, y)]
    }

    pub fn get_tiletype_at_position(&self, pos: Position) -> TileType {
        self.get_tiletype_at_idx(pos.get_idx(self.width))
    }

    pub fn get_xy_as_idx(&self, x: usize, y: usize) -> usize {
        xy_to_idx(self.width, x, y)
    }

    pub fn get_idx_as_xy(&self, idx: usize) -> (usize, usize) {
        idx_to_xy(self.width, idx)
    }
}
