use std::ops::Deref;

use sfml::graphics::{RenderStates, RenderTarget, RenderWindow, Sprite};
use sfml::window::Event;

use super::game_state_trait::{GameState, Transition};

use crate::world::World;

pub struct BattleState;

impl BattleState {
  pub fn new() -> Self {
    BattleState {}
  }
}

impl<'b> GameState for BattleState {
  fn run(&mut self, window: &mut RenderWindow, world: &mut World) -> Transition {
    // match event {
    //   EventType::MouseButtonPressed(MouseButton::Left) => {
    //     if let Some((x, y)) = input_handler.mouse_position() {
    //       if let Some((camera, transform)) = (cameras, transforms).join().next() {
    //         let center_screen = Point3::new(x, y, 0.0);

    //         let world_point = camera.projection().screen_to_world_point(
    //           center_screen,
    //           screen_dimensions.diagonal(),
    //           &transform,
    //         );

    //         match tile_map.find_tile(world_point.x as i32, world_point.y as i32) {
    //           Some((index, TileType::Slot, rect)) => {
    //             let flyout_input_state = Box::new(FlyoutInputState::new(
    //               entities,
    //               updater,
    //               index,
    //               rect,
    //               flyout_actions.get_actions(&EntityType::Tile(TileType::Slot)),
    //             ));
    //             return Transition::PushState(flyout_input_state);
    //           }
    //           _ => (),
    //         }
    //       }
    //     }
    //   }
    //   _ => (),
    // }
    let texture = &world.texture_storage.get_texture_data("background").texture;

    let sprite = Sprite::with_texture(texture.deref());

    window.set_active(true);
    window.draw_sprite(&sprite, RenderStates::default());
    window.display();

    // Event processing
    while let Some(event) = window.poll_event() {
      // Request closing for the window
      match event {
        Event::Closed => window.close(),
        Event::MouseButtonPressed { button, x, y } => println!("Left click"),
        _ => (),
      }
    }

    Transition::KeepState
  }
}
