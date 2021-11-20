use rltk::Rltk;
use serde::*;
use specs::prelude::*;

pub mod initialize;
use initialize::initialize_world;
pub mod player;
use player::*;

use crate::ecs::components::*;
use crate::ecs::systems::run_systems;
use crate::map::*;

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
                {
                    let has_position_storage = ecs.read_storage::<HasPosition>();
                    let has_renderable_storage = ecs.read_storage::<HasRenderable>();
                    for (pos, render) in (&has_position_storage, &has_renderable_storage).join() {
                        ctx.set(
                            pos.position.x,
                            pos.position.y,
                            render.renderable.fg,
                            render.renderable.bg,
                            render.renderable.glyph,
                        );
                    }
                }
                ctx.print(1, 1, &format!("FPS: {}", ctx.fps));
                player_input(ecs, ctx);
                run_systems(ecs);
                ecs.maintain();
                None
            }
        }
    }
}
