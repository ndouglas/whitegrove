use serde::*;
use specs::prelude::*;

use crate::effects::{enqueue_effect, Effect, Target};
use crate::model::Position;
use crate::spatial_index::MOSS_TILES;

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Moss {
    Seed,
    Moss,
}

impl Moss {
    pub fn count_moss_around_position(&self, position: &Position) -> usize {
        let moss_tiles = MOSS_TILES.lock().unwrap();
        let mut count = 0;
        for adjacent_position in position.get_adjacent_positions() {
            if moss_tiles.get_at_position(&adjacent_position) {
                count += 1;
            }
        }
        count
    }

    pub fn tick(&self, entity: Entity, position: &Position) {
        use self::Moss::*;
        match self {
            Moss => {
                let count = self.count_moss_around_position(position);
                match count {
                    1 | 3 | 5 | 8 => {
                        enqueue_effect(
                            None,
                            Effect::SeedMoss,
                            Target::Positions {
                                positions: position.get_adjacent_positions(),
                            },
                        );
                    }
                    _ => {
                        enqueue_effect(
                            None,
                            Effect::MossLifecycle { next: Seed },
                            Target::Entity { entity },
                        );
                    }
                }
            }
            Seed => {
                let count = self.count_moss_around_position(position);
                match count {
                    2 | 3 | 5 | 7 => {
                        enqueue_effect(
                            None,
                            Effect::MossLifecycle { next: Moss },
                            Target::Entity { entity },
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}
