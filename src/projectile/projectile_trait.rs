use crate::minion::MinionTrait;

use generational_arena::{Arena, Index};

use crate::shared_traits::*;

pub trait ProjectileTrait: DrawableTrait + MortalTrait {
  fn update<'a>(&mut self, minions: &mut Arena<Box<dyn MinionTrait>>, elapsed: f32);

  fn fire(&mut self);
  fn set_target(&mut self, target_id: Index);
}