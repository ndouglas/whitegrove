use std::sync::Mutex;

use crate::map::TileLighting;

lazy_static! {
    pub static ref TILE_LIGHTING: Mutex<TileLighting> =
        Mutex::new(TileLighting::new(128, 128 * 128));
}

pub fn set_dimensions(width: usize, length: usize) {
    TILE_LIGHTING.lock().unwrap().set_dimensions(width, length);
}

pub fn set_backgrounds() {
    TILE_LIGHTING.lock().unwrap().set_from_backgrounds();
}
