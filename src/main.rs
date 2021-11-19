extern crate derivative;
extern crate serde;
extern crate specs;
extern crate specs_derive;

pub mod ecs;
pub mod gui;
pub mod model;
pub mod render;

fn main() -> rltk::BError {
    gui::open()
}
