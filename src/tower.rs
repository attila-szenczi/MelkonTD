use amethyst::{
  core::math::Vector3,
  core::transform::Transform,
  ecs::{prelude::*, Component, DenseVecStorage},
  renderer::SpriteRender,
};

use if_chain::if_chain;

use crate::minion::Minion;
use crate::projectile::Projectile;
use crate::simple_animation::SimpleAnimation;
use crate::z_layer::{z_layer_to_coordinate, ZLayer};

pub struct Tower {
  //TODO Boxed dyn trait object to handle firing, evolution options etc
  pub target: Option<Entity>,
  pub damage: i32,
  pub firing_timer: f32,
  pub range: f32,
  sprite_render: SpriteRender,
  sprite_scale: Vector3<f32>,
  charging_projectile: Option<Entity>, //Temp
}

impl Component for Tower {
  type Storage = DenseVecStorage<Self>;
}

impl Tower {
  pub fn new(sprite_render: SpriteRender, sprite_scale: Vector3<f32>) -> Self {
    Tower {
      target: None,
      damage: 10,
      firing_timer: 1.,
      range: 70.,
      sprite_render,
      sprite_scale,
      charging_projectile: None,
    }
  }

  pub fn update<'a>(
    &mut self,
    entities: &Entities<'a>,
    tower_transform: &Transform,
    minions: &ReadStorage<'a, Minion>,
    transforms: &ReadStorage<'a, Transform>,
    projectiles: &mut WriteStorage<'a, Projectile>,
    updater: &Read<'a, LazyUpdate>,
    elapsed: f32,
  ) {
    if self.update_timer(elapsed, entities, updater, tower_transform.translation()) {
      if_chain! {
          if let Some(entity) = self.target;
          if let Some(target_transform) = transforms.get(entity);
          if self.is_in_range(tower_transform.translation(), target_transform.translation());
          then {
              self.fire(entities, projectiles);
          } else {
              //TODO: Lookup instead of entities join?
              for (entity, _minion, transform) in (entities, minions, transforms).join() {
                  let tower_translation = tower_transform.translation();
                  if self.is_in_range(tower_translation, transform.translation()) {
                      self.target = Some(entity);
                      self.fire(entities, projectiles);

                      break;
                  }
              }
          }
      }
    }
  }

  fn fire<'a>(&mut self, entities: &Entities<'a>, projectiles: &mut WriteStorage<'a, Projectile>) {
    if let Some(charging_projectile_entity) = self.charging_projectile {
      if let Some(mut projectile) = projectiles.get_mut(charging_projectile_entity) {
        projectile.fired = true;
        projectile.target = self.target;
      }
    }
    self.charging_projectile = None;
    self.reset_timer();
  }

  fn charge_projectile<'a>(
    &mut self,
    entities: &Entities<'a>,
    updater: &Read<'a, LazyUpdate>,
    tower_translation: &Vector3<f32>,
  ) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
      tower_translation.x as f32,
      tower_translation.y as f32 + 20.,
      z_layer_to_coordinate(ZLayer::Projectile),
    );
    transform.set_scale(Vector3::new(
      self.sprite_scale.x / 10. * 1.1,
      self.sprite_scale.y / 10. * 1.1,
      self.sprite_scale.z,
    ));
    self.charging_projectile = Some(
      updater
        .create_entity(&entities)
        .with(self.sprite_render.clone())
        .with(transform)
        .with(Projectile::new(None, 10, 5., 150., self.sprite_scale))
        .with(SimpleAnimation::new(0, 8, 0.05))
        .build(),
    );
  }

  fn update_timer<'a>(
    &mut self,
    elapsed: f32,
    entities: &Entities<'a>,
    updater: &Read<'a, LazyUpdate>,
    tower_translation: &Vector3<f32>,
  ) -> bool {
    if self.firing_timer > 0. {
      self.firing_timer -= elapsed;
      if self.firing_timer < 0.8 && self.charging_projectile == None {
        self.charge_projectile(entities, updater, tower_translation);
      }
    } else {
      self.firing_timer = 0.;
    }
    self.firing_timer <= 0.
  }

  fn reset_timer(&mut self) {
    self.firing_timer += 1.;
  }

  fn is_in_range(&self, lhs: &Vector3<f32>, rhs: &Vector3<f32>) -> bool {
    let y_diff = lhs.y - rhs.y;
    let x_diff = lhs.x - rhs.x;
    let square_sum = y_diff * y_diff + x_diff * x_diff;
    square_sum.sqrt() < self.range
  }
}
