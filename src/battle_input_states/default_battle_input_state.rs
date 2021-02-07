use sfml::window::mouse::Button;
use sfml::window::Event;

use super::input_state_trait::*;
use crate::world::World;

pub struct DefaultBattleInputState {}

impl InputStateTrait for DefaultBattleInputState {
  fn process_event<'a>(&mut self, event: &Event, world: &mut World) -> Transition {
    match event {
      Event::MouseButtonPressed {
        button: Button::Left,
        x,
        y,
      } => {
        println!("Left click {} {}", x, y);
      }
      _ => (),
    }
    Transition::KeepState
  }
}
