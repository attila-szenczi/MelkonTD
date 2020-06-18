use amethyst::{
  core::math::Vector3,
  core::transform::Transform,
  ecs::{
    prelude::{Entity, WriteStorage},
    Component, DenseVecStorage,
  },
};

use if_chain::if_chain;

use crate::minion::Minion;
use utils::coord::Vector2;

pub struct Projectile {
  //TODO Boxed dyn trait object to handle targeting (minion, ground etc) and updating (line, spline etc)
  pub target: Option<Entity>,
  pub damage: i32,
  pub detonation_range: f32,
  pub speed: f32,
  pub delete: bool,
  pub fired: bool, //Temp as it's related to a concrete projectile type
  pub max_scale: Vector3<f32>, //Same
  pub increase_scale : bool, //Same
}

impl Component for Projectile {
  type Storage = DenseVecStorage<Self>;
}

impl Projectile {
  pub fn new(
    target: Option<Entity>,
    damage: i32,
    detonation_range: f32,
    speed: f32,
    max_scale: Vector3<f32>,
  ) -> Self {
    Projectile {
      target,
      damage,
      detonation_range,
      speed,
      delete: false,
      fired: false,
      max_scale,
      increase_scale : true,
    }
  }

  pub fn update<'a>(
    &mut self,
    projectile_entity: Entity,
    minions: &mut WriteStorage<'a, Minion>,
    transforms: &mut WriteStorage<'a, Transform>,
    elapsed: f32,
  ) {
    if !self.fired {
      return ();
    }
    //TODO: Can i spare that clone?
    let projectile_transform = transforms.get(projectile_entity).unwrap().clone();
    if_chain! {
        if let Some(target) = self.target;
        if let Some(target_transform) = transforms.get(target);
        then {
            if self.is_in_range(projectile_transform.translation(), target_transform.translation()) {
                let minion =  minions.get_mut(target);
                match minion {
                    Some(minion) => minion.hit(self.damage),
                    _ => ()
                }
                self.delete = true;
            } else {
                let target_translation = target_transform.translation();
                let projectile_translation = projectile_transform.translation();
                //TODO: eliminate intermediate vector
                let mut direction = Vector2::new(target_translation.x - projectile_translation.x,
                                                 target_translation.y - projectile_translation.y);
                direction.normalize();

                //TODO: Stop at target!
                direction.x *= elapsed * self.speed;
                direction.y *= elapsed * self.speed;

                let projectile_transform_mut = transforms.get_mut(projectile_entity).unwrap();
                projectile_transform_mut.set_translation_x(projectile_translation.x + direction.x);
                projectile_transform_mut.set_translation_y(projectile_translation.y + direction.y);
            }
        } else {
            self.delete = true;
        }
    }
  }

  fn is_in_range(&self, lhs: &Vector3<f32>, rhs: &Vector3<f32>) -> bool {
    let y_diff = lhs.y - rhs.y;
    let x_diff = lhs.x - rhs.x;
    let square_sum = y_diff * y_diff + x_diff * x_diff;

    square_sum.sqrt() < self.detonation_range
  }
}
