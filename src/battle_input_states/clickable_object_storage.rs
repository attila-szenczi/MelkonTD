use sfml::graphics::IntRect;

use generational_arena::{Arena, Index};

pub enum ClickableObjectType {
  Slot,
  Tower(Index),
}

pub struct ClickableObject {
  pub rect: IntRect,
  pub object_type: ClickableObjectType,
}

pub struct ClickableObjectStorage {
  clickable_objects: Arena<ClickableObject>,
}

impl ClickableObjectStorage {
  pub fn new() -> ClickableObjectStorage {
    ClickableObjectStorage {
      clickable_objects: Arena::new(),
    }
  }

  pub fn insert(&mut self, rect: IntRect, object_type: ClickableObjectType) {
    self
      .clickable_objects
      .insert(ClickableObject { rect, object_type });
  }

  pub fn find_object(
    &mut self,
    position_x: i32,
    position_y: i32,
  ) -> Option<(Index, &mut ClickableObject)> {
    self.clickable_objects.iter_mut().find(|object| {
      position_x >= object.1.rect.left
        && position_x <= object.1.rect.left + object.1.rect.width
        && position_y >= object.1.rect.top
        && position_y <= object.1.rect.top + object.1.rect.height
    })
  }
}
