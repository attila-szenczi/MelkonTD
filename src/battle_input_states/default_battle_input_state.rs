use sfml::system::Vector2f;
use sfml::window::mouse::Button;
use sfml::window::Event;

use super::input_state_trait::*;

use crate::animation::LinearScalePositionTransition;
use crate::battle_input_states::*;
use crate::flyout_actions::*;
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
            match &object.rect_type {
              ClickableObjectRectType::Tower(Index) => (),
              Slot => {
                //let middlePoint = sfml
                //TODO: Generate it from distance + angle as
                let position_from = Vector2f::new(
                  (object.rect.left + object.rect.width / 2) as f32,
                  (object.rect.top + object.rect.height / 2) as f32,
                );
                let position_to = Vector2f::new(position_from.x, position_from.y - 100.);
                let scale_from = Vector2f::new(0., 0.);
                let scale_to = Vector2f::new(1., 1.);
                let transition = LinearScalePositionTransition::new(
                  position_from,
                  position_to,
                  scale_from,
                  scale_to,
                  0.5,
                );
                let flyout_action = Box::new(FlyoutAction::new(
                  transition,
                  String::from("sprites/locked_icon.png"),
                ));
                world.active_flyout_actions.push(flyout_action);
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
