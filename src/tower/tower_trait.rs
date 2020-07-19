use amethyst::{core::transform::Transform, ecs::prelude::*};

use crate::minion::Minion;
use crate::projectile::Projectile;

use sfml::system::Vector2f;

pub trait TowerTrait: Send + Sync {
  fn update<'a>(
    &mut self,
    entities: &Entities<'a>,
    tower_transform: &Transform,
    minions: &ReadStorage<'a, Minion>,
    transforms: &ReadStorage<'a, Transform>,
    projectiles: &mut WriteStorage<'a, Projectile>,
    updater: &Read<'a, LazyUpdate>,
    elapsed: f32,
  );
  fn sprite_sheet_name(&self) -> &'static str;
  fn position(&self) -> &Vector2f;
  fn position_mut(&mut self) -> &mut Vector2f;
}
