use amethyst::ecs::prelude::Entity;

use utils::coord::Coord;
use utils::rect::Rect;

use crate::texture_storage::TextureStorage;

pub struct World {
  pub texture_storage: TextureStorage,
}

impl World {
  pub fn new() -> Self {
    World {
      texture_storage: TextureStorage::new(),
    }
  }
}
