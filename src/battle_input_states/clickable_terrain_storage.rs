use sfml::graphics::IntRect;

use generational_arena::Index;

pub enum ClickableTerrainRectType {
  Unusable,
  Slot,
  Tower(Index),
}

pub struct ClickableTerrain {
  rect: IntRect,
  rect_type: ClickableTerrainRectType,
}

pub struct ClickableTerrainStorage {
  clickable_terrains: Vec<ClickableTerrain>,
}

impl ClickableTerrainStorage {
  pub fn new() -> ClickableTerrainStorage {
    ClickableTerrainStorage {
      clickable_terrains: Vec::new(),
    }
  }

  pub fn insert(&mut self, rect: IntRect, rect_type: ClickableTerrainRectType) {
    self
      .clickable_terrains
      .push(ClickableTerrain { rect, rect_type });
  }

  pub fn find_terrain(
    &mut self,
    position_x: i32,
    position_y: i32,
  ) -> Option<&mut ClickableTerrain> {
    self.clickable_terrains.iter_mut().find(|terrain| {
      position_x >= terrain.rect.left
        && position_x <= terrain.rect.left + terrain.rect.width
        && position_y >= terrain.rect.top
        && position_y <= terrain.rect.top + terrain.rect.height
    })
  }
}
