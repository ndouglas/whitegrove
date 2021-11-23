use serde::*;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Lifetime {
    pub lifetime: f32,
}
