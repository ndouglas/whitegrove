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

pub mod combat;
pub mod commands;
pub mod dice;
pub mod ecs;
pub mod effects;
pub mod entity;
pub mod error;
pub mod garbage_collector;
pub mod gui;
pub mod lifecycle;
pub mod light;
pub mod map;
pub mod model;
pub mod neural_net;
pub mod particle;
pub mod random;
pub mod render;
pub mod spatial_index;

fn main() -> rltk::BError {
    pretty_env_logger::init();
    trace!("Opening GUI");
    gui::open(128, 128)
}
