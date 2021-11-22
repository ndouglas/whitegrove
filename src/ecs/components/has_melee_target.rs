use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug, Hash, PartialEq)]
pub struct HasMeleeTarget {
    pub melee_target: Entity,
}
