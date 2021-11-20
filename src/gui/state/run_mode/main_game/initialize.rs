use rltk::RGB;
use specs::prelude::*;

use crate::ecs::components::*;
use crate::map::*;
use crate::model::*;
use crate::render::*;

pub fn inject_player(ecs: &mut World, x: usize, y: usize) {
    let player = ecs.create_entity()
        .with(HasPosition {
            position: Position { x: x, y: y },
        })
        .with(HasRenderable {
            renderable: Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            },
        })
        .with(IsPlayer {})
        .build();
    ecs.insert(player);
}

pub fn inject_mobs(ecs: &mut World) {
    for i in 0..10 {
        ecs.create_entity()
            .with(HasPosition {
                position: Position { x: i * 7, y: 20 },
            })
            .with(HasRenderable {
                renderable: Renderable {
                    glyph: rltk::to_cp437('☺'),
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                },
            })
            .with(WantsToMoveWest {})
            .build();
    }
}

pub fn inject_new_map(ecs: &mut World, width: usize, height: usize) {
    ecs.insert(Map::new(width, height));
}

pub fn initialize_world(ecs: &mut World, width: usize, height: usize) {
    inject_player(ecs, width / 2, height / 2);
    inject_mobs(ecs);
    inject_new_map(ecs, width, height);
}
