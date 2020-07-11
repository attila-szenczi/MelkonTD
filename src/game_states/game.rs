use sfml::graphics::{FloatRect, RenderTarget, RenderWindow, View};
use sfml::window::Style;

use super::game_state_trait::{GameState, Transition};
use super::loading_state::LoadingState;
use crate::world::World;

pub struct Game {
  state_stack: Vec<Box<dyn GameState>>,
  world: World,
}

impl Game {
  pub fn new() -> Self {
    Game {
      state_stack: vec![Box::new(LoadingState::new())],
      world: World::new(),
    }
  }

  pub fn run(&mut self) {
    let mut window = RenderWindow::new((1920, 1080), "MelkonTD", Style::CLOSE, &Default::default());
    window.set_framerate_limit(60);
    let camera = View::from_rect(&FloatRect::new(0., 0., 1920., 1080.));
    window.set_view(&camera);

    while !self.state_stack.is_empty() {
      let state = self.state_stack.last_mut().unwrap();

      let transition = state.run(&mut window, &mut self.world);
      match transition {
        Transition::KeepState => (),
        Transition::PopState => {
          self.state_stack.pop();
          ()
        }
        Transition::PushState(state) => {
          self.state_stack.push(state);
          ()
        }
      }
    }
  }
}
