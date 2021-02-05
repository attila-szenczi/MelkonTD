use if_chain::if_chain;

use generational_arena::Index;

use super::projectile_trait::ProjectileTrait;
use crate::animation::TimedAnimation;
use crate::minion::MinionTrait;
use crate::shared_traits::*;

use generational_arena::Arena;

use utils::coord::Vector2;

use sfml::system::Vector2f;

#[derive(PartialEq)]
enum PulsingState {
  Increase,
  Decrease,
  Dieing, //Target lost
}

pub struct PulsingElectricBall {
  target_index: Option<Index>,
  damage: i32,
  detonation_range: f32,
  speed: f32,
  dead: bool,
  fired: bool,
  pulsing_state: PulsingState,
  last_direction: Option<Vector2>, //TODO: Add trait on top of sf::Vector2f and use it?
  position: Vector2f,
  scale: Vector2f,
  normal_scale: Vector2f,
  animation: TimedAnimation,
}

impl PulsingElectricBall {
  pub fn new(damage: i32, detonation_range: f32, speed: f32) -> Self {
    PulsingElectricBall {
      target_index: None,
      damage,
      detonation_range,
      speed,
      dead: false,
      fired: false,
      pulsing_state: PulsingState::Increase,
      last_direction: None,
      position: Vector2f::new(1., 1.),
      scale: Vector2f::new(0.1, 0.1),
      normal_scale: Vector2f::new(1., 1.),
      animation: TimedAnimation::new(8, 0.05), //TODO: inject it from texture storage
    }
  }

  fn is_in_range(&self, lhs: &Vector2f, rhs: &Vector2f) -> bool {
    let y_diff = lhs.y - rhs.y;
    let x_diff = lhs.x - rhs.x;
    let square_sum = y_diff * y_diff + x_diff * x_diff;
    square_sum.sqrt() < self.detonation_range
  }

  fn pulse(&mut self, elapsed: f32) {
    self.adjust_scale(elapsed);
    self.handle_scale_under_overflow();
    self.scale.y = self.scale.x;
  }

  fn adjust_scale(&mut self, elapsed: f32) {
    let diff = {
      if self.pulsing_state == PulsingState::Increase {
        self.normal_scale.x * elapsed * 0.8
      } else {
        -self.normal_scale.x * elapsed * 0.8
      }
    };

    self.scale.x += diff;
  }

  fn handle_scale_under_overflow(&mut self) {
    if self.scale.x > self.normal_scale.x {
      let overflow = self.scale.x - self.normal_scale.x;
      self.scale.x = self.normal_scale.x - overflow;
      self.pulsing_state = PulsingState::Decrease;
    } else if self.pulsing_state == PulsingState::Decrease
      && self.scale.x < self.normal_scale.x * 0.8
    {
      let underflow = self.normal_scale.x * 0.8 - self.scale.x;
      self.scale.x = self.normal_scale.x * 0.8 + underflow;
      self.pulsing_state = PulsingState::Increase;
    }
  }

  fn hit_minion<'a>(&mut self, minion: &mut Box<dyn MinionTrait>) {
    minion.hit(self.damage);
    self.dead = true;
  }

  fn update_projectile_position(&mut self, direction: &Vector2, elapsed: f32) {
    self.position.x = self.position.x + direction.x * elapsed * self.speed;
    self.position.y = self.position.y + direction.y * elapsed * self.speed;

    self.last_direction = Some(direction.clone());
  }

  fn handle_going_beyond_target<'a>(&mut self, target: &mut Box<dyn MinionTrait>) {
    let mut new_direction = Vector2::new(
      target.position().x - self.position.x,
      target.position().y - self.position.y,
    );

    new_direction.normalize();
    if &new_direction != &self.last_direction.unwrap() {
      self.position.x = target.position().x;
      self.position.y = target.position().y;

      self.hit_minion(target);
    }
  }
}

impl ProjectileTrait for PulsingElectricBall {
  fn update<'a>(&mut self, minions: &mut Arena<Box<dyn MinionTrait>>, elapsed: f32) {
    self.pulse(elapsed);
    self.animation.update(elapsed);

    if !self.fired {
      return ();
    }

    if_chain! {
      if let Some(target_index) = self.target_index;
      if let Some(target) = minions.get_mut(target_index);
      then {
        if self.is_in_range(&self.position, target.position()) {
          self.hit_minion(target);
        } else {
          let mut direction = Vector2::new(
            target.position().x - self.position.x,
            target.position().y - self.position.y,
          );
          direction.normalize();
          self.update_projectile_position(&direction, elapsed);
          self.handle_going_beyond_target(target);
        }
      } else {
        let direction = self.last_direction.unwrap();
        self.update_projectile_position(&direction, elapsed);
        self.pulsing_state = PulsingState::Dieing;
        if self.scale.x < 0.01 {
          self.dead = true;
        }
      }
    }
  }

  fn fire(&mut self) {
    self.fired = true;
  }
  fn set_target(&mut self, target_index: Index) {
    self.target_index = Some(target_index);
  }
}

impl DrawableTrait for PulsingElectricBall {
  fn sprite_sheet_name(&self) -> &'static str {
    "private_sprites/pulsing_electric_ball.png"
  }

  fn position(&self) -> &Vector2f {
    &self.position
  }

  fn position_mut(&mut self) -> &mut Vector2f {
    &mut self.position
  }

  fn scale(&self) -> &Vector2f {
    &self.scale
  }

  fn scale_mut(&mut self) -> &mut Vector2f {
    &mut self.scale
  }

  fn current_frame(&self) -> usize {
    return self.animation.current_frame;
  }
}

impl MortalTrait for PulsingElectricBall {
  fn dead(&self) -> bool {
    return self.dead;
  }
}
