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

use super::input_state_trait::{EventType, InputState, Transition};
use crate::texture_lookup::TextureLookup;
use crate::tile_map::{TileMap, TileType};
use crate::tower::Tower;
use crate::z_layer::z_layer_to_coordinate;
use crate::z_layer::ZLayer;

pub struct DefaultInputState;

impl<'b> InputState for DefaultInputState {
  fn process_events<'a>(
    &mut self,
    event: &EventType,
    input_handler: &Read<'a, InputHandler<StringBindings>>,
    tile_map: &mut WriteExpect<'a, TileMap>,
    entities: &Entities<'a>,
    updater: &Read<'a, LazyUpdate>,
    cameras: &ReadStorage<'a, Camera>,
    transforms: &ReadStorage<'a, Transform>,
    screen_dimensions: &ReadExpect<'a, ScreenDimensions>,
    texture_lookup: &Read<'a, TextureLookup>,
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
                let mut transform = Transform::default();
                transform.set_translation_xyz(
                  rect.bottom_left.x as f32,
                  rect.bottom_left.y as f32,
                  z_layer_to_coordinate(ZLayer::Tower),
                );

                let projectile_sprite_render_with_scale = texture_lookup
                  .get_texture_with_scale("private_sprites/pulsing_electric_ball", 32);
                let entity = updater
                  .create_entity(&entities)
                  .with(texture_lookup.get_texture("sprites/tower", 0))
                  .with(transform)
                  .with(Tower::new(
                    projectile_sprite_render_with_scale.0,
                    projectile_sprite_render_with_scale.1,
                  ))
                  .build();
                tile_map.occupy_slot(index, entity);
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
