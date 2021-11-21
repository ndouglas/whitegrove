use specs::prelude::*;

use crate::model::*;

#[derive(Clone, Debug, Default)]
pub struct TileEntities {
    pub width: usize,
    pub length: usize,
    pub vector: Vec<Vec<Entity>>,
}

impl TileEntities {

    pub fn new(width: usize, length: usize) -> Self {
        TileEntities {
            width: width,
            length: length,
            vector: vec![Vec::new(); length],
        }
    }

    pub fn get_xy_as_idx(&self, (x, y): (usize, usize)) -> usize {
        xy_to_idx(self.width, (x, y))
    }

    pub fn get_position_as_idx(&self, position: &Position) -> usize {
        xy_to_idx(self.width, (position.x, position.y))
    }

    pub fn get_at_idx(&self, idx: usize) -> &Vec<Entity> {
        &self.vector[idx]
    }

    pub fn get_at_xy(&self, (x, y): (usize, usize)) -> &Vec<Entity> {
        self.get_at_idx(self.get_xy_as_idx((x, y)))
    }

    pub fn get_at_position(&self, position: &Position) -> &Vec<Entity> {
        self.get_at_idx(self.get_position_as_idx(position))
    }

    pub fn clear(&mut self) {
        for vector in self.vector.iter_mut() {
            vector.clear();
        }
    }

    pub fn clear_at_idx(&mut self, idx: usize) {
        self.vector[idx].clear();
    }

    pub fn clear_at_xy(&mut self, (x, y): (usize, usize)) {
        self.clear_at_idx(self.get_xy_as_idx((x, y)));
    }

    pub fn clear_at_position(&mut self, position: &Position) {
        self.clear_at_xy((position.x, position.y));
    }

    pub fn add_at_idx(&mut self, idx: usize, entity: Entity) {
        self.vector[idx].push(entity);
    }

    pub fn add_at_xy(&mut self, (x, y): (usize, usize), entity: Entity) {
        self.add_at_idx(self.get_xy_as_idx((x, y)), entity);
    }

    pub fn add_at_position(&mut self, position: Position, entity: Entity) {
        self.add_at_xy((position.x, position.y), entity);
    }

    pub fn remove_at_idx(&mut self, idx: usize, entity: Entity) {
        self.vector[idx].retain(|&ent| ent != entity);
    }

    pub fn remove_at_xy(&mut self, (x, y): (usize, usize), entity: Entity) {
        self.remove_at_idx(self.get_xy_as_idx((x, y)), entity);
    }

    pub fn remove_at_position(&mut self, position: Position, entity: Entity) {
        self.remove_at_xy((position.x, position.y), entity);
    }

}
