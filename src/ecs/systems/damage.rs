use specs::prelude::*;

use crate::ecs::components::*;

pub struct Damage {}

impl<'a> System<'a> for Damage {
    type SystemData = (
        WriteStorage<'a, HasHitPoints>,
        WriteStorage<'a, HasSufferedDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut has_hit_points_storage, mut has_suffered_damage_storage) = data;
        for (mut has_hit_points, has_suffered_damage) in
            (&mut has_hit_points_storage, &has_suffered_damage_storage).join()
        {
            has_hit_points.hit_points.current -= has_suffered_damage.get_total();
        }
        has_suffered_damage_storage.clear();
    }
}
