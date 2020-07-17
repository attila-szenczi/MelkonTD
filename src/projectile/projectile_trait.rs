use amethyst::{
  core::transform::Transform,
  ecs::prelude::{Entity, WriteStorage},
};

use crate::minion::Minion;

pub trait ProjectileTrait: Send + Sync {
  fn update<'a>(
    &mut self,
    projectile_entity: Entity,
    minions: &mut WriteStorage<'a, Minion>,
    transforms: &mut WriteStorage<'a, Transform>,
    elapsed: f32,
  );

  fn fire(&mut self);
  fn set_target(&mut self, entity: Entity);

  fn dead(&self) -> bool;
  
  fn sprite_sheet_name(&self) -> &'static str;
  fn transform(&self) -> &Transform;
  fn transform_mut(&mut self) -> &mut Transform;
}
