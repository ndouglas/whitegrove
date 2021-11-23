use specs::prelude::*;
use specs_derive::Component;

use crate::light::Source as LightSource;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct HasLightSource {
    pub light_source: LightSource,
}
