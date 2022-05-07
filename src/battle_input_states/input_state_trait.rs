use sfml::window::Event;

use crate::world::World;

pub enum Transition {
  KeepState,
  PopState,
  PushState(Box<dyn InputStateTrait>),
}

pub trait InputStateTrait {
  fn process_event<'a>(&mut self, event: &Event, world: &mut World) -> Transition;
}
