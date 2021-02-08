use crate::animation::LinearScalePositionTransition;
use crate::shared_traits::*;

use crate::world::World;

use sfml::system::Vector2f;

pub struct FlyoutAction {
  transition: LinearScalePositionTransition,
  image_name: String,
  dead: bool,
}

impl FlyoutAction {
  pub fn new(transition: LinearScalePositionTransition, image_name: String) -> Self {
    FlyoutAction {
      transition,
      image_name,
      dead: false,
    }
  }

  pub fn execute(&mut self, _world: &mut World) {
    self.dead = true;
  }

  pub fn update(&mut self, elapsed: f32) {
    self.transition.update(elapsed);
  }

  pub fn image_name(&self) -> &str {
    &self.image_name
  }

  pub fn position(&self) -> &Vector2f {
    &self.transition.position_current
  }

  pub fn scale(&self) -> &Vector2f {
    &self.transition.scale_current
  }
}

impl MortalTrait for FlyoutAction {
  fn dead(&self) -> bool {
    self.dead
  }
}
