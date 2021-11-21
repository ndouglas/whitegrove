use rltk::{Algorithm2D, BaseMap, Point, Rltk};

use crate::model::{idx_to_xy, xy_to_idx, Position, Rectangle};

pub mod tile;
pub use tile::*;
pub mod tile_entities;
pub use tile_entities::*;
pub mod tile_flags;
pub use tile_flags::*;
pub mod tile_map;
pub use tile_map::*;
pub mod viewshed;
pub use viewshed::*;

#[derive(Clone, Default, Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub length: usize,
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rectangle>,
    pub occupied_tiles: TileFlags,
    pub revealed_tiles: TileFlags,
    pub combatable_tiles: TileFlags,
    pub tile_entities: TileEntities,
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
            occupied_tiles: TileFlags::new(width, length),
            revealed_tiles: TileFlags::new(width, length),
            combatable_tiles: TileFlags::new(width, length),
            tile_entities: TileEntities::new(width, length),
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            let (x, y) = self.get_idx_as_xy(idx);
            if self.revealed_tiles.get_at_idx(idx) {
                let renderable = tile.get_renderable();
                ctx.set(x, y, renderable.fg, renderable.bg, renderable.glyph);
            }
        }
    }

    pub fn get_tiletype_at_idx(&self, idx: usize) -> TileType {
        self.tiles[idx]
    }

    pub fn get_tiletype_at_xy(&self, (x, y): (usize, usize)) -> TileType {
        self.tiles[xy_to_idx(self.width, (x, y))]
    }

    pub fn get_tiletype_at_position(&self, pos: Position) -> TileType {
        self.get_tiletype_at_idx(pos.get_idx(self.width))
    }

    pub fn get_xy_as_idx(&self, (x, y): (usize, usize)) -> usize {
        xy_to_idx(self.width, (x, y))
    }

    pub fn get_idx_as_xy(&self, idx: usize) -> (usize, usize) {
        idx_to_xy(self.width, idx)
    }

    pub fn is_exit_valid_xy(&self, (x, y): (usize, usize)) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.get_xy_as_idx((x, y));
        if !self.tiles[idx as usize].is_walkable() {
            return false;
        }
        !self.occupied_tiles.get_at_idx(idx)
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

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx % self.width;
        let y = idx / self.width;
        let w = self.width;

        // Cardinal directions
        if self.is_exit_valid_xy((x - 1, y)) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid_xy((x + 1, y)) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid_xy((x, y - 1)) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid_xy((x, y + 1)) {
            exits.push((idx + w, 1.0))
        };

        // Diagonals
        if self.is_exit_valid_xy((x - 1, y - 1)) {
            exits.push(((idx - w) - 1, 1.45));
        }
        if self.is_exit_valid_xy((x + 1, y - 1)) {
            exits.push(((idx - w) + 1, 1.45));
        }
        if self.is_exit_valid_xy((x - 1, y + 1)) {
            exits.push(((idx + w) - 1, 1.45));
        }
        if self.is_exit_valid_xy((x + 1, y + 1)) {
            exits.push(((idx + w) + 1, 1.45));
        }

        exits
    }
}
