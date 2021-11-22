use specs::prelude::*;

use crate::ecs::components::*;

pub struct MeleeCombat {}

impl<'a> System<'a> for MeleeCombat {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, HasMeleeTarget>,
        ReadStorage<'a, HasName>,
        ReadStorage<'a, HasHitPoints>,
        WriteStorage<'a, HasSufferedDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut has_melee_target_storage,
            has_name_storage,
            has_hit_points_storage,
            mut has_suffered_damage_storage,
        ) = data;
        let mut satisfied = vec![];

        for (entity, has_melee_target, has_name, has_hit_points) in (
            &entities,
            &has_melee_target_storage,
            &has_name_storage,
            &has_hit_points_storage,
        )
            .join()
        {
            if has_hit_points.hit_points.current > 0 {
                let target_entity = has_melee_target.melee_target;
                if !entities.is_alive(target_entity) {
                    satisfied.push(entity);
                    continue;
                }
                let target_hit_points = has_hit_points_storage.get(target_entity).unwrap();
                if target_hit_points.hit_points.current > 0 {
                    let target_name = has_name_storage.get(has_melee_target.melee_target).unwrap();
                    let damage = i32::max(0, 10);
                    if damage == 0 {
                        debug!("{} is unable to hurt {}", &has_name.name, &target_name.name);
                    } else {
                        debug!(
                            "{} hits {} for {} hp",
                            &has_name.name, &target_name.name, damage
                        );
                        HasSufferedDamage::new_damage(
                            &mut has_suffered_damage_storage,
                            has_melee_target.melee_target,
                            damage,
                        );
                    }
                }
            }
        }
        for entity in satisfied.iter() {
            debug!(
                "{} is satisfied.",
                has_name_storage.get(*entity).unwrap().name
            );
            has_melee_target_storage.remove(*entity);
        }

        has_melee_target_storage.clear();
    }
}
