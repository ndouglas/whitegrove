use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::map::Viewshed;

#[derive(Component, Clone, Debug, Deserialize, Serialize)]
pub struct HasViewshed {
    pub viewshed: Viewshed,
}
