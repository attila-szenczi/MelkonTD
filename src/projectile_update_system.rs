use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    core::transform::Transform,
    ecs::{
        prelude::*,
        prelude::{Read, System},
    },
};

use crate::minion::Minion;
use crate::projectile::Projectile;

#[derive(SystemDesc)]
pub struct ProjectileUpdateSystem;

impl<'a> System<'a> for ProjectileUpdateSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Projectile>,
        WriteStorage<'a, Minion>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (entities, mut projectiles, mut minions, mut transforms, time): Self::SystemData) {
       for (entity, projectile) in (&entities, &mut projectiles).join() {
            projectile.update(entity, &mut minions, &mut transforms, time.delta_seconds());
       }
    }
}