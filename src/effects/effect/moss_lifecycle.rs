use specs::prelude::*;

use crate::ecs::components::*;
use crate::effects::{Effect, Spawner};
use crate::lifecycle::Moss as MossLifecycle;
use crate::light::Factory as LightSourceFactory;
use crate::map::Viewshed;
use crate::render::Factory as RenderableFactory;

pub fn moss_lifecycle(ecs: &mut World, spawner: &Spawner, target: Entity) {
    if let Effect::MossLifecycle { next } = spawner.effect {
        use MossLifecycle::*;
        let mut has_moss_lifecycle_storage = ecs.write_storage::<HasMossLifecycle>();
        let mut has_renderable_storage = ecs.write_storage::<HasRenderable>();
        let mut has_light_source_storage = ecs.write_storage::<HasLightSource>();
        let mut has_viewshed_storage = ecs.write_storage::<HasViewshed>();
        match next {
            Seed => {
                if let Some(has_moss_lifecycle) = has_moss_lifecycle_storage.get_mut(target) {
                    has_moss_lifecycle.moss_lifecycle = Seed;
                }
                has_renderable_storage.remove(target);
                has_light_source_storage.remove(target);
                has_viewshed_storage.remove(target);
            }
            Moss => {
                if let Some(has_moss_lifecycle) = has_moss_lifecycle_storage.get_mut(target) {
                    has_moss_lifecycle.moss_lifecycle = Moss;
                }
                has_renderable_storage
                    .insert(
                        target,
                        HasRenderable {
                            renderable: RenderableFactory::Moss.create(),
                        },
                    )
                    .expect("Could not insert renderable.");
                has_light_source_storage
                    .insert(
                        target,
                        HasLightSource {
                            light_source: LightSourceFactory::Moss.create(),
                        },
                    )
                    .expect("Could not insert light source.");
                has_viewshed_storage
                    .insert(
                        target,
                        HasViewshed {
                            viewshed: Viewshed::new(5),
                        },
                    )
                    .expect("Could not insert viewshed.");
            }
        }
    }
}
