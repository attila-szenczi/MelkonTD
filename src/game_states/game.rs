use sfml::window::{Style, Window};

use super::game_state_trait::{GameState, Transition};
use super::loading_state::LoadingState;

pub struct Game {
  state_stack: Vec<Box<dyn GameState>>,
}

impl Game {
  pub fn new() -> Self {
    Game {
      state_stack: vec![Box::new(LoadingState::new())],
    }
  }

  pub fn run(&mut self) {
    let mut window = Window::new((1920, 1080), "MelkonTD", Style::CLOSE, &Default::default());
    window.set_framerate_limit(60);

    while !self.state_stack.is_empty() {
      let state = self.state_stack.last_mut().unwrap();

      let transition = state.run(&mut window);
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
