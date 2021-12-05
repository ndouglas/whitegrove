use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::lifecycle::Moss as MossLifecycle;

#[derive(Component, Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct HasMossLifecycle {
    pub moss_lifecycle: MossLifecycle,
}
