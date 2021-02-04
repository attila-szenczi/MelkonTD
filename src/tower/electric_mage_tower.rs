use if_chain::if_chain;

use super::tower_trait::TowerTrait;

use crate::minion::MinionTrait;
use crate::projectile::{ProjectileTrait, PulsingElectricBall};

use crate::shared_traits::*;

use generational_arena::{Arena, Index};

use sfml::system::Vector2f;

pub struct ElectricMageTower {
  pub target_index: Option<Index>,
  pub damage: i32,
  pub firing_timer: f32,
  pub range: f32,
  charging_projectile_index: Option<Index>,
  position: Vector2f,
  scale: Vector2f,
  delete: bool,
}

impl ElectricMageTower {
  pub fn new(position: Vector2f) -> Self {
    ElectricMageTower {
      target_index: None,
      damage: 10,
      firing_timer: 1.,
      range: 150.,
      charging_projectile_index: None,
      position,
      scale: Vector2f::new(1., 1.),
      delete: false,
    }
  }

  fn fire<'a>(&mut self, projectiles: &mut Arena<Box<dyn ProjectileTrait>>) {
    if let Some(charging_projectile_index) = self.charging_projectile_index {
      if let Some(projectile) = projectiles.get_mut(charging_projectile_index) {
        projectile.fire();
        projectile.set_target(self.target_index.unwrap());
        self.charging_projectile_index = None;
        self.reset_timer();
      }
    }
  }

  fn charge_projectile<'a>(&mut self, projectiles: &mut Arena<Box<dyn ProjectileTrait>>) {
    //TODO: Scale?
    let mut projectile = Box::new(PulsingElectricBall::new(
      10,
      5.,
      150.,
      Vector2f::new(1., 1.),
    ));
    let position = projectile.position_mut();
    position.x = self.position.x - 10.;
    position.y = self.position.y - 80.;

    self.charging_projectile_index = Some(projectiles.insert(projectile));
  }

  fn update_timer<'a>(
    &mut self,
    elapsed: f32,
    projectiles: &mut Arena<Box<dyn ProjectileTrait>>,
  ) -> bool {
    if self.firing_timer > 0. {
      self.firing_timer -= elapsed;
      if self.firing_timer < 0.8 && self.charging_projectile_index == None {
        self.charge_projectile(projectiles);
      }
    } else {
      self.firing_timer = 0.;
    }
    self.firing_timer <= 0.
  }

  fn reset_timer(&mut self) {
    self.firing_timer += 1.;
  }

  fn is_in_range(&self, lhs: &Vector2f, rhs: &Vector2f) -> bool {
    let y_diff = lhs.y - rhs.y;
    let x_diff = lhs.x - rhs.x;
    let square_sum = y_diff * y_diff + x_diff * x_diff;
    square_sum.sqrt() < self.range
  }
}

impl TowerTrait for ElectricMageTower {
  fn update<'a>(
    &mut self,
    minions: &mut Arena<Box<dyn MinionTrait>>,
    projectiles: &mut Arena<Box<dyn ProjectileTrait>>,
    elapsed: f32,
  ) {
    if self.update_timer(elapsed, projectiles) {
      if_chain! {
          if let Some(target_index) = self.target_index;
          if let Some(target_minion) = minions.get(target_index);
          if self.is_in_range(&self.position, target_minion.position());
          then {
              self.fire(projectiles);
          } else {
            for (index, minion) in minions {
              //println!("positions: {} {} {} {}", self.position.x, self.position.y,
               //minion.position().x, minion.position().y);
              if self.is_in_range(&self.position, minion.position()) {
                self.target_index = Some(index);
                self.fire(projectiles);

                break;
              }
            }
          }
      }
    }
  }
}

impl DrawableTrait for ElectricMageTower {
  fn sprite_sheet_name(&self) -> &'static str {
    "private_sprites/electric_tower.png"
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
}

impl MortalTrait for ElectricMageTower {
  fn dead(&self) -> bool {
    return self.delete;
  }
}
