use rltk::Rltk;
use specs::prelude::*;

use crate::ecs::components::HasParticleLifetime;

pub fn collect_garbage(ecs: &mut World, ctx: &Rltk) {
    let mut dead_particles: Vec<Entity> = Vec::new();
    {
        let mut has_particle_lifetime_storage = ecs.write_storage::<HasParticleLifetime>();
        let entities = ecs.entities();
        for (entity, mut has_particle_lifetime) in
            (&entities, &mut has_particle_lifetime_storage).join()
        {
            has_particle_lifetime.particle_lifetime.lifetime -= ctx.frame_time_ms;
            if has_particle_lifetime.particle_lifetime.lifetime < 0.0 {
                dead_particles.push(entity);
            }
        }
    }
    for entity in dead_particles.iter() {
        ecs.delete_entity(*entity).expect("Particle will not die!");
    }
}
