use rltk::RGB;
use specs::prelude::*;
use std::fmt;

use crate::spatial_index::TILE_ENTITIES;

pub mod blood_spatter;
pub use blood_spatter::*;
pub mod inflict_damage;
pub use inflict_damage::*;

use super::Spawner;

#[derive(Clone, Debug)]
pub enum Effect {
    Damage { amount: i32 },
    BloodSpatter { color: RGB },
}

impl Effect {
    pub fn affects_entities(&self) -> bool {
        use Effect::*;
        match self {
            Damage { .. } => true,
            _ => false,
        }
    }

    pub fn affect_tile(&self, ecs: &mut World, spawner: &Spawner, idx: usize) {
        if self.affects_entities() {
            TILE_ENTITIES
                .lock()
                .unwrap()
                .get_at_idx(idx)
                .iter()
                .for_each(|entity| self.affect_entity(ecs, spawner, *entity));
        }
        use Effect::*;
        match self {
            BloodSpatter { .. } => blood_spatter(ecs, spawner, idx),
            _ => {}
        }
    }

    pub fn affect_entity(&self, ecs: &mut World, spawner: &Spawner, target: Entity) {
        use Effect::*;
        match self {
            Damage { .. } => inflict_damage(ecs, spawner, target),
            _ => {}
        }
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
