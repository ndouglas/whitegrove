use rltk::{Algorithm2D, BaseMap, Point, Rltk};
use serde::*;

use crate::model::{idx_to_xy, xy_to_idx, Position, Rectangle};

pub mod tile;
pub use tile::*;
pub mod tile_map;
pub use tile_map::*;
pub mod viewshed;
pub use viewshed::*;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub length: usize,
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rectangle>,
    pub revealed_tiles: Vec<bool>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let length = width * height;
        let (tiles, rooms) = get_rooms_and_corridors_tile_map(width, height);
        Map {
            width: width,
            height: height,
            length: length,
            tiles: tiles,
            rooms: rooms,
            revealed_tiles: vec![false; length],
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            let (x, y) = self.get_idx_as_xy(idx);
            if self.revealed_tiles[idx] {
                let renderable = tile.get_renderable();
                ctx.set(x, y, renderable.fg, renderable.bg, renderable.glyph);
            }
        }
    }

    pub fn set_revealed_tiles_from_positions(&mut self, positions: Vec<&Position>) {
        for position in positions.iter() {
            let idx = self.get_xy_as_idx(position.x, position.y);
            self.revealed_tiles[idx] = true;
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

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].is_opaque()
    }
}
