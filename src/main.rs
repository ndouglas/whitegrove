extern crate derivative;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
extern crate specs;
extern crate specs_derive;

pub mod ecs;
pub mod gui;
pub mod model;
pub mod render;

fn main() -> rltk::BError {
    pretty_env_logger::init();
    trace!("Opening GUI");
    gui::open()
}
