use amethyst::{
  core::math::Point3,
  core::transform::Transform,
  ecs::{
    prelude::*,
    prelude::{Entities, LazyUpdate, Read},
  },
  input::{InputHandler, StringBindings},
  renderer::Camera,
  window::ScreenDimensions,
  winit::MouseButton,
};

use super::flyout_input_state::FlyoutInputState;
use super::input_state_trait::{EventType, InputState, Transition};
use crate::flyout_actions::{EntityType, FlyoutActionStorage};
use crate::tile_map::{TileMap, TileType};

pub struct DefaultInputState;

impl<'b> InputState for DefaultInputState {
  fn process_event<'a>(
    &mut self,
    event: &EventType,
    input_handler: &Read<'a, InputHandler<StringBindings>>,
    tile_map: &mut WriteExpect<'a, TileMap>,
    entities: &Entities<'a>,
    updater: &Read<'a, LazyUpdate>,
    cameras: &ReadStorage<'a, Camera>,
    transforms: &ReadStorage<'a, Transform>,
    screen_dimensions: &ReadExpect<'a, ScreenDimensions>,
    flyout_actions: &Read<'a, FlyoutActionStorage>,
  ) -> Transition {
    match event {
      EventType::MouseButtonPressed(MouseButton::Left) => {
        if let Some((x, y)) = input_handler.mouse_position() {
          if let Some((camera, transform)) = (cameras, transforms).join().next() {
            let center_screen = Point3::new(x, y, 0.0);

            let world_point = camera.projection().screen_to_world_point(
              center_screen,
              screen_dimensions.diagonal(),
              &transform,
            );

            match tile_map.find_tile(world_point.x as i32, world_point.y as i32) {
              Some((index, TileType::Slot, rect)) => {
                let flyout_input_state = Box::new(FlyoutInputState::new(
                  entities,
                  updater,
                  index,
                  rect,
                  flyout_actions.get_actions(&EntityType::Tile(TileType::Slot)),
                ));
                return Transition::PushState(flyout_input_state);
              }
              _ => (),
            }
          }
        }
      }
      _ => (),
    }

    Transition::KeepState
  }
}
