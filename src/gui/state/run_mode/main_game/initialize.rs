use specs::prelude::*;

use crate::ecs::components::*;
use crate::ecs::resources::CompositeViewshed as CompositeViewshedResource;
use crate::map::*;
use crate::model::*;
use crate::random;
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
        .with(HasName {
            name: "Player".to_string(),
        })
        .build();
    ecs.insert(player);
}

pub fn inject_mobs(ecs: &mut World, rooms: &Vec<Rectangle>) {
    for _i in 0..2000 {
        let room = rooms[random::range(0, rooms.len())];
        let (spawn_x, spawn_y) = room.get_center_xy();
        let roll = random::roll_dice(1, 3);
        let renderable;
        let name;
        match roll {
            1 => {
                renderable = RenderableFactory::Orc.create();
                name = "Orc";
            }
            2 => {
                renderable = RenderableFactory::Goblin.create();
                name = "Goblin";
            }
            _ => {
                renderable = RenderableFactory::Idiot.create();
                name = "Idiot";
            }
        }
        ecs.create_entity()
            .with(HasPosition {
                position: Position {
                    x: spawn_x,
                    y: spawn_y,
                },
            })
            .with(HasRenderable {
                renderable: renderable,
            })
            .with(HasViewshed {
                viewshed: Viewshed::new(8),
            })
            .with(WantsToMove::Randomly { duration: 5 })
            .with(HasName {
                name: name.to_string(),
            })
            .build();
    }
}

pub fn initialize_world(ecs: &mut World, width: usize, height: usize) {
    let map = Map::new(width, height);
    let (spawn_x, spawn_y) = map.rooms[0].get_center_xy();
    inject_player(ecs, spawn_x, spawn_y);
    inject_mobs(ecs, &map.rooms);
    ecs.insert(map);
    ecs.insert(CompositeViewshedResource::new());
}
