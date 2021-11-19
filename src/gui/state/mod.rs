use rltk::{GameState, Rltk};

pub mod run_mode;
pub use run_mode::*;

pub struct State {
    run_mode: RunMode,
}

impl State {

    pub fn new() -> Self {
        State {
            run_mode: RunMode::Initial,
        }
    }

}

impl GameState for State {

    fn tick(&mut self, ctx: &mut Rltk) {
        if let Some(new_mode) = self.run_mode.tick(ctx) {
            self.run_mode = new_mode;
        }
    }

}
