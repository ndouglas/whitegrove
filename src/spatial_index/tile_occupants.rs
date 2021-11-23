use std::sync::Mutex;

use crate::map::TileOccupants;

lazy_static! {
    pub static ref TILE_OCCUPANTS: Mutex<TileOccupants> =
        Mutex::new(TileOccupants::new(128, 128 * 128));
}

pub fn set_dimensions(width: usize, length: usize) {
    TILE_OCCUPANTS.lock().unwrap().set_dimensions(width, length);
}
