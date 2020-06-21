use amethyst::ecs::prelude::{Component, DenseVecStorage};

use super::tower_trait::TowerTrait;
use std::ops::{Deref, DerefMut};

pub struct Tower {
  tower: Box<dyn TowerTrait>,
}

impl Tower {
  pub fn new(tower: Box<dyn TowerTrait>) -> Self {
    Tower { tower }
  }
}

impl Component for Tower {
  type Storage = DenseVecStorage<Self>;
}

impl Deref for Tower {
  type Target = Box<dyn TowerTrait>;

  fn deref(&self) -> &Self::Target {
    &self.tower
  }
}

impl DerefMut for Tower {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tower
  }
}
