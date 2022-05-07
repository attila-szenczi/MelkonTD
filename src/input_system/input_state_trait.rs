// use amethyst::{
//   core::transform::Transform,
//   ecs::{
//     prelude::*,
//     prelude::{Entities, LazyUpdate, Read},
//   },
//   input::{InputEvent, InputHandler, StringBindings},
//   renderer::Camera,
//   window::ScreenDimensions,
// };

// use crate::flyout_actions::FlyoutActionStorage;
// use crate::tile_map::TileMap;

// pub enum Transition {
//   KeepState,
//   PopState,
//   PushState(Box<dyn InputState>),
// }

// pub type EventType = InputEvent<StringBindings>;

// pub trait InputState: Send {
//   fn process_event<'a>(
//     &mut self,
//     event: &EventType,
//     input_handler: &Read<'a, InputHandler<StringBindings>>,
//     tile_map: &mut WriteExpect<'a, TileMap>,
//     entities: &Entities<'a>,
//     updater: &Read<'a, LazyUpdate>,
//     cameras: &ReadStorage<'a, Camera>,
//     transforms: &ReadStorage<'a, Transform>,
//     screen_dimensions: &ReadExpect<'a, ScreenDimensions>,
//     flyout_action_storage: &Read<'a, FlyoutActionStorage>,
//   ) -> Transition;
// }
