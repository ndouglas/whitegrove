use rltk::RGB;

use crate::random::range as random_range;

use super::Source;

#[derive(Clone, Copy, Debug)]
pub enum Factory {
    /// A candle provides very little light.
    Candle,
    /// A torch provides more and stronger light.
    Torch,
    /// A patch of phosphorescent moss.
    Moss,
    /// A completely random light source.
    Random,
}

/// A factory.
impl Factory {
    /// Creates a light source.
    pub fn create(self) -> Source {
        use Factory::*;
        match self {
            Candle => Source::new(RGB::from_u8(255, 127, 0), 6, 64),
            Torch => Source::new(RGB::from_u8(255, 127, 0), 10, 128),
            Moss => Source::new(RGB::from_u8(173, 223, 173), 5, 32),
            Random => Source::new(
                RGB::from_u8(
                    random_range(0, 5) as u8 * 60,
                    random_range(0, 5) as u8 * 60,
                    random_range(0, 5) as u8 * 60,
                ),
                random_range(4, 12),
                random_range(32, 96),
            ),
        }
    }
}
