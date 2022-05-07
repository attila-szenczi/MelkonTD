use crate::battle_input_states::ClickableObjectStorage;
use crate::flyout_actions::*;
use crate::minion::MinionTrait;
use crate::projectile::ProjectileTrait;
use crate::texture_storage::TextureStorage;
use crate::tower::TowerTrait;

use generational_arena::Arena;

pub struct World {
  pub texture_storage: TextureStorage,
  pub minions: Arena<Box<dyn MinionTrait>>,
  pub towers: Arena<Box<dyn TowerTrait>>,
  pub projectiles: Arena<Box<dyn ProjectileTrait>>,
  pub clickable_objects: ClickableObjectStorage,
  pub active_flyout_actions: Vec<Box<FlyoutAction>>,
  //pub ui_elements: Vec<Box<dyn ClickableUiElement>>,
}

impl World {
  pub fn new() -> Self {
    World {
      texture_storage: TextureStorage::new(),
      minions: Arena::new(),
      towers: Arena::new(),
      projectiles: Arena::new(),
      clickable_objects: ClickableObjectStorage::new(),
      active_flyout_actions: Vec::new(),
    }
  }
}
