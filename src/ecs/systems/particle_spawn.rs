pub struct ParticleSpawn {}

impl<'a> System<'a> for ParticleSpawn {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, HasPosition>,
        WriteStorage<'a, HasRenderable>,
        WriteStorage<'a, ParticleLifetime>,
        WriteExpect<'a, ParticleBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut positions, mut renderables, mut particles, mut particle_builder) = data;
        for new_particle in particle_builder.requests.iter() {
            let p = entities.create();
            positions
                .insert(
                    p,
                    Position {
                        x: new_particle.x,
                        y: new_particle.y,
                    },
                )
                .expect("Unable to inser position");
            renderables
                .insert(
                    p,
                    Renderable {
                        fg: new_particle.fg,
                        bg: new_particle.bg,
                        glyph: new_particle.glyph,
                        render_order: 0,
                    },
                )
                .expect("Unable to insert renderable");
            particles
                .insert(
                    p,
                    ParticleLifetime {
                        lifetime_ms: new_particle.lifetime,
                        animation: None,
                    },
                )
                .expect("Unable to insert lifetime");
        }

        particle_builder.requests.clear();
    }
}
