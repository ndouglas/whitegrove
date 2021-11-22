use serde::*;

use crate::dice::Dice;

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct MeleeWeapon {
    pub damage_dice: Dice,
}
