use sfml::graphics::RenderWindow;

use crate::world::World;

pub enum Transition {
  KeepState,
  #[allow(dead_code)]
  PopState, // For PauseState
  #[allow(dead_code)]
  PushState(Box<dyn GameState>), // For PauseState
  ReplaceState(Box<dyn GameState>),
  Quit,
}

pub trait GameState {
  fn run(&mut self, window: &mut RenderWindow, world: &mut World) -> Transition;
}
