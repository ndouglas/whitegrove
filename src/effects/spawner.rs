use specs::prelude::*;

use super::Effect;
use super::Target;

#[derive(Clone, Debug)]
pub struct Spawner {
    pub creator: Option<Entity>,
    pub effect: Effect,
    pub target: Target,
}
