use rltk::{main_loop, BError};

pub mod context;
pub use context::*;
pub mod state;
pub use state::*;

pub fn open(width: usize, height: usize) -> BError {
    let gs = State::new();
    let context = create_context(width, height);
    main_loop(context, gs)
}
