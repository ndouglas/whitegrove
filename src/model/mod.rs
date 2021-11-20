pub mod compass;
pub use compass::*;
pub mod position;
pub use position::*;
pub mod unsafe_position;
pub use unsafe_position::*;

pub fn idx_to_xy(width: usize, idx: usize) -> (usize, usize) {
    let x = idx % width;
    let y = idx / width;
    (x, y)
}

pub fn xy_to_idx(width: usize, x: usize, y: usize) -> usize {
    y * width + x
}
