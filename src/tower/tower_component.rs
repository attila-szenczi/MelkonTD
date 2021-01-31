// use amethyst::ecs::prelude::{Component, DenseVecStorage};

// use super::tower_trait::TowerTrait;
// use super::tower_type::TowerType;
// use std::ops::{Deref, DerefMut};

// pub struct Tower {
//   tower: Box<dyn TowerTrait>,
//   pub tower_type: TowerType,
// }

// impl Tower {
//   pub fn new(tower: Box<dyn TowerTrait>, tower_type: TowerType) -> Self {
//     Tower { tower, tower_type }
//   }
// }

// impl Component for Tower {
//   type Storage = DenseVecStorage<Self>;
// }

// impl Deref for Tower {
//   type Target = Box<dyn TowerTrait>;

//   fn deref(&self) -> &Self::Target {
//     &self.tower
//   }
// }

// impl DerefMut for Tower {
//   fn deref_mut(&mut self) -> &mut Self::Target {
//     &mut self.tower
//   }
// }
