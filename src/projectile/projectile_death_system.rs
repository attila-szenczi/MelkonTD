// use amethyst::{derive::SystemDesc, ecs::prelude::*};

// use crate::projectile::Projectile;

// #[derive(SystemDesc)]
// pub struct ProjectileDeathSystem;

// impl<'a> System<'a> for ProjectileDeathSystem {
//   type SystemData = (Entities<'a>, ReadStorage<'a, Projectile>);

//   fn run(&mut self, (entities, projectiles): Self::SystemData) {
//     for (entity, projectile) in (&entities, &projectiles).join() {
//       if projectile.dead() {
//         match entities.delete(entity) {
//           Err(e) => println!("error during entity deletion: {:?}", e),
//           Ok(_v) => (),
//         }
//       }
//     }
//   }
// }
