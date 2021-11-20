use serde::*;
use std::sync::Mutex;

use crate::render::Renderable;

pub mod floor;
use floor::*;
pub mod wall;
use wall::*;

pub trait TileTrait {
    fn get_renderable(&self) -> Renderable;
    fn is_walkable(&self) -> bool;
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum TileType {
    Floor,
    Wall,
}

lazy_static! {
    static ref FLOOR: Mutex<Floor> = Mutex::new(Floor {});
    static ref WALL: Mutex<Wall> = Mutex::new(Wall {});
}

impl TileType {
}

impl TileTrait for TileType {

    fn get_renderable(&self) -> Renderable {
        use TileType::*;
        match self {
            Floor => FLOOR.lock().unwrap().get_renderable(),
            Wall => WALL.lock().unwrap().get_renderable(),
        }
    }

    fn is_walkable(&self) -> bool {
        use TileType::*;
        match self {
            Floor => FLOOR.lock().unwrap().is_walkable(),
            Wall => WALL.lock().unwrap().is_walkable(),
        }
    }

}
