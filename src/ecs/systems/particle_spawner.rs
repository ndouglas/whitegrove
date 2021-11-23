use specs::prelude::*;

use crate::ecs::components::*;
use crate::particle::Builder as ParticleBuilder;

pub struct ParticleSpawner {}

impl<'a> System<'a> for ParticleSpawner {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, ParticleBuilder>,
        Entities<'a>,
        WriteStorage<'a, HasPosition>,
        WriteStorage<'a, HasRenderable>,
        WriteStorage<'a, HasParticleLifetime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut particle_builder,
            entities,
            mut has_position_storage,
            mut has_renderable_storage,
            mut has_particle_lifetime_storage,
        ) = data;
        for request in particle_builder.requests.iter() {
            let entity = entities.create();
            has_position_storage
                .insert(
                    entity,
                    HasPosition {
                        position: request.position,
                    },
                )
                .expect("Unable to insert position");
            has_renderable_storage
                .insert(
                    entity,
                    HasRenderable {
                        renderable: request.renderable,
                    },
                )
                .expect("Unable to insert renderable");
            has_particle_lifetime_storage
                .insert(
                    entity,
                    HasParticleLifetime {
                        particle_lifetime: request.lifetime,
                    },
                )
                .expect("Unable to insert lifetime");
        }
        particle_builder.requests.clear();
    }
}
