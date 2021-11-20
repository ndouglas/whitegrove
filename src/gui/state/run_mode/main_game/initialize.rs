use specs::prelude::*;

use crate::ecs::components::*;
use crate::ecs::resources::CompositeViewshed as CompositeViewshedResource;
use crate::map::*;
use crate::model::*;
use crate::render::Factory as RenderableFactory;

pub fn inject_player(ecs: &mut World, x: usize, y: usize) {
    let player = ecs
        .create_entity()
        .with(HasPosition {
            position: Position { x: x, y: y },
        })
        .with(HasRenderable {
            renderable: RenderableFactory::Player.create(),
        })
        .with(HasViewshed {
            viewshed: Viewshed::new(10),
        })
        .with(IsPlayer {})
        .build();
    ecs.insert(player);
}

pub fn inject_mobs(ecs: &mut World) {
    for i in 0..100 {
        ecs.create_entity()
            .with(HasPosition {
                position: Position {
                    x: (i + 1) * 5 % 128,
                    y: (i + 1) * 5 % 128,
                },
            })
            .with(HasRenderable {
                renderable: RenderableFactory::Monster.create(),
            })
            .with(HasViewshed {
                viewshed: Viewshed::new(8),
            })
            .with(WantsToMove {
                compass_direction: CompassDirection::West,
            })
            .build();
    }
}

pub fn inject_new_map(ecs: &mut World, width: usize, height: usize) {
    ecs.insert(Map::new(width, height));
}

pub fn initialize_world(ecs: &mut World, width: usize, height: usize) {
    inject_new_map(ecs, width, height);
    ecs.insert(CompositeViewshedResource::new());
    inject_player(ecs, width / 2, height / 2);
    inject_mobs(ecs);
}
