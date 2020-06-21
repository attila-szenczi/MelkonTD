use amethyst::{derive::SystemDesc, ecs::prelude::*};

use crate::hierarchy_lookup::HierarchyLookup;
use crate::minion::Minion;

#[derive(SystemDesc)]
pub struct MinionDeathSystem;

impl<'a> System<'a> for MinionDeathSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Minion>,
    Write<'a, HierarchyLookup>,
  );

  fn run(&mut self, (entities, minions, mut hierarchy_lookup): Self::SystemData) {
    for (entity, minion) in (&entities, &minions).join() {
      if minion.dead() {
        match entities.delete(entity) {
          Err(e) => println!("error during entity deletion: {:?}", e),
          Ok(_v) => (),
        }
        hierarchy_lookup.remove_entity(entity);
      }
    }
  }
}
