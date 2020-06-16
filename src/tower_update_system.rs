use amethyst::{
  core::timing::Time,
  core::transform::Transform,
  derive::SystemDesc,
  ecs::{
    prelude::*,
    prelude::{Read, System},
  },
};

use crate::minion::Minion;
use crate::tower::Tower;

#[derive(SystemDesc, Default)]
pub struct TowerUpdateSystem;

impl<'a> System<'a> for TowerUpdateSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, Tower>,
    ReadStorage<'a, Minion>,
    ReadStorage<'a, Transform>,
    Read<'a, LazyUpdate>,
    Read<'a, Time>,
  );

  fn run(&mut self, (entities, mut towers, minions, transforms, updater, time): Self::SystemData) {
    for (tower, transform) in (&mut towers, &transforms).join() {
      tower.update(
        &entities,
        transform,
        &minions,
        &transforms,
        &updater,
        time.delta_seconds(),
      );
    }
  }
}
