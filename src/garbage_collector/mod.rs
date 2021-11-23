use rltk::Rltk;
use specs::prelude::*;

use crate::combat::garbage_collector::collect_garbage as combat_garbage_collector;
use crate::particle::garbage_collector::collect_garbage as particle_garbage_collector;

pub fn collect_garbage(ecs: &mut World, ctx: &Rltk) {
    combat_garbage_collector(ecs, ctx);
    particle_garbage_collector(ecs, ctx);
}
