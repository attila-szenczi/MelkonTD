use amethyst::ecs::{Component, DenseVecStorage};

use super::minion_trait::MinionTrait;

pub struct TestMinion {
  pub max_health: i32,
  pub health: i32,
}

impl TestMinion {
  pub fn new() -> Self {
    TestMinion {
      max_health: 20,
      health: 20,
    }
  }
}

impl Component for TestMinion {
  type Storage = DenseVecStorage<Self>;
}

impl MinionTrait for TestMinion {
  fn hit(&mut self, damage: i32) {
    self.health -= damage;
  }

  fn health(&self) -> i32 {
    self.health
  }

  fn max_health(&self) -> i32 {
    self.max_health
  }
  fn dead(&self) -> bool
  {
    self.health <= 0
  }
}
