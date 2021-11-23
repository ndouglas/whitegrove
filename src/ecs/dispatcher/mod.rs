use specs::prelude::World;
use std::fmt;

use crate::ecs::systems::*;

#[cfg(target_arch = "wasm32")]
#[macro_use]
mod single_thread;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
mod multi_thread;

#[cfg(target_arch = "wasm32")]
pub use single_thread::*;

#[cfg(not(target_arch = "wasm32"))]
pub use multi_thread::*;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs: *mut World);
}

impl fmt::Display for Box<dyn UnifiedDispatcher + 'static> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnifiedDispatcher")
    }
}

construct_dispatcher!(
    (Ai, "ai", &[]),
    (Movement, "movement", &[]),
    (IndexTileEntities, "index_tile_entities", &[]),
    (IndexTileOccupants, "index_tile_occupants", &[]),
    (IndexOccupiedTiles, "index_occupied_tiles", &[]),
    (Visibility, "visibility", &[]),
    (IndexRevealedTiles, "index_revealed_tiles", &[]),
    (MeleeCombat, "melee_combat", &[]),
    (ParticleSpawner, "particle_spawner", &[])
);

pub fn get_new_dispatcher() -> Box<dyn UnifiedDispatcher + 'static> {
    new_dispatch()
}
