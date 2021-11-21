use serde::*;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Clone, Debug, Deserialize, Serialize)]
pub struct HasName {
    pub name: String,
}
