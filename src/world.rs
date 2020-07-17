use crate::minion::MinionTrait;
use crate::tower::TowerTrait;
use crate::projectile::ProjectileTrait;
use crate::texture_storage::TextureStorage;

pub struct World {
  pub texture_storage: TextureStorage,
  pub minions: Vec<Box<dyn MinionTrait>>,
  pub towers: Vec<Box<dyn TowerTrait>>,
  pub projectiles: Vec<Box<dyn ProjectileTrait>>,
  //pub ui_elements: Vec<Box<dyn ClickableUiElement>>,
}

impl World {
  pub fn new() -> Self {
    World {
      texture_storage: TextureStorage::new(),
      minions: Vec::new(),
      towers: Vec::new(),
      projectiles: Vec::new(),
    }
  }
}
