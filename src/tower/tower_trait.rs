use crate::minion::MinionTrait;
use crate::projectile::ProjectileTrait;
use crate::shared_traits::*;

use generational_arena::Arena;

pub trait TowerTrait: DrawableTrait + MortalTrait {
  fn update<'a>(
    &mut self,
    minions: &mut Arena<Box<dyn MinionTrait>>,
    projectiles: &mut Arena<Box<dyn ProjectileTrait>>,
    elapsed: f32,
  );
}
