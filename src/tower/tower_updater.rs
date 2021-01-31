use crate::world::World;

pub fn update_towers(world: &mut World, elapsed: f32) {
  for (_index, tower) in &mut world.towers {
    tower.update(&mut world.minions, &mut world.projectiles, elapsed);
  }
}
