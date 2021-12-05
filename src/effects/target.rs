use specs::prelude::*;

use crate::model::Position;

use super::Spawner;

#[derive(Clone, Debug)]
pub enum Target {
    Position { position: Position },
    Positions { positions: Vec<Position> },
    Entity { entity: Entity },
    Entities { entities: Vec<Entity> },
}

impl Target {
    pub fn apply(&self, ecs: &mut World, spawner: &Spawner) {
        use Target::*;
        match self {
            Position { position } => spawner.effect.affect_tile(ecs, spawner, position),
            Positions { positions } => positions
                .iter()
                .for_each(|position| spawner.effect.affect_tile(ecs, spawner, position)),
            Entity { entity } => spawner.effect.affect_entity(ecs, spawner, *entity),
            Entities { entities } => entities
                .iter()
                .for_each(|entity| spawner.effect.affect_entity(ecs, spawner, *entity)),
        }
    }
}
