use rltk::Rltk;
use serde::*;
use specs::prelude::*;

pub mod initialize;
use initialize::initialize_world;

use crate::ecs::components::*;
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
                ctx.cls();
                let map = ecs.fetch::<Map>();
                map.draw(ctx);
                let positions = ecs.read_storage::<HasPosition>();
                let renderables = ecs.read_storage::<HasRenderable>();
                for (pos, render) in (&positions, &renderables).join() {
                    ctx.set(
                        pos.position.x,
                        pos.position.y,
                        render.renderable.fg,
                        render.renderable.bg,
                        render.renderable.glyph,
                    );
                }
                ctx.print(1, 1, &format!("FPS: {}", ctx.fps));

                None
            }
        }
    }
}
