use crate::minion::MinionTrait;

use generational_arena::Arena;

pub fn update_minions(minions: &mut Arena<Box<dyn MinionTrait>>, elapsed: f32) {
  for (_index, minion) in minions {
    let position = minion.position_mut();
    position.x += 50. * elapsed;
  }
}
