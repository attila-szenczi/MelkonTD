// use amethyst::{
//   core::timing::Time,
//   derive::SystemDesc,
//   ecs::{
//     prelude::*,
//     prelude::{Read, System},
//   },
//   renderer::SpriteRender,
// };

// use crate::simple_animation::SimpleAnimation;

// #[derive(SystemDesc)]
// pub struct SimpleAnimationSystem;

// impl<'a> System<'a> for SimpleAnimationSystem {
//   type SystemData = (
//     WriteStorage<'a, SpriteRender>,
//     WriteStorage<'a, SimpleAnimation>,
//     Read<'a, Time>,
//   );

//   fn run(&mut self, (mut sprite_renders, mut animations, time): Self::SystemData) {
//     for (sprite_render, anim) in (&mut sprite_renders, &mut animations).join() {
//       let elapsed = time.delta_seconds();
//       anim.elapsed_time += elapsed;
//       let frame_count = (anim.elapsed_time / anim.time_per_frame) as i32 % anim.frames;
//       if frame_count != anim.current_frame {
//         anim.current_frame = frame_count;
//         sprite_render.sprite_number = frame_count as usize;
//       }
//     }
//   }
// }
