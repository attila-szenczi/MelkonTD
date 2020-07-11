use sfml::graphics::RenderWindow;

use crate::world::World;

pub enum Transition {
  KeepState,
  #[allow(dead_code)]
  PopState,
  PushState(Box<dyn GameState>),
}

pub trait GameState: Send {
  fn run(&mut self, window: &mut RenderWindow, world: &mut World) -> Transition;
}
