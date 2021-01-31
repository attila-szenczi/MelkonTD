use crate::minion::MinionTrait;
use crate::projectile::ProjectileTrait;

use sfml::system::Vector2f;
use generational_arena::Arena;

pub trait TowerTrait {
  fn update<'a>(&mut self, minions: &mut Arena<Box<dyn MinionTrait>>, projectiles: &mut Arena<Box<dyn ProjectileTrait>>, elapsed: f32);
  fn sprite_sheet_name(&self) -> &'static str;
  fn position(&self) -> &Vector2f;
  fn position_mut(&mut self) -> &mut Vector2f;
}
