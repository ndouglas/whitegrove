use rltk::{Rltk, RltkBuilder};

pub fn create_context(width: usize, height: usize) -> Rltk {
    RltkBuilder::simple(width, height)
        .unwrap()
        .with_title("Whitegrove")
        .with_vsync(false)
        .build()
        .expect("Could not create context.")
}
