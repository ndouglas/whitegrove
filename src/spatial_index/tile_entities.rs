use std::sync::Mutex;

use crate::map::TileEntities;

lazy_static! {
    pub static ref TILE_ENTITIES: Mutex<TileEntities> =
        Mutex::new(TileEntities::new(128, 128 * 128));
}

pub fn set_dimensions(width: usize, length: usize) {
    TILE_ENTITIES.lock().unwrap().set_dimensions(width, length);
}
