use sfml::system::Vector2f;

pub struct LinearScalePositionTransition {
  pub position_current: Vector2f,
  pub scale_current: Vector2f,
  position_from: Vector2f,
  position_to: Vector2f,
  scale_from: Vector2f,
  scale_to: Vector2f,
  elapsed: f32,
  transition_time: f32,
  transition_finished: bool,
}

impl LinearScalePositionTransition {
  pub fn new(
    position_from: Vector2f,
    position_to: Vector2f,
    scale_from: Vector2f,
    scale_to: Vector2f,
    transition_time: f32,
  ) -> LinearScalePositionTransition {
    LinearScalePositionTransition {
      position_from,
      position_to,
      position_current: position_from,
      scale_from,
      scale_to,
      scale_current: scale_from,
      elapsed: 0.,
      transition_time,
      transition_finished: false,
    }
  }

  pub fn update(&mut self, elapsed: f32) -> bool {
    if self.elapsed > self.transition_time {
      if self.transition_finished {
        return false;
      }

      self.transition_finished = true;
    }

    self.elapsed += elapsed;

    let end_percentage = self.transition_percentage_multiplier();
    let begin_percentage = 1. - end_percentage;
    self.position_current.x =
      self.position_from.x * begin_percentage + self.position_to.x * end_percentage;
    self.position_current.y =
      self.position_from.y * begin_percentage + self.position_to.y * end_percentage;
    self.scale_current.x = self.scale_from.x * begin_percentage + self.scale_to.x * end_percentage;
    self.scale_current.y = self.scale_from.y * begin_percentage + self.scale_to.y * end_percentage;
    return true;
  }

  pub fn transition_percentage_multiplier(&self) -> f32 {
    (self.elapsed / self.transition_time).min(1.)
  }
}
