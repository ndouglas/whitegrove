use rltk::RGB;
use specs::prelude::*;
use std::fmt;

use crate::particle::Lifetime as ParticleLifetime;
use crate::model::Position;
use crate::render::Renderable;
use crate::spatial_index::TILE_ENTITIES;

pub mod blood_spatter;
pub use blood_spatter::*;
pub mod display_tile_particle;
pub use display_tile_particle::*;
pub mod inflict_damage;
pub use inflict_damage::*;
pub mod update_position;
pub use update_position::*;

use crate::entity::get_entity_position;

use super::Spawner;

#[derive(Clone, Debug)]
pub enum Effect {
    Damage {
        amount: i32,
    },
    BloodSpatter {
        color: RGB,
    },
    DisplayTileParticle {
        renderable: Renderable,
        lifetime: ParticleLifetime,
    },
    UpdatePosition {
        new_position: Position,
    },
}

impl Effect {
    pub fn affects_entities(&self) -> bool {
        use Effect::*;
        match self {
            Damage { .. } => true,
            UpdatePosition { .. } => true,
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
            DisplayTileParticle { .. } => display_tile_particle(ecs, spawner, idx),
            _ => {}
        }
    }

    pub fn affect_entity(&self, ecs: &mut World, spawner: &Spawner, target: Entity) {
        use Effect::*;
        match self {
            Damage { .. } => inflict_damage(ecs, spawner, target),
            DisplayTileParticle { .. } => {
                if let Some(position) = get_entity_position(ecs, target) {
                    display_tile_particle(ecs, spawner, position.idx);
                }
            }
            UpdatePosition { .. } => update_position(ecs, spawner, target),
            _ => {}
        }
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
