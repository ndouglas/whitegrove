use specs::prelude::*;

use super::Spawner;

#[derive(Clone, Debug)]
pub enum Target {
    Tile { index: usize },
    TileList { indices: Vec<usize> },
    Entity { target: Entity },
    EntityList { targets: Vec<Entity> },
}

impl Target {
    pub fn apply(&self, ecs: &mut World, spawner: &Spawner) {
        use Target::*;
        match self {
            Tile { index } => spawner.effect.affect_tile(ecs, spawner, *index),
            TileList { indices } => indices
                .iter()
                .for_each(|index| spawner.effect.affect_tile(ecs, spawner, *index)),
            Entity { target } => spawner.effect.affect_entity(ecs, spawner, *target),
            EntityList { targets } => targets
                .iter()
                .for_each(|entity| spawner.effect.affect_entity(ecs, spawner, *entity)),
        }
    }
}
