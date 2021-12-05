use specs::prelude::*;

use crate::effects::{Effect, Spawner};
use crate::model::Position;
use crate::spatial_index::*;

pub fn blood_spatter(_ecs: &mut World, spawner: &Spawner, position: &Position) {
    if let Effect::BloodSpatter { color } = spawner.effect {
        TILE_BACKGROUNDS
            .lock()
            .unwrap()
            .set_at_position(position, &color);
    }
}
