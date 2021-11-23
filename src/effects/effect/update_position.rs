use specs::prelude::*;

use crate::ecs::components::*;
use crate::effects::{Effect, Spawner};

pub fn update_position(ecs: &mut World, spawner: &Spawner, target: Entity) {
    if let Effect::UpdatePosition { new_position } = spawner.effect {
        let mut has_position_storage = ecs.write_storage::<HasPosition>();
        if let Some(has_position) = has_position_storage.get_mut(target) {
            has_position.position.set_from_position(&new_position);
        }
        let mut has_viewshed_storage = ecs.write_storage::<HasViewshed>();
        if let Some(has_viewshed) = has_viewshed_storage.get_mut(target) {
            has_viewshed.viewshed.is_dirty = true;
        }
    }
}
