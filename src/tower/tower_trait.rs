use crate::world::World;

use sfml::system::Vector2f;

pub trait TowerTrait {
  fn update<'a>(&mut self, world: &mut World, elapsed: f32);
  fn sprite_sheet_name(&self) -> &'static str;
  fn position(&self) -> &Vector2f;
  fn position_mut(&mut self) -> &mut Vector2f;
}
