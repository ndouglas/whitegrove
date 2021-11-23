use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::particle::Lifetime as ParticleLifetime;

#[derive(Component, Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HasParticleLifetime {
    pub particle_lifetime: ParticleLifetime,
}
