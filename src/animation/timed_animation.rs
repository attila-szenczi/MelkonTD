pub struct TimedAnimation {
  frames: i32,
  pub current_frame: usize,
  time_per_frame: f32,
  elapsed_time: f32,
  cycle_time: f32,
}

impl TimedAnimation {
  pub fn new(frames: i32, time_per_frame: f32) -> TimedAnimation {
    TimedAnimation {
      frames: frames,
      current_frame: 0,
      time_per_frame: time_per_frame,
      elapsed_time: 0.0,
      cycle_time: time_per_frame * frames as f32,
    }
  }

  pub fn update(&mut self, elapsed: f32) {
    self.elapsed_time += elapsed;
    let frame_count = (self.elapsed_time / self.time_per_frame) as i32 % self.frames;
    self.current_frame = frame_count as usize;

    if self.elapsed_time > self.cycle_time {
      self.elapsed_time -= self.cycle_time
    }
  }
}
