use crate::animation::LinearScalePositionTransition;

use crate::flyout_actions::{ActionWithImage, FlyoutAction};

use sfml::system::Vector2f;

pub fn generate_flyouts(active_flyout_actions: &mut Vec<Box<FlyoutAction>>, actions: &Vec<ActionWithImage>, position_from: Vector2f) {
  let flyout_transform_position_to_vectors =
    create_flyout_offset_vectors(actions.len());

  for i in 0..actions.len() {
    let direction_vec = flyout_transform_position_to_vectors[i as usize];
    let position_to = Vector2f::new(position_from.x + direction_vec.x, position_from.y + direction_vec.y); 

    let scale_from = Vector2f::new(0., 0.);
    let scale_to = Vector2f::new(1., 1.);
    let transition =
      LinearScalePositionTransition::new(position_from, position_to, scale_from, scale_to, 0.3);
    let flyout_action = Box::new(FlyoutAction::new(
      transition,
      actions[i].image_name.clone(),
      64, //TODO: Get it from texturemap
      64,
      0, // TODO: Pass from up once entity types has associated source_ids
      i as i32,
    ));

    active_flyout_actions.push(flyout_action);
  }
}

fn create_flyout_offset_vectors(len: usize) -> Vec<Vector2f> {
  let mut result = Vec::new();
  match len {
    0 => result,
    1 => {
      result.push(create_flyout_offset_vector(0.));
      result
    }
    2 => {
      result.push(create_flyout_offset_vector(-20.));
      result.push(create_flyout_offset_vector(20.));
      result
    }
    3 => {
      result.push(create_flyout_offset_vector(-30.));
      result.push(create_flyout_offset_vector(0.));
      result.push(create_flyout_offset_vector(30.));
      result
    }

    4 => {
      result.push(create_flyout_offset_vector(-40.));
      result.push(create_flyout_offset_vector(-15.));
      result.push(create_flyout_offset_vector(15.));
      result.push(create_flyout_offset_vector(40.));
      result
    }
    _ => panic!("max 4 actions are supported"),
  }
}

fn create_flyout_offset_vector(angle: f32) -> Vector2f {
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
