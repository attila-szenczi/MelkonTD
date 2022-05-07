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
  //Index of the object which popped the flyout
  clickable_object_idx: Index,
}

impl FlyoutInputState {
  pub fn new(
    position_from: Vector2f,
    clickable_object_idx: Index,
    active_flyout_actions: &mut Vec<Box<FlyoutAction>>,
  ) -> Self {

    const FLYOUT_COUNT: i32 = 4;
    //TODO: Inject transition here. Flyout is created outside
    let flyout_transform_position_to_vectors =
      create_flyout_position_to_vectors(FLYOUT_COUNT);

    for i in 0..FLYOUT_COUNT {
      let direction_vec = flyout_transform_position_to_vectors[i as usize];
      println!("position_from {} {}", position_from.x, position_from.y);
      println!("direction {} {}", direction_vec.x, direction_vec.y);
      let position_to = Vector2f::new(position_from.x + direction_vec.x, position_from.y + direction_vec.y); 
      
      println!("FLyout popped to {} {}", position_to.x, position_to.y);

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
    }   

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

        return Transition::PopState;
      }
      _ => (),
    }
    Transition::KeepState
  }
}

fn create_flyout_position_to_vectors(len: i32) -> Vec<Vector2f> {
  let mut result = Vec::new();
  match len {
    0 => result,
    1 => {
      result.push(create_flyout_position_to_vector(0.));
      result
    }
    2 => {
      result.push(create_flyout_position_to_vector(20.));
      result.push(create_flyout_position_to_vector(-20.));
      result
    }
    3 => {
      result.push(create_flyout_position_to_vector(30.));
      result.push(create_flyout_position_to_vector(0.));
      result.push(create_flyout_position_to_vector(-30.));
      result
    }

    4 => {
      result.push(create_flyout_position_to_vector(40.));
      result.push(create_flyout_position_to_vector(15.));
      result.push(create_flyout_position_to_vector(-15.));
      result.push(create_flyout_position_to_vector(-40.));
      result
    }
    _ => panic!("only 4 action is supported"),
  }
}

fn create_flyout_position_to_vector(angle: f32) -> Vector2f {
  let mut direction_vec = Vector2f::new(0., -1.);
  //TODO: Move these to a util file
  rotate_ccw(&mut direction_vec, angle);
  let distance = 200.;
  direction_vec.x *= distance;
  direction_vec.y *= distance;

  direction_vec
}

fn rotate_ccw(vec: &mut Vector2f, angle_in_degree: f32) {
  let theta = angle_in_degree.to_radians();

  let cs = f32::cos(theta);
  let sn = f32::sin(theta);

  let px = vec.x * cs - vec.y * sn;
  vec.y = vec.x * sn + vec.y * cs;
  vec.x = px;
}
