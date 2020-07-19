use crate::minion::TestMinion;
use crate::world::World;
use sfml::system::Vector2f;

const SPAWN_POINT_X: f32 = 50.;
const SPAWN_POINT_Y: f32 = 820.;

pub struct MinionSpawner {
  pub counter: i32,
}

impl MinionSpawner {
  pub fn new() -> Self {
    MinionSpawner { counter: 2000 }
  }

  pub fn update(&mut self, world: &mut World) {
    self.counter += 1;

    if self.counter >= 200 {
      let minion = Box::new(TestMinion::new(Vector2f::new(SPAWN_POINT_X, SPAWN_POINT_Y)));
      world.minions.push(minion);

      self.counter = 0;
    }
  }
}
