use crate::animation::LinearScalePositionTransition;

use crate::world::World;

use sfml::graphics::IntRect;
use sfml::system::Vector2f;

pub struct FlyoutAction {
  transition: LinearScalePositionTransition,
  image_name: String,
  full_width: i32,
  pub rect: IntRect,
  full_height: i32,
}

impl FlyoutAction {
  pub fn new(
    transition: LinearScalePositionTransition,
    image_name: String,
    full_width: i32,
    full_height: i32,
  ) -> Self {
    FlyoutAction {
      transition,
      image_name,
      full_width,
      full_height,
      rect: IntRect::new(0, 0, 0, 0),
    }
  }

  pub fn execute(&mut self, _world: &mut World) {}

  pub fn update(&mut self, elapsed: f32) {
    if self.transition.update(elapsed) {
      //Note: Can be 2 px off cause of rounding
      let percentage_multiplier = self.transition.transition_percentage_multiplier();
      self.rect.left = (self.transition.position_current.x
        - self.full_width as f32 * percentage_multiplier / 2.) as i32;
      self.rect.width = (self.full_width as f32 * percentage_multiplier) as i32;

      self.rect.top = (self.transition.position_current.y
        - self.full_height as f32 * percentage_multiplier / 2.) as i32;
      self.rect.height = (self.full_height as f32 * percentage_multiplier) as i32;
    }
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
