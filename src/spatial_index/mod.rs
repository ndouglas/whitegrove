pub mod moss_seeded_tiles;
pub use moss_seeded_tiles::MOSS_SEEDED_TILES;
pub mod moss_tiles;
pub use moss_tiles::MOSS_TILES;
pub mod occupied_tiles;
pub use occupied_tiles::OCCUPIED_TILES;
pub mod revealed_tiles;
pub use revealed_tiles::REVEALED_TILES;
pub mod tile_backgrounds;
pub use tile_backgrounds::TILE_BACKGROUNDS;
pub mod tile_entities;
pub use tile_entities::TILE_ENTITIES;
pub mod tile_lighting;
pub use tile_lighting::TILE_LIGHTING;
pub mod tile_occupants;
pub use tile_occupants::TILE_OCCUPANTS;

pub fn set_spatial_index_dimensions(width: usize, length: usize) {
    moss_seeded_tiles::set_dimensions(width, length);
    moss_tiles::set_dimensions(width, length);
    occupied_tiles::set_dimensions(width, length);
    revealed_tiles::set_dimensions(width, length);
    tile_backgrounds::set_dimensions(width, length);
    tile_entities::set_dimensions(width, length);
    tile_lighting::set_dimensions(width, length);
    tile_occupants::set_dimensions(width, length);
}
