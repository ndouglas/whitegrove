use specs::prelude::*;

use crate::effects::{Effect, Spawner};
use crate::spatial_index::*;

pub fn particle(_ecs: &mut World, spawner: &Spawner, idx: usize) {
    if let Effect::BloodSpatter { color } = spawner.effect {
        TILE_BACKGROUNDS.lock().unwrap().set_at_idx(idx, &color);
    }
}
