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
use crate::flyout_actions::{FlyoutAction, FlyoutActionStorage};
use crate::tile_map::TileMap;
use crate::z_layer::z_layer_to_coordinate;
use crate::z_layer::ZLayer;
use utils::coord::{Coord, Vector2};
use utils::rect::Rect;

pub struct FlyoutInputState {
  pub clicked_tile_index: i32,
  pub clicked_tile_rect: Rect,
  pub flyout_entities: Vec<Entity>,
  pub flyout_rects: Vec<Rect>,
  pub flyout_actions: Vec<FlyoutAction>,
}

impl FlyoutInputState {
  pub fn new<'a>(
    entities: &Entities<'a>,
    updater: &Read<'a, LazyUpdate>,
    clicked_tile_index: i32,
    clicked_tile_rect: Rect,
    flyout_actions: Vec<FlyoutAction>,
  ) -> Self {
    let tile_dimension = clicked_tile_rect.width as f32;
    let flyout_dimension = 24.;
    let dimension_diff = tile_dimension - flyout_dimension;

    let flyout_transform_vectors = create_flyout_transform_vectors(flyout_actions.len() as i32);
    let mut flyout_entities = Vec::new();
    let mut flyout_rects = Vec::new();
    for (i, action) in flyout_actions.iter().enumerate() {
      let mut transform = Transform::default();
      transform.set_translation_xyz(
        clicked_tile_rect.bottom_left.x as f32
          + tile_dimension / 2.
          + flyout_transform_vectors[i].x,
        clicked_tile_rect.bottom_left.y as f32
          + tile_dimension / 2.
          + flyout_transform_vectors[i].y,
        z_layer_to_coordinate(ZLayer::UiFlyout),
      );
      //transform.set_scale(action.icon.default_scale);

      // flyout_entities.push(
      //   updater
      //     .create_entity(&entities)
      //     .with(action.icon.sprite_render.clone())
      //     .with(transform)
      //     .build(),
      // );

      flyout_rects.push(Rect::new(
        Coord::new(
          clicked_tile_rect.bottom_left.x
            + (dimension_diff / 2.) as i32
            + flyout_transform_vectors[i].x as i32,
          clicked_tile_rect.bottom_left.y
            + (dimension_diff / 2.) as i32
            + flyout_transform_vectors[i].y as i32,
        ),
        clicked_tile_rect.width - dimension_diff as i32,
        clicked_tile_rect.height - dimension_diff as i32,
      ));
    }

    FlyoutInputState {
      clicked_tile_index,
      clicked_tile_rect,
      flyout_entities,
      flyout_rects,
      flyout_actions,
    }
  }
}

impl<'b> InputState for FlyoutInputState {
  fn process_event<'a>(
    &mut self,
    event: &EventType,
    input_handler: &Read<'a, InputHandler<StringBindings>>,
    _tile_map: &mut WriteExpect<'a, TileMap>,
    entities: &Entities<'a>,
    updater: &Read<'a, LazyUpdate>,
    cameras: &ReadStorage<'a, Camera>,
    transforms: &ReadStorage<'a, Transform>,
    screen_dimensions: &ReadExpect<'a, ScreenDimensions>,
    _flyout_action_storage: &Read<'a, FlyoutActionStorage>,
  ) -> Transition {
    match event {
      EventType::MouseButtonPressed(MouseButton::Left) => {
        if let Some((x, y)) = input_handler.mouse_position() {
          let world_point = {
            if let Some((camera, transform)) = (cameras, transforms).join().next() {
              let center_screen = Point3::new(x, y, 0.0);
              Some(camera.projection().screen_to_world_point(
                center_screen,
                screen_dimensions.diagonal(),
                &transform,
              ))
            } else {
              None
            }
          };
          if let Some(world_point) = world_point {
            for (i, action) in self.flyout_actions.iter().enumerate() {
              if self.flyout_rects[i].is_in(world_point.x, world_point.y) {
                let clicked_tile_index = self.clicked_tile_index.clone();
                let clicked_tile_rect = self.clicked_tile_rect.clone();
                let flyout_action = action.clone();
                // updater.exec_mut(move |world| {
                //   (flyout_action.action)(world, clicked_tile_index, clicked_tile_rect);
                // });

                for entity in &self.flyout_entities {
                  entities
                    .delete(entity.clone())
                    .expect("failed to delete flyout");
                }

                return Transition::PopState;
              }
            }
            for entity in &self.flyout_entities {
              entities
                .delete(entity.clone())
                .expect("failed to delete flyout");
            }
            return Transition::PopState;
          }
        }
      }
      _ => (),
    }
    Transition::KeepState
  }
}

fn create_flyout_transform_vectors(len: i32) -> Vec<Vector2> {
  let mut result = Vec::new();
  match len {
    0 => result,
    1 => {
      result.push(create_transform_vector(0.));
      result
    }
    2 => {
      result.push(create_transform_vector(30.));
      result.push(create_transform_vector(-30.));
      result
    }
    3 => {
      result.push(create_transform_vector(45.));
      result.push(create_transform_vector(0.));
      result.push(create_transform_vector(-45.));
      result
    }

    4 => {
      result.push(create_transform_vector(75.));
      result.push(create_transform_vector(30.));
      result.push(create_transform_vector(-30.));
      result.push(create_transform_vector(-75.));
      result
    }
    _ => panic!("only 4 action is supported"),
  }
}

fn create_transform_vector(angle: f32) -> Vector2 {
  let mut direction_vec = Vector2::new_unit_vector_up();
  direction_vec.rotate_ccw(angle);
  direction_vec.x *= 40.;
  direction_vec.y *= 40.;

  direction_vec
}
