use rltk::{ BError, main_loop };

pub mod context;
pub use context::*;
pub mod state;
pub use state::*;

pub fn open() -> BError {
    let gs = State::new();
    let context = create_context();
    main_loop(context, gs)
}
