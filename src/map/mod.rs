use rltk::{a_star_search, Algorithm2D, BaseMap, NavigationPath, Point, Rltk};

use crate::model::{idx_to_xy, xy_to_idx, Position, Rectangle};
use crate::spatial_index::*;

pub mod tile;
pub use tile::*;
pub mod tile_backgrounds;
pub use tile_backgrounds::*;
pub mod tile_entities;
pub use tile_entities::*;
pub mod tile_flags;
pub use tile_flags::*;
pub mod tile_lighting;
pub use tile_lighting::*;
pub mod tile_map;
pub use tile_map::*;
pub mod tile_occupants;
pub use tile_occupants::*;
pub mod viewshed;
pub use viewshed::*;

#[derive(Clone, Default, Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub length: usize,
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rectangle>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let length = width * height;
        let (tiles, rooms) = get_rooms_and_corridors_tile_map(width, height);
        set_spatial_index_dimensions(width, length);
        Map {
            width: width,
            height: height,
            length: length,
            tiles: tiles,
            rooms: rooms,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        let revealed_tiles = REVEALED_TILES.lock().unwrap();
        let tile_backgrounds = TILE_BACKGROUNDS.lock().unwrap();
        let tile_lighting = TILE_LIGHTING.lock().unwrap();
        for (idx, tile) in self.tiles.iter().enumerate() {
            let (x, y) = self.get_idx_as_xy(idx);
            if revealed_tiles.get_at_idx(idx) {
                let renderable = tile.get_renderable();
                let mut bg = renderable.bg;
                bg = tile_lighting.get_at_idx(idx);
                if let Some(rgb) = tile_backgrounds.get_at_idx(idx) {
                    //bg = rgb;
                }
                ctx.set(x, y, renderable.fg, bg, renderable.glyph);
            }
        }
    }

    pub fn get_tiletype_at_idx(&self, idx: usize) -> TileType {
        self.tiles[idx]
    }

    pub fn get_tiletype_at_xy(&self, (x, y): (usize, usize)) -> TileType {
        self.tiles[xy_to_idx(self.width, (x, y))]
    }

    pub fn get_tiletype_at_position(&self, position: Position) -> TileType {
        self.get_tiletype_at_idx(position.idx)
    }

    pub fn get_xy_as_idx(&self, (x, y): (usize, usize)) -> usize {
        xy_to_idx(self.width, (x, y))
    }

    pub fn get_idx_as_xy(&self, idx: usize) -> (usize, usize) {
        idx_to_xy(self.width, idx)
    }

    pub fn get_position_as_idx(&self, position: &Position) -> usize {
        self.get_xy_as_idx((position.x, position.y))
    }

    pub fn get_idx_as_position(&self, idx: usize) -> Position {
        Position::from_idx(idx, (self.width, self.height))
    }

    pub fn is_exit_valid_xy(&self, (x, y): (usize, usize)) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.get_xy_as_idx((x, y));
        if !self.tiles[idx as usize].is_walkable() {
            return false;
        }
        !OCCUPIED_TILES.lock().unwrap().get_at_idx(idx)
    }

    pub fn get_astar_path_idx(&mut self, idx1: usize, idx2: usize) -> NavigationPath {
        a_star_search(idx1 as usize, idx2 as usize, self)
    }

    pub fn get_next_astar_step_idx(&mut self, idx1: usize, idx2: usize) -> Option<usize> {
        let path = self.get_astar_path_idx(idx1, idx2);
        if path.success && path.steps.len() > 1 {
            Some(path.steps[1])
        } else {
            None
        }
    }

    pub fn get_next_astar_step_xy(
        &mut self,
        xy1: (usize, usize),
        xy2: (usize, usize),
    ) -> Option<(usize, usize)> {
        match self.get_next_astar_step_idx(self.get_xy_as_idx(xy1), self.get_xy_as_idx(xy2)) {
            None => None,
            Some(idx) => Some(self.get_idx_as_xy(idx)),
        }
    }

    pub fn get_next_astar_step_position(
        &mut self,
        position1: &Position,
        position2: &Position,
    ) -> Option<Position> {
        match self.get_next_astar_step_idx(
            self.get_position_as_idx(position1),
            self.get_position_as_idx(position2),
        ) {
            None => None,
            Some(idx) => Some(self.get_idx_as_position(idx)),
        }
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
