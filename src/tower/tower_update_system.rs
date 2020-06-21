use amethyst::{
  core::timing::Time,
  core::transform::Transform,
  derive::SystemDesc,
  ecs::{
    prelude::*,
    prelude::{Read, System},
  },
};

use super::tower_component::Tower;
use crate::minion::Minion;
use crate::projectile::Projectile;

#[derive(SystemDesc, Default)]
pub struct TowerUpdateSystem;

impl<'a> System<'a> for TowerUpdateSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, Tower>,
    ReadStorage<'a, Minion>,
    ReadStorage<'a, Transform>,
    WriteStorage<'a, Projectile>,
    Read<'a, LazyUpdate>,
    Read<'a, Time>,
  );

  fn run(
    &mut self,
    (entities, mut towers, minions, transforms, mut projectiles, updater, time): Self::SystemData,
  ) {
    for (tower, transform) in (&mut towers, &transforms).join() {
      tower.update(
        &entities,
        transform,
        &minions,
        &transforms,
        &mut projectiles,
        &updater,
        time.delta_seconds(),
      );
    }
  }
}
