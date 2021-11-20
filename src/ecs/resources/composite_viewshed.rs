use std::collections::HashSet;
use std::iter::Extend;

use crate::map::Viewshed;
use crate::model::Position;

pub struct CompositeViewshed {
    pub visible_positions: HashSet<Position>,
}

impl CompositeViewshed {
    pub fn new() -> Self {
        CompositeViewshed {
            visible_positions: HashSet::new(),
        }
    }

    pub fn get_composited_viewsheds(&self, viewsheds: Vec<&Viewshed>) -> HashSet<Position> {
        let mut result: HashSet<Position> = HashSet::new();
        for viewshed in viewsheds {
            let visible_positions: HashSet<Position> =
                viewshed.visible_positions.iter().copied().collect();
            result.extend(visible_positions);
        }
        result
    }

    pub fn composite_viewsheds(&mut self, viewsheds: Vec<&Viewshed>) {
        self.visible_positions = self.get_composited_viewsheds(viewsheds);
    }
}
