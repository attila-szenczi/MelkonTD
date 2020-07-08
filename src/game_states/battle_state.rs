use sfml::window::{Event, Window};

use super::game_state_trait::{GameState, Transition};

pub struct BattleState;

impl BattleState {
  pub fn new() -> Self {
    BattleState {}
  }
}

impl<'b> GameState for BattleState {
  fn run(&mut self, window: &mut Window) -> Transition {
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
    while window.is_open() {
      // Event processing
      while let Some(event) = window.poll_event() {
        // Request closing for the window
        if event == Event::Closed {
          window.close();
        }
      }
    }

    Transition::KeepState
  }
}
