use rltk::RGB;
use std::cmp::max;

use crate::model::{ Position, get_dim_distance };

/// Something that gives off light.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Source {
    /// The color of light produced.
    pub color: RGB,
    /// Radius of lit area.
    pub radius: usize,
    /// The intensity of the light produced.
    pub intensity: usize,
}

/// Convert -1.0..1.0 float to u8.
#[inline]
pub fn get_f32_as_u8(float: f32) -> u8 {
    (float * 255.0) as u8
}

/// Something that gives off light.
impl Source {

    pub fn new(color: RGB, radius: usize, intensity: usize) -> Self {
        Source {
            color: color,
            radius: radius,
            intensity: intensity,
        }
    }


    /// Compute intensity of light at a distance.
    pub fn intensity_at(&self, source_position: &Position, lit_position: &Position) -> usize {
        let dx = get_dim_distance(source_position.x, lit_position.x);
        let dy = get_dim_distance(source_position.y, lit_position.y);
        if dx > self.radius || dy > self.radius {
            0
        } else {
            let distance = ((dx as f64).powi(2) + (dy as f64).powi(2)).sqrt();
            let coefficient = -(self.intensity as f64) / (self.radius as f64);
            let result: i32 = (self.intensity as f64 + distance * coefficient) as i32;
            let result = max(result, 0) as usize;
            result
        }
    }

    /// Transform a color at a specified distance.
    pub fn transform_color_at(&self, color: RGB, source_position: &Position, lit_position: &Position) -> RGB {
        let intensity = self.intensity_at(source_position, lit_position);
        let multiplier = intensity as f64 / 512 as f64;
        let red = get_f32_as_u8(color.r);
        let green = get_f32_as_u8(color.g);
        let blue = get_f32_as_u8(color.b);
        let my_red = get_f32_as_u8(self.color.r);
        let my_green = get_f32_as_u8(self.color.g);
        let my_blue = get_f32_as_u8(self.color.b);
        let r_diff = (my_red as i32 - red as i32).abs();
        let g_diff = (my_green as i32 - green as i32).abs();
        let b_diff = (my_blue as i32 - blue as i32).abs();
        let new_r = (red as f64 + (r_diff as f64 * multiplier)) as u8;
        let new_g = (green as f64 + (g_diff as f64 * multiplier)) as u8;
        let new_b = (blue as f64 + (b_diff as f64 * multiplier)) as u8;
        RGB::from_u8(new_r, new_g, new_b)
    }
}
