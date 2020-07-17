use amethyst::ecs::{Component, DenseVecStorage};

use super::minion_trait::MinionTrait;

use sfml::graphics::Transform;

pub struct TestMinion {
  pub max_health: i32,
  pub health: i32,
  transform: Transform,
}

impl TestMinion {
  pub fn new() -> Self {
    TestMinion {
      max_health: 20,
      health: 20,
      transform: Transform::default(),
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
  fn dead(&self) -> bool {
    self.health <= 0
  }

  fn sprite_sheet_name(&self) -> &'static str {
    "private_sprites/5_enemies_1_attack_018"
  }

  fn transform(&self) -> &Transform {
    &self.transform
  }

  fn transform_mut(&mut self) -> &mut Transform {
    &mut self.transform
  }
}
