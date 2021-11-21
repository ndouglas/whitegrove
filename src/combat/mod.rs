use serde::*;

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct HitPoints {
    pub max: i32,
    pub current: i32,
}

impl HitPoints {
    pub fn new(hp: i32) -> Self {
        HitPoints {
            max: hp,
            current: hp,
        }
    }
}
