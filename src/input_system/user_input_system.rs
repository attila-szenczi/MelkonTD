// use amethyst::shrev::EventChannel;
// use amethyst::{
//   assets::AssetStorage,
//   core::transform::Transform,
//   derive::SystemDesc,
//   ecs::{
//     prelude::*,
//     prelude::{Entities, LazyUpdate, Read, System},
//   },
//   input::{InputHandler, StringBindings},
//   renderer::{Camera, SpriteSheet, Texture},
//   window::ScreenDimensions,
// };

// use super::default_input_state::DefaultInputState;
// use super::input_state_trait::{EventType, InputState, Transition};
// use crate::flyout_actions::FlyoutActionStorage;
// use crate::tile_map::TileMap;

// #[derive(SystemDesc)]
// pub struct UserInputSystem {
//   reader_id: ReaderId<EventType>,
//   state_stack: Vec<Box<dyn InputState>>,
// }

// impl UserInputSystem {
//   pub fn new(reader_id: ReaderId<EventType>) -> Self {
//     UserInputSystem {
//       reader_id,
//       state_stack: vec![Box::new(DefaultInputState)],
//     }
//   }
// }

// impl<'a> System<'a> for UserInputSystem {
//   type SystemData = (
//     Read<'a, EventChannel<EventType>>,
//     Read<'a, InputHandler<StringBindings>>,
//     WriteExpect<'a, TileMap>,
//     Entities<'a>,
//     Read<'a, LazyUpdate>,
//     Read<'a, AssetStorage<Texture>>,
//     Read<'a, AssetStorage<SpriteSheet>>,
//     ReadStorage<'a, Camera>,
//     ReadStorage<'a, Transform>,
//     ReadExpect<'a, ScreenDimensions>,
//     Read<'a, FlyoutActionStorage>,
//   );

//   fn run(
//     &mut self,
//     (
//       channel,
//       input_handler,
//       mut tile_map,
//       entities,
//       updater,
//       _texture_storage,
//       _sprite_sheet_storage,
//       camera_storage,
//       transform_storage,
//       screen_dimensions,
//       flyout_action_storage,
//     ): Self::SystemData,
//   ) {
//     let state = self.state_stack.last_mut().unwrap();
//     let mut transition = Transition::KeepState;
//     for event in channel.read(&mut self.reader_id) {
//       //Continue the loop even in case it's not KeepState to drop each event before transitioning to the next state.
//       match transition {
//         Transition::KeepState => {
//           transition = state.process_event(
//             event,
//             &input_handler,
//             &mut tile_map,
//             &entities,
//             &updater,
//             &camera_storage,
//             &transform_storage,
//             &screen_dimensions,
//             &flyout_action_storage,
//           );
//         }
//         _ => (),
//       }
//     }

//     match transition {
//       Transition::KeepState => (),
//       Transition::PopState => {
//         self.state_stack.pop();
//         ()
//       }
//       Transition::PushState(state) => {
//         self.state_stack.push(state);
//         ()
//       }
//     }
//   }
// }
