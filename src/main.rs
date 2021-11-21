extern crate derivative;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate rand;
extern crate serde;
extern crate specs;
extern crate specs_derive;

pub mod commands;
pub mod ecs;
pub mod error;
pub mod gui;
pub mod map;
pub mod model;
pub mod neural_net;
pub mod random;
pub mod render;

fn main() -> rltk::BError {
    pretty_env_logger::init();
    trace!("Opening GUI");
    gui::open(128, 128)
}
