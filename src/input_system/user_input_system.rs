use amethyst::shrev::EventChannel;
use amethyst::{
  assets::AssetStorage,
  core::transform::Transform,
  derive::SystemDesc,
  ecs::{
    prelude::*,
    prelude::{Entities, LazyUpdate, Read, System},
  },
  input::{InputHandler, StringBindings},
  renderer::{Camera, SpriteSheet, Texture},
  window::ScreenDimensions,
};

use super::default_input_state::DefaultInputState;
use super::input_state_trait::EventType;
use super::input_state_trait::InputState;
use crate::texture_lookup::TextureLookup;
use crate::tile_map::TileMap;

#[derive(SystemDesc)]
pub struct UserInputSystem {
  reader_id: ReaderId<EventType>,
  state_stack: Vec<Box<dyn InputState>>,
}

impl UserInputSystem {
  pub fn new(reader_id: ReaderId<EventType>) -> Self {
    UserInputSystem {
      reader_id,
      state_stack: vec![Box::new(DefaultInputState)],
    }
  }
}

impl<'a> System<'a> for UserInputSystem {
  type SystemData = (
    Read<'a, EventChannel<EventType>>,
    Read<'a, InputHandler<StringBindings>>,
    WriteExpect<'a, TileMap>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
    Read<'a, AssetStorage<Texture>>,
    Read<'a, AssetStorage<SpriteSheet>>,
    ReadStorage<'a, Camera>,
    ReadStorage<'a, Transform>,
    ReadExpect<'a, ScreenDimensions>,
    Read<'a, TextureLookup>,
  );

  fn run(
    &mut self,
    (
      channel,
      input_handler,
      mut tile_map,
      entities,
      updater,
      _texture_storage,
      _sprite_sheet_storage,
      camera_storage,
      transform_storage,
      screen_dimensions,
      texture_lookup,
    ): Self::SystemData,
  ) {
    let state = self.state_stack.last_mut().unwrap();
    for event in channel.read(&mut self.reader_id) {
      state.process_event(
        event,
        &input_handler,
        &mut tile_map,
        &entities,
        &updater,
        &camera_storage,
        &transform_storage,
        &screen_dimensions,
        &texture_lookup,
      );
    }
  }
}
