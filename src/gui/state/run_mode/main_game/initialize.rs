use rltk::RGB;
use specs::prelude::*;

use crate::ecs::components::*;
use crate::model::*;
use crate::render::*;

pub fn inject_player(ecs: &mut World) {
    ecs.create_entity()
        .with(HasPosition {
            position: Position { x: 40, y: 25 },
        })
        .with(HasRenderable {
            renderable: Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            },
        })
        .build();
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

pub fn initialize_world(ecs: &mut World) {
    inject_player(ecs);
    inject_mobs(ecs);
}
