use sfml::window::Window;

pub enum Transition {
  KeepState,
  #[allow(dead_code)]
  PopState,
  PushState(Box<dyn GameState>),
}

pub trait GameState: Send {
  fn run(&mut self, window: &mut Window) -> Transition;
}
