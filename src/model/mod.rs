pub mod compass;
pub use compass::*;
pub mod position;
pub use position::*;
pub mod rectangle;
pub use rectangle::*;
pub mod unsafe_position;
pub use unsafe_position::*;

pub fn idx_to_xy(width: usize, idx: usize) -> (usize, usize) {
    let x = idx % width;
    let y = idx / width;
    (x, y)
}

pub fn xy_to_idx(width: usize, (x, y): (usize, usize)) -> usize {
    y * width + x
}

pub fn get_dim_distance(a: usize, b: usize) -> usize {
    (a as i32 - b as i32).abs() as usize
}
