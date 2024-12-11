use sfml::system::Vector2f;
use sfml::window::mouse::Button;
use sfml::window::Event;

use super::input_state_trait::*;

use crate::flyout_actions::{FlyoutAction, generate_flyouts, ActionWithImage};
use crate::world::World;

use generational_arena::Index;

pub struct FlyoutInputState {
  //Index of the object which popped the flyout. 
  //TODO: Do I need it at all?
  clickable_object_idx: Index,
}

impl FlyoutInputState {
  pub fn new(
    position_from: Vector2f,
    clickable_object_idx: Index,
    active_flyout_actions: &mut Vec<Box<FlyoutAction>>,
    actions: &Vec<ActionWithImage>,
  ) -> Self {

    generate_flyouts(active_flyout_actions, actions, position_from);

    FlyoutInputState {
      clickable_object_idx,
    }
  }
}

impl InputStateTrait for FlyoutInputState {
  fn process_event<'a>(&mut self, event: &Event, world: &mut World) -> Transition {
    match event {
      Event::MouseButtonPressed {
        button: Button::LEFT,
        x,
        y,
      } => {
        println!("Clicked to {} {}", x, y);
        //TODO: Factorize bound check (repeated in clickable_object_storage)
        {
          let v = world.active_flyout_actions.get(0).unwrap();
          println!(
            "Flyout position {} {} {} {}",
            v.rect.left, v.rect.top, v.rect.width, v.rect.height
          );
        }
        //TODO: clear active_flyout_actions on leaving state?
        let action_option = world.active_flyout_actions.drain(..).find(|object| {
          *x >= object.rect.left
            && *x <= object.rect.left + object.rect.width
            && *y >= object.rect.top
            && *y <= object.rect.top + object.rect.height
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

        world.active_flyout_actions.clear();

        return Transition::PopState;
      }
      _ => (),
    }
    Transition::KeepState
  }
}
