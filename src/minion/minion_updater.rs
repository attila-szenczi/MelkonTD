use crate::minion::MinionTrait;

pub fn update_minions(minions: &mut Vec<Box<dyn MinionTrait>>, elapsed: f32) {
  for minion in minions {
    let position = minion.position_mut();
    position.x += 50. * elapsed;
  }
}
