use rltk::{to_cp437, RGB};
use specs::prelude::*;

use crate::ecs::components::*;
use crate::effects::{enqueue_effect, get_entity_position, Effect, Spawner, Target};
use crate::particle::Lifetime as ParticleLifetime;
use crate::random;
use crate::render::Renderable;

pub fn inflict_damage(ecs: &mut World, spawner: &Spawner, target: Entity) {
    if let Effect::Damage { amount } = spawner.effect {
        let mut has_hit_points_storage = ecs.write_storage::<HasHitPoints>();
        if let Some(has_hit_points) = has_hit_points_storage.get_mut(target) {
            has_hit_points.hit_points.current -= amount;
            if let Some(position) = get_entity_position(ecs, target) {
                let color_name = match random::range(0, 11) {
                    1 => rltk::INDIAN_RED,
                    2 => rltk::MEDIUMVIOLETRED,
                    3 => rltk::ORANGE_RED,
                    4 => rltk::PALEVIOLETRED,
                    5 => rltk::RED,
                    6 => rltk::RED1,
                    7 => rltk::RED2,
                    8 => rltk::RED3,
                    9 => rltk::RED4,
                    10 => rltk::DARKSALMON,
                    _ => rltk::DARK_RED,
                };
                enqueue_effect(
                    None,
                    Effect::BloodSpatter {
                        color: RGB::named(color_name),
                    },
                    Target::Tile {
                        index: position.idx,
                    },
                );
                enqueue_effect(
                    None,
                    Effect::DisplayTileParticle {
                        renderable: Renderable {
                            glyph: to_cp437('‼'),
                            fg: RGB::named(rltk::ORANGE),
                            bg: RGB::named(color_name),
                            render_order: i32::MAX,
                        },
                        lifetime: ParticleLifetime { lifetime: 200.0 },
                    },
                    Target::Entity { target },
                );
            }
        }
    }
}
