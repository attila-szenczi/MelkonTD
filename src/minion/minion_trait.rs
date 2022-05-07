use crate::shared_traits::*;

pub trait MinionTrait: DrawableTrait + MortalTrait + MoveableTrait {
  fn hit(&mut self, damage: i32);

  fn health(&self) -> i32;
  fn max_health(&self) -> i32;
}
