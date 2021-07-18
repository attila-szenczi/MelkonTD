use sfml::system::Vector2f;
use sfml::window::mouse::Button;
use sfml::window::Event;

use super::input_state_trait::*;

use crate::animation::LinearScalePositionTransition;
use crate::battle_input_states::*;
use crate::flyout_actions::*;
use crate::world::World;

use generational_arena::Index;

pub struct FlyoutInputState {
  clickable_object_idx: Index,
}

impl FlyoutInputState {
  pub fn new(
    position_from: Vector2f,
    clickable_object_idx: Index,
    active_flyout_actions: &mut Vec<Box<FlyoutAction>>,
  ) -> Self {
    let position_to = Vector2f::new(position_from.x, position_from.y - 100.);

    let scale_from = Vector2f::new(0., 0.);
    let scale_to = Vector2f::new(1., 1.);
    let transition =
      LinearScalePositionTransition::new(position_from, position_to, scale_from, scale_to, 0.5);
    let flyout_action = Box::new(FlyoutAction::new(
      transition,
      String::from("sprites/locked_icon.png"),
      64, //TODO: Get it from texturemap
      64,
    ));
    //TODO: Drop 4
    active_flyout_actions.push(flyout_action);

    FlyoutInputState {
      clickable_object_idx,
    }
  }
}

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
