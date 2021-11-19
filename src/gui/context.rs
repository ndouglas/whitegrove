use rltk::{Rltk, RltkBuilder};

pub fn create_context() -> Rltk {
    RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
        .expect("Could not create context.")
}
