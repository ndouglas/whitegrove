use specs::prelude::*;

use crate::combat::*;
use crate::dice::Dice;
use crate::ecs::components::*;
use crate::ecs::resources::*;
use crate::lifecycle::Moss as MossLifecycle;
use crate::light::Factory as LightSourceFactory;
use crate::map::*;
use crate::model::*;
use crate::particle::Builder as ParticleBuilder;
use crate::random;
use crate::render::Factory as RenderableFactory;

pub fn inject_player(ecs: &mut World, (x, y): (usize, usize), (width, height): (usize, usize)) {
    let player = ecs
        .create_entity()
        .with(HasPosition {
            position: Position::from_xy((x, y), (width, height)),
        })
        .with(HasRenderable {
            renderable: RenderableFactory::Player.create(),
        })
        .with(HasViewshed {
            viewshed: Viewshed::new(15),
        })
        .with(IsPlayer {})
        .with(OccupiesTile {})
        .with(HasHitPoints {
            hit_points: HitPoints::new(12800000),
        })
        .with(HasLightSource {
            light_source: LightSourceFactory::Torch.create(),
        })
        .with(HasName {
            name: "Player".to_string(),
        })
        .build();
    ecs.insert(player);
}

pub fn inject_mobs(
    ecs: &mut World,
    rooms: &Vec<Rectangle>,
    count: usize,
    (width, height): (usize, usize),
) {
    for i in 0..count {
        let room = rooms[random::range(0, rooms.len())];
        let (spawn_x, spawn_y) = room.get_center_xy();
        let roll = random::roll_dice(1, 3);
        let renderable;
        let name;
        match roll {
            1 => {
                renderable = RenderableFactory::Orc.create();
                name = "Orc";
            }
            2 => {
                renderable = RenderableFactory::Goblin.create();
                name = "Goblin";
            }
            _ => {
                renderable = RenderableFactory::Idiot.create();
                name = "Idiot";
            }
        }
        ecs.create_entity()
            .with(HasPosition {
                position: Position::from_xy((spawn_x, spawn_y), (width, height)),
            })
            .with(HasRenderable {
                renderable: renderable,
            })
            .with(HasViewshed {
                viewshed: Viewshed::new(15),
            })
            .with(WantsToMove::Randomly { duration: 2 })
            .with(OccupiesTile {})
            .with(HasHitPoints {
                hit_points: HitPoints::new(32),
            })
            .with(HasMeleeWeapon {
                melee_weapon: MeleeWeapon {
                    damage_dice: Dice {
                        number: 1,
                        sides: 8,
                        bonus: 1,
                    },
                },
            })
                        .with(HasLightSource {
                            light_source: LightSourceFactory::Random.create(),
                        })
            .with(HasName {
                name: format!("{} #{}", name, i),
            })
            .build();
    }
}

pub fn inject_moss(ecs: &mut World, map: &Map, count: usize, (width, height): (usize, usize)) {
    let rooms = &map.rooms;
    let mut created = 0;
    loop {
        let room = rooms[random::range(0, rooms.len())];
        let (spawn_x, spawn_y) = room.get_random_xy();
        if !map.is_exit_valid_xy((spawn_x, spawn_y)) {
            continue;
        }
        ecs.create_entity()
            .with(HasPosition {
                position: Position::from_xy((spawn_x, spawn_y), (width, height)),
            })
            .with(HasRenderable {
                renderable: RenderableFactory::Moss.create(),
            })
            .with(HasLightSource {
                light_source: LightSourceFactory::Moss.create(),
            })
            .with(HasMossLifecycle {
                moss_lifecycle: MossLifecycle::Moss,
            })
            .with(HasViewshed {
                viewshed: Viewshed::new(5),
            })
            .with(HasName {
                name: format!("Moss"),
            })
            .build();
        created += 1;
        if created == count {
            return;
        }
    }
}

pub fn inject_moss_seeds(
    ecs: &mut World,
    map: &Map,
    count: usize,
    (width, height): (usize, usize),
) {
    let rooms = &map.rooms;
    let mut created = 0;
    loop {
        let room = rooms[random::range(0, rooms.len())];
        let (spawn_x, spawn_y) = room.get_random_xy();
        if !map.is_exit_valid_xy((spawn_x, spawn_y)) {
            continue;
        }
        ecs.create_entity()
            .with(HasPosition {
                position: Position::from_xy((spawn_x, spawn_y), (width, height)),
            })
            .with(HasMossLifecycle {
                moss_lifecycle: MossLifecycle::Seed,
            })
            .with(HasName {
                name: format!("Moss"),
            })
            .build();
        created += 1;
        if created == count {
            return;
        }
    }
}

pub fn initialize_world(ecs: &mut World, width: usize, height: usize) {
    let map = Map::new(width, height);
    let (spawn_x, spawn_y) = map.rooms[0].get_center_xy();
    inject_player(ecs, (spawn_x, spawn_y), (width, height));
    inject_mobs(ecs, &map.rooms, 5, (width, height));
    inject_moss(ecs, &map, 100, (width, height));
    inject_moss_seeds(ecs, &map, 15, (width, height));
    ecs.insert(map);
    ecs.insert(Tick(0));
    ecs.insert(ParticleBuilder::new());
}
