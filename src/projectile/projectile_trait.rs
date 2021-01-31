use crate::minion::MinionTrait;

use generational_arena::{Arena, Index};

use sfml::system::Vector2f;

pub trait ProjectileTrait {
  fn update<'a>(&mut self, minions: &mut Arena<Box<dyn MinionTrait>>, elapsed: f32);

  fn fire(&mut self);
  fn set_target(&mut self, target_id: Index);

  fn dead(&self) -> bool;
  fn sprite_sheet_name(&self) -> &'static str;
  fn position(&self) -> &Vector2f;
  fn position_mut(&mut self) -> &mut Vector2f;
  fn scale(&self) -> Vector2f;
  fn scale_mut(&mut self) -> &mut Vector2f;
}
