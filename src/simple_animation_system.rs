use amethyst::{
  core::timing::Time,
  core::transform::Transform,
  derive::SystemDesc,
  ecs::{
    prelude::*,
    prelude::{Read, System},
  },
  renderer::SpriteRender,
};

use crate::projectile::Projectile;
use crate::simple_animation::SimpleAnimation;

#[derive(SystemDesc)]
pub struct SimpleAnimationSystem;

impl<'a> System<'a> for SimpleAnimationSystem {
  type SystemData = (
    WriteStorage<'a, SpriteRender>,
    WriteStorage<'a, SimpleAnimation>,
    WriteStorage<'a, Projectile>, //Temp as it's not generalized yet
    WriteStorage<'a, Transform>,
    Read<'a, Time>,
  );

  fn run(
    &mut self,
    (mut sprite_renders, mut animations, mut projectiles, mut transforms, time): Self::SystemData,
  ) {
    for (sprite_render, anim, projectile, transform) in (
      &mut sprite_renders,
      &mut animations,
      &mut projectiles,
      &mut transforms,
    )
      .join()
    {
      let elapsed = time.delta_seconds();
      anim.elapsed_time += elapsed;
      let frame_count = (anim.elapsed_time / anim.time_per_frame) as i32 % anim.frames;
      if frame_count != anim.current_frame {
        anim.current_frame = frame_count;
        sprite_render.sprite_number = frame_count as usize;
      }
      let scale = transform.scale_mut();

      let diff = {
        if projectile.increase_scale {
          projectile.max_scale.x * elapsed
        } else {
          -projectile.max_scale.x * elapsed
        }
      };
      scale.x += diff;
      if scale.x > projectile.max_scale.x {
        scale.x = projectile.max_scale.x;
        projectile.increase_scale = false;
      } else if !projectile.increase_scale && scale.x < projectile.max_scale.x * 0.8 {
        scale.x = projectile.max_scale.x * 0.8;
        projectile.increase_scale = true;
      }
      scale.y = scale.x;
    }
  }
}
