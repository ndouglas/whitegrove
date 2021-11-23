use specs::prelude::*;
use std::collections::HashSet;
use std::iter::Extend;

use crate::ecs::components::*;
use crate::map::viewshed::Viewshed;
use crate::model::Position;
use crate::spatial_index::REVEALED_TILES;

fn get_composited_viewsheds(viewsheds: Vec<&Viewshed>) -> Vec<&Position> {
    let mut result: HashSet<&Position> = HashSet::new();
    for viewshed in viewsheds {
        let visible_positions: HashSet<&Position> = viewshed
            .visible_positions
            .iter()
            .map(|position| position)
            .collect();
        result.extend(visible_positions);
    }
    result.into_iter().collect()
}

pub struct IndexRevealedTiles {}

impl<'a> System<'a> for IndexRevealedTiles {
    type SystemData = (Entities<'a>, ReadStorage<'a, HasViewshed>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, has_viewshed_storage) = data;
        let viewsheds: Vec<&Viewshed> = (&entities, &has_viewshed_storage)
            .join()
            .map(|(_entity, has_viewshed)| &has_viewshed.viewshed)
            .collect();
        let positions = get_composited_viewsheds(viewsheds);
        REVEALED_TILES
            .lock()
            .unwrap()
            .set_at_positions(&positions, true);
    }
}
