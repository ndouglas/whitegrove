use specs::prelude::*;

use crate::effects::{Effect, Spawner};
use crate::model::Position;
use crate::particle::{Builder as ParticleBuilder, Request as ParticleRequest};

pub fn display_tile_particle(ecs: &mut World, spawner: &Spawner, position: &Position) {
    if let Effect::DisplayTileParticle {
        renderable,
        lifetime,
    } = spawner.effect
    {
        let mut particle_builder = ecs.fetch_mut::<ParticleBuilder>();
        let request = ParticleRequest::new(*position, renderable, lifetime);
        particle_builder.add_request(request);
    }
}
