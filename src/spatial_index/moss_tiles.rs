use std::sync::Mutex;

use crate::map::TileFlags;

lazy_static! {
    pub static ref MOSS_TILES: Mutex<TileFlags> = Mutex::new(TileFlags::new(128, 128 * 128));
}

pub fn set_dimensions(width: usize, length: usize) {
    MOSS_TILES.lock().unwrap().set_dimensions(width, length);
}
