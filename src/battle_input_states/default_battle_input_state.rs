use sfml::system::Vector2f;
use sfml::window::mouse::Button;
use sfml::window::Event;

use super::flyout_input_state::FlyoutInputState;
use super::input_state_trait::*;

use crate::battle_input_states::*;
use crate::world::World;

pub struct DefaultBattleInputState {}

impl InputStateTrait for DefaultBattleInputState {
  fn process_event<'a>(&mut self, event: &Event, world: &mut World) -> Transition {
    match event {
      Event::MouseButtonPressed {
        button: Button::LEFT,
        x,
        y,
      } => {
        let object_option = world.clickable_objects.find_object(x.clone(), y.clone());
        //TODO: Use this for flyout and simply don't process out of state stuff.
        match object_option {
          None => (),
          Some((index, object)) => {
            println!(
              "Object left clicked {} {} {} {}",
              object.rect.left, object.rect.top, object.rect.width, object.rect.height
            );
            match &object.object_type {
              ClickableObjectType::Tower(_) => (),
              ClickableObjectType::Slot => {
                //let middlePoint = sfml
                //TODO: Generate it from distance + angle as
                let position_from = Vector2f::new(
                  (object.rect.left + object.rect.width / 2) as f32,
                  (object.rect.top + object.rect.height / 2) as f32,
                );

                return Transition::PushState(Box::new(FlyoutInputState::new(position_from, index, &mut world.active_flyout_actions)));
              }
            }
          }
        }
      }
      _ => (),
    }
    Transition::KeepState
  }
}
