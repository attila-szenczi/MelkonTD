use amethyst::ecs::{Component, DenseVecStorage};

use super::minion_trait::MinionTrait;

use sfml::system::Vector2f;

pub struct TestMinion {
  pub max_health: i32,
  pub health: i32,
  position: Vector2f,
}

impl TestMinion {
  pub fn new() -> Self {
    TestMinion {
      max_health: 20,
      health: 20,
      position: Vector2f::default(),
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
    "private_sprites/5_enemies_1_attack_018.png"
  }

  fn position(&self) -> &Vector2f {
    &self.position
  }

  fn position_mut(&mut self) -> &mut Vector2f {
    &mut self.position
  }
}
