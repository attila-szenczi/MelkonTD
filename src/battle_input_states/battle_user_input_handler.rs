use sfml::window::Event;

use super::default_battle_input_state::DefaultBattleInputState;
use super::input_state_trait::*;
use crate::world::World;

pub struct BattleUserInputHandler {
  state_stack: Vec<Box<dyn InputStateTrait>>,
}

impl BattleUserInputHandler {
  pub fn new() -> Self {
    BattleUserInputHandler {
      state_stack: vec![Box::new(DefaultBattleInputState {})],
    }
  }

  pub fn process_event(&mut self, event: &Event, world: &mut World) {
    if !self.state_stack.is_empty() {
      let state = self.state_stack.last_mut().unwrap();

      let transition = state.process_event(event, world);
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
