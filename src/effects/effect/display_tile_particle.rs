use specs::prelude::*;

use crate::effects::{Effect, Spawner};
use crate::map::Map;
use crate::particle::{Builder as ParticleBuilder, Request as ParticleRequest};

pub fn display_tile_particle(ecs: &mut World, spawner: &Spawner, idx: usize) {
    if let Effect::DisplayTileParticle {
        renderable,
        lifetime,
    } = spawner.effect
    {
        let map = ecs.fetch::<Map>();
        let mut particle_builder = ecs.fetch_mut::<ParticleBuilder>();
        let position = map.get_idx_as_position(idx);
        let request = ParticleRequest::new(position, renderable, lifetime);
        particle_builder.add_request(request);
    }
}
