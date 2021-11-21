use rltk::Rltk;
use serde::*;
use specs::prelude::*;

pub mod main_game;
use main_game::Mode as MainGameMode;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum RunMode {
    Initial,
    MainGame { mode: MainGameMode },
}

impl RunMode {
    pub fn tick(self, ecs: &mut World, ctx: &mut Rltk) -> Option<RunMode> {
        use RunMode::*;
        match self {
            Initial => Some(MainGame {
                mode: MainGameMode::Initialize,
            }),
            MainGame { mode } => mode.tick(ecs, ctx),
        }
    }

    pub fn should_maintain_ecs(self) -> bool {
        use RunMode::*;
        match self {
            Initial => false,
            MainGame { mode } => mode.should_maintain_ecs(),
        }
    }
}
