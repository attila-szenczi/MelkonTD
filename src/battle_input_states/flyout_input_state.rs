use sfml::system::Vector2f;
use sfml::window::mouse::Button;
use sfml::window::Event;

use super::input_state_trait::*;

use crate::animation::LinearScalePositionTransition;
use crate::battle_input_states::*;
use crate::flyout_actions::*;
use crate::world::World;

pub struct FlyoutInputState {}

impl InputStateTrait for FlyoutInputState {
  fn process_event<'a>(&mut self, event: &Event, world: &mut World) -> Transition {
    match event {
      Event::MouseButtonPressed {
        button: Button::Left,
        x,
        y,
      } => {
        println!("Clicked to {} {}", x, y);
        //TODO: More idiomatic solution than cloning?
        let x_pos = x.clone();
        let y_pos = y.clone();
        //TODO: Factorize bound check (repeated in clickable_object_storage)
        {
          let v = world.active_flyout_actions.get(0).unwrap();
          println!(
            "Flyout position {} {} {} {}",
            v.rect.left, v.rect.top, v.rect.width, v.rect.height
          );
        }
        let action_option = world.active_flyout_actions.drain(..).find(|object| {
          x_pos >= object.rect.left
            && x_pos <= object.rect.left + object.rect.width
            && y_pos >= object.rect.top
            && y_pos <= object.rect.top + object.rect.height
        });

        match action_option {
          None => (),
          Some(mut flyout_action) => {
            println!(
              "Flyout left clicked {} {} {} {}",
              flyout_action.rect.left,
              flyout_action.rect.top,
              flyout_action.rect.width,
              flyout_action.rect.height
            );
            flyout_action.execute(world);
          }
        }

        return Transition::PopState;
      }
      _ => (),
    }
    Transition::KeepState
  }
}
