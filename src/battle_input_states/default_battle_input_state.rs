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
        let object_option = world.clickable_objects.find_object(x.clone(), y.clone());
        match object_option {
          None => (),
          Some(object) => {
            println!(
              "Object left clicked {} {} {} {}",
              object.rect.left, object.rect.top, object.rect.width, object.rect.height
            );
          }
        }
      }
      _ => (),
    }
    Transition::KeepState
  }
}
