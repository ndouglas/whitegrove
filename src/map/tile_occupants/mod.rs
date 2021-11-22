use specs::prelude::*;

use crate::model::*;

#[derive(Clone, Debug, Default)]
pub struct TileOccupants {
    pub width: usize,
    pub length: usize,
    pub vector: Vec<Option<Entity>>,
}

impl TileOccupants {
    pub fn new(width: usize, length: usize) -> Self {
        TileOccupants {
            width: width,
            length: length,
            vector: vec![None; length],
        }
    }

    pub fn get_xy_as_idx(&self, (x, y): (usize, usize)) -> usize {
        xy_to_idx(self.width, (x, y))
    }

    pub fn get_position_as_idx(&self, position: &Position) -> usize {
        xy_to_idx(self.width, (position.x, position.y))
    }

    pub fn get_at_idx(&self, idx: usize) -> Option<Entity> {
        trace!("Getting occupant at {}", idx);
        self.vector[idx]
    }

    pub fn get_at_xy(&self, (x, y): (usize, usize)) -> Option<Entity> {
        self.get_at_idx(self.get_xy_as_idx((x, y)))
    }

    pub fn get_at_position(&self, position: &Position) -> Option<Entity> {
        self.get_at_idx(self.get_position_as_idx(position))
    }

    pub fn clear(&mut self) {
        trace!("Clearing vector.");
        self.vector = vec![None; self.length];
    }

    pub fn set_at_idx(&mut self, idx: usize, entity: Entity) {
        trace!("Setting occupant {:?} at {}.", entity, idx);
        self.vector[idx] = Some(entity);
    }

    pub fn set_at_xy(&mut self, (x, y): (usize, usize), entity: Entity) {
        self.set_at_idx(self.get_xy_as_idx((x, y)), entity);
    }

    pub fn set_at_position(&mut self, position: &Position, entity: Entity) {
        self.set_at_xy((position.x, position.y), entity);
    }

    pub fn remove_at_idx(&mut self, idx: usize) {
        trace!("Removing occupant at {}.", idx);
        self.vector[idx] = None;
    }

    pub fn remove_at_xy(&mut self, (x, y): (usize, usize)) {
        self.remove_at_idx(self.get_xy_as_idx((x, y)));
    }

    pub fn remove_at_position(&mut self, position: &Position) {
        self.remove_at_xy((position.x, position.y));
    }
}
