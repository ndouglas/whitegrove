use serde::*;

use crate::map::Map;
use crate::model::Position;

pub mod field_of_view;
use field_of_view::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Viewshed {
    pub visible_positions: Vec<Position>,
    pub range: usize,
}

impl Viewshed {
    pub fn new(range: usize) -> Self {
        Viewshed {
            visible_positions: Vec::new(),
            range: range,
        }
    }

    pub fn contains(&self, pos: &Position) -> bool {
        self.visible_positions.contains(pos)
    }

    pub fn contains_xy(&self, (x, y): (usize, usize)) -> bool {
        self.contains(&Position {
          x, y,
        })
    }

    pub fn recalculate(&mut self, pos: &Position, map: &Map) {
        self.visible_positions.clear();
        self.visible_positions = field_of_view(pos.x as i32, pos.y as i32, self.range.try_into().unwrap(), map);
    }

}
