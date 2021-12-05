use rltk::RGB;
use std::cmp::max;

use crate::model::{get_dim_distance, Position};

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
    pub fn get_intensity_at(&self, source_position: &Position, lit_position: &Position) -> usize {
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

    /// Compute multiplier of light at a distance.
    pub fn get_intensity_multiplier_at(&self, source_position: &Position, lit_position: &Position) -> f64 {
        self.get_intensity_at(source_position, lit_position) as f64 / 512 as f64
    }

    /// Transform a color component at a specified distance.
    pub fn transform_color_component(&self, transformer: f32, base: f32, multiplier: f64) -> u8 {
        let new_base = (base * 255.0) as f64;
        let new_trans = (transformer * 255.0) as u8;
        let diff = (new_trans as i32 - new_base as i32).abs() as f64;
        (new_base + (diff * multiplier)) as u8
    }

    /// Transform a color at a specified distance.
    pub fn transform_color_at(
        &self,
        color: RGB,
        source_position: &Position,
        lit_position: &Position,
    ) -> RGB {
        let multiplier = self.get_intensity_multiplier_at(source_position, lit_position);
        let my_color = self.color.clone();
        let new_r = self.transform_color_component(my_color.r, color.r, multiplier);
        let new_g = self.transform_color_component(my_color.g, color.g, multiplier);
        let new_b = self.transform_color_component(my_color.b, color.b, multiplier);
        RGB::from_u8(new_r, new_g, new_b)
    }
}
