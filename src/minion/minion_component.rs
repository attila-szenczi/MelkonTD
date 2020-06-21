use amethyst::ecs::prelude::{Component, DenseVecStorage};

use super::minion_trait::MinionTrait;
use std::ops::{Deref, DerefMut};

pub struct Minion {
  minion: Box<dyn MinionTrait>,
}

impl Minion {
  pub fn new(minion: Box<dyn MinionTrait>) -> Self {
    Minion { minion }
  }
}

impl Component for Minion {
  type Storage = DenseVecStorage<Self>;
}

impl Deref for Minion {
  type Target = Box<dyn MinionTrait>;

  fn deref(&self) -> &Self::Target {
    &self.minion
  }
}

impl DerefMut for Minion {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.minion
  }
}
