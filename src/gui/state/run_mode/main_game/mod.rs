use rltk::{Rltk, RGB};
use serde::*;
use specs::prelude::*;

pub mod initialize;
use initialize::{initialize_world, inject_mobs};
pub mod player;
use player::*;

use crate::ecs::components::*;
use crate::map::*;
use crate::spatial_index::TILE_LIGHTING;

use super::RunMode;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Mode {
    Initialize,
    DoSomeStuff,
}

impl Mode {
    pub fn tick(self, ecs: &mut World, ctx: &mut Rltk) -> Option<RunMode> {
        use Mode::*;
        match self {
            Initialize => {
                initialize_world(ecs, 128, 128);
                Some(RunMode::MainGame {
                    mode: Mode::DoSomeStuff,
                })
            }
            DoSomeStuff => {
                {
                    let map = ecs.fetch::<Map>();
                    map.draw(ctx);
                }
                let entity_count;
                {
                    let has_position_storage = ecs.read_storage::<HasPosition>();
                    let has_renderable_storage = ecs.read_storage::<HasRenderable>();
                    let tile_lighting = TILE_LIGHTING.lock().unwrap();
                    for (pos, render) in (&has_position_storage, &has_renderable_storage).join() {
                        ctx.set(
                            pos.position.x,
                            pos.position.y,
                            render.renderable.fg,
                            tile_lighting.get_at_position(&pos.position),
                            render.renderable.glyph,
                        );
                    }
                }
                {
                    let entities = ecs.entities();
                    let has_hit_points_storage = ecs.read_storage::<HasHitPoints>();
                    entity_count = (&entities, &has_hit_points_storage).join().count();
                }
                ctx.print_color(
                    1,
                    1,
                    RGB::named(rltk::RED),
                    RGB::named(rltk::BLACK),
                    &format!("FPS: {}   Entities: {}", ctx.fps, entity_count),
                );
                if ctx.fps >= 59.9 {
                    {
                        let rooms;
                        let map_width;
                        let map_height;
                        {
                            let map = ecs.fetch::<Map>();
                            map_width = map.width;
                            map_height = map.height;
                            rooms = map.rooms.clone();
                        }
                        inject_mobs(ecs, &rooms, 1, (map_width, map_height));
                    }
                }
                player_input(ecs, ctx);
                None
            }
        }
    }

    pub fn should_maintain_ecs(self) -> bool {
        use Mode::*;
        match self {
            Initialize => false,
            DoSomeStuff => true,
        }
    }
}
