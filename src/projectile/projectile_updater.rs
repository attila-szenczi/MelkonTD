use crate::world::World;

pub fn update_projectiles(world: &mut World, elapsed: f32) {
  for (_index, projectile) in &mut world.projectiles {
    projectile.update(&mut world.minions, elapsed);
  }
}
