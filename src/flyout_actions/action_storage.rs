use crate::world::World;

use std::collections::HashMap;

use sfml::graphics::IntRect;

//pub type ActionClosureT = Box<dyn Fn(&mut World, i32, &IntRect)>;

pub struct ActionWithImage {
  //pub action: ActionClosureT, TODO: Do I need an actual function? Might be an enum dispatched to a function call elsewhere
  pub action_index: i32,
  pub image_name: String,
}

// Available associated build/upgrade/skill actions prefilled on level load, eg.: In earlier levels some upgrades might be locked.
// Or some options become available mid level
pub struct ActionStorage {
  actions: HashMap<i32, Vec<ActionWithImage>>,
}

impl ActionStorage {
  pub fn new(
  ) -> Self {
    ActionStorage {
      actions: HashMap::new(),
    }
  }
}
