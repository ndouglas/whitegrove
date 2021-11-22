use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::combat::MeleeWeapon;

#[derive(Component, Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct HasMeleeWeapon {
    pub melee_weapon: MeleeWeapon,
}
