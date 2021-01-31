use amethyst::{
  core::math::Vector3,
  core::transform::Transform,
  ecs::prelude::{Entity, WriteStorage},
};

use std::cell::RefCell;
use std::rc::Weak;

use if_chain::if_chain;

use super::projectile_trait::ProjectileTrait;
use crate::minion::{Minion, MinionTrait, TestMinion};
use utils::coord::Vector2;

use sfml::system::Vector2f;

#[derive(PartialEq)]
enum PulsingState {
  Increase,
  Decrease,
  Dieing, //Target lost
}

pub struct PulsingElectricBall {
  target: Weak<RefCell<dyn MinionTrait>>,
  damage: i32,
  detonation_range: f32,
  speed: f32,
  delete: bool,
  fired: bool,
  pulsing_state: PulsingState,
  last_direction: Option<Vector2>,
  position: Vector2f,
  scale_multiplier: Vector2f,
}

impl PulsingElectricBall {
  pub fn new(damage: i32, detonation_range: f32, speed: f32, scale_multiplier: Vector2f) -> Self {
    let empty: Weak<RefCell<dyn MinionTrait>> = Weak::<RefCell<TestMinion>>::new();
    PulsingElectricBall {
      target: empty,
      damage,
      detonation_range,
      speed,
      delete: false,
      fired: false,
      pulsing_state: PulsingState::Increase,
      last_direction: None,
      position: Vector2f::new(1., 1.),
      scale_multiplier,
    }
  }

  fn is_in_range(&self, lhs: &Vector3<f32>, rhs: &Vector3<f32>) -> bool {
    let y_diff = lhs.y - rhs.y;
    let x_diff = lhs.x - rhs.x;
    let square_sum = y_diff * y_diff + x_diff * x_diff;
    square_sum.sqrt() < self.detonation_range
  }

  fn pulse(&mut self, scale: &mut Vector3<f32>, elapsed: f32) {
    self.adjust_scale(scale, elapsed);
    self.handle_scale_under_overflow(scale);
    scale.y = scale.x;
  }

  fn adjust_scale(&mut self, scale: &mut Vector3<f32>, elapsed: f32) {
    let diff = {
      if self.pulsing_state == PulsingState::Increase {
        //  self.normal_scale.x * elapsed
      } else {
        //  -self.normal_scale.x * elapsed
      }
    };

    // scale.x += diff;
  }

  fn handle_scale_under_overflow(&mut self, scale: &mut Vector3<f32>) {
    // if scale.x > self.normal_scale.x {
    //   let overflow = scale.x - self.normal_scale.x;
    //   scale.x = self.normal_scale.x - overflow;
    //   self.pulsing_state = PulsingState::Decrease;
    // } else if self.pulsing_state == PulsingState::Decrease && scale.x < self.normal_scale.x * 0.8 {
    //   let underflow = self.normal_scale.x * 0.8 - scale.x;
    //   scale.x = self.normal_scale.x * 0.8 + underflow;
    //   self.pulsing_state = PulsingState::Increase;
    // }
  }

  fn hit_minion<'a>(&mut self, minions: &mut WriteStorage<'a, Minion>, target: Entity) {
    let minion = minions.get_mut(target);
    match minion {
      Some(minion) => minion.hit(self.damage),
      _ => (),
    }
    self.delete = true;
  }

  fn update_projectile_translation(
    &mut self,
    projectile_transform: &mut Transform,
    direction: &Vector2,
    elapsed: f32,
  ) {
    let projectile_translation = projectile_transform.translation().clone();
    projectile_transform
      .set_translation_x(projectile_translation.x + direction.x * elapsed * self.speed);
    projectile_transform
      .set_translation_y(projectile_translation.y + direction.y * elapsed * self.speed);

    self.last_direction = Some(direction.clone());
  }

  fn handle_going_beyond_target<'a>(
    &mut self,
    projectile_transform: &mut Transform,
    target_translation: &Vector3<f32>,
    direction: &Vector2,
    minions: &mut WriteStorage<'a, Minion>,
    target: Entity,
  ) {
    let mut new_direction = Vector2::new(
      target_translation.x - projectile_transform.translation().x,
      target_translation.y - projectile_transform.translation().y,
    );

    new_direction.normalize();
    if &new_direction != direction {
      projectile_transform.set_translation_x(target_translation.x);
      projectile_transform.set_translation_y(target_translation.y);

      self.hit_minion(minions, target);
    }
  }
}

impl ProjectileTrait for PulsingElectricBall {
  fn update<'a>(
    &mut self,
    projectile_entity: Entity,
    minions: &mut WriteStorage<'a, Minion>,
    transforms: &mut WriteStorage<'a, Transform>,
    elapsed: f32,
  ) {
    self.pulse(
      transforms.get_mut(projectile_entity).unwrap().scale_mut(),
      elapsed,
    );

    if !self.fired {
      return ();
    }
    let projectile_transform = transforms.get(projectile_entity).unwrap();
    //if_chain! {
    // if let Some(target) = self.target;
    // if let Some(target_transform) = transforms.get(target);
    // then {
    //     if self.is_in_range(projectile_transform.translation(), target_transform.translation()) {
    //       self.hit_minion(minions, target);
    //     } else {
    //         let target_translation = target_transform.translation().clone();
    //         let projectile_translation = projectile_transform.translation();
    //         let mut direction = Vector2::new(target_translation.x - projectile_translation.x,
    //                                          target_translation.y - projectile_translation.y);
    //         direction.normalize();
    //         let projectile_transform_mut = transforms.get_mut(projectile_entity).unwrap();
    //         self.update_projectile_translation(projectile_transform_mut, &direction, elapsed);

    //         self.handle_going_beyond_target(projectile_transform_mut, &target_translation, &direction, minions, target);
    //     }
    // } else {
    //   let projectile_transform = transforms.get_mut(projectile_entity).unwrap();
    //   let direction = self.last_direction.unwrap();

    //     self.update_projectile_translation(projectile_transform, &direction, elapsed);
    //     self.pulsing_state = PulsingState::Dieing;
    //     if projectile_transform.scale().x < 0.01 {
    //       self.delete = true;
    //     }
    // }
    //}
  }

  fn dead(&self) -> bool {
    return self.delete;
  }

  fn fire(&mut self) {
    self.fired = true;
  }
  fn set_target(&mut self, entity: Entity) {
    // self.target = Some(entity);
  }

  fn sprite_sheet_name(&self) -> &'static str {
    "private_sprites/pulsing_electric_ball.png"
  }

  fn position(&self) -> &Vector2f {
    &self.position
  }

  fn position_mut(&mut self) -> &mut Vector2f {
    &mut self.position
  }

  fn scale_multiplier(&self) -> Vector2f {
    self.scale_multiplier
  }

  fn scale_multiplier_mut(&mut self) -> &mut Vector2f {
    &mut self.scale_multiplier
  }
}
