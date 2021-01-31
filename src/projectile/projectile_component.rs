// use amethyst::ecs::prelude::{Component, DenseVecStorage};

// use super::projectile_trait::ProjectileTrait;
// use std::ops::{Deref, DerefMut};

// pub struct Projectile {
//   projectile: Box<dyn ProjectileTrait>,
// }

// impl Projectile {
//   pub fn new(projectile: Box<dyn ProjectileTrait>) -> Self {
//     Projectile { projectile }
//   }
// }

// impl Component for Projectile {
//   type Storage = DenseVecStorage<Self>;
// }

// impl Deref for Projectile {
//   type Target = Box<dyn ProjectileTrait>;

//   fn deref(&self) -> &Self::Target {
//     &self.projectile
//   }
// }

// impl DerefMut for Projectile {
//   fn deref_mut(&mut self) -> &mut Self::Target {
//     &mut self.projectile
//   }
// }
