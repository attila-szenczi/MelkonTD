use amethyst::{
  core::timing::Time,
  core::transform::Transform,
  derive::SystemDesc,
  ecs::{
    prelude::*,
    prelude::{Read, System},
  },
};

use crate::hierarchy_lookup::HierarchyLookup;
use crate::minion::Minion;

#[derive(SystemDesc)]
pub struct MinionUpdateSystem;

//TODO: Minion update system or split the healthbar
impl<'a> System<'a> for MinionUpdateSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Minion>,
    WriteStorage<'a, Transform>,
    Read<'a, Time>,
    Read<'a, HierarchyLookup>,
  );

  fn run(&mut self, (entities, minions, mut transforms, time, hierarchy_lookup): Self::SystemData) {
    for (_minion, transform) in (&minions, &mut transforms).join() {
      transform.prepend_translation_y(-50. * time.delta_seconds());
    }

    for (entity, minion) in (&entities, &minions).join() {
      let healthbar_front_entity = hierarchy_lookup
        .get_child(entity, "healthbar_front")
        .unwrap();
      let transform = transforms.get_mut(*healthbar_front_entity);

      match transform {
        Some(t) => {
          //TODO: Get rid of hardcoded stuff
          let multiplyer = (minion.health() as f32 / minion.max_health() as f32) * 2.;
          t.set_translation_x((multiplyer - 2.) * 16.);
          let scale = t.scale_mut();
          scale.x = multiplyer;
        }
        _ => (),
      }
    }
  }
}
