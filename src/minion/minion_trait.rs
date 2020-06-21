pub trait MinionTrait: Send + Sync {
  fn hit(&mut self, damage: i32);

  fn health(&self) -> i32;
  fn max_health(&self) -> i32;
  fn dead(&self) -> bool;
}
