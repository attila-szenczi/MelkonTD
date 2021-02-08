use amethyst::ecs::{Component, DenseVecStorage};

use super::minion_trait::MinionTrait;

use crate::shared_traits::*;

use sfml::system::Vector2f;

pub struct TestMinion {
  max_health: i32,
  health: i32,
  position: Vector2f,
  scale: Vector2f,
}

impl TestMinion {
  pub fn new(position: Vector2f) -> Self {
    TestMinion {
      max_health: 20,
      health: 20,
      position,
      scale: Vector2f::new(1., 1.),
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
}

impl DrawableTrait for TestMinion {
  fn sprite_sheet_name(&self) -> &'static str {
    "private_sprites/5_enemies_1_attack_018.png"
  }

  fn position(&self) -> &Vector2f {
    &self.position
  }

  fn scale(&self) -> &Vector2f {
    &self.scale
  }

  fn current_frame(&self) -> usize {
    return 0;
  }
}

impl MortalTrait for TestMinion {
  fn dead(&self) -> bool {
    self.health <= 0
  }
}

impl MoveableTrait for TestMinion {
  fn position_mut(&mut self) -> &mut Vector2f {
    &mut self.position
  }

  fn scale_mut(&mut self) -> &mut Vector2f {
    &mut self.scale
  }
}
