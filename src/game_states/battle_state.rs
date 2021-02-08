use super::game_state_trait::{GameState, Transition};
use sfml::graphics::RenderWindow;
use sfml::system::Clock;
use sfml::window::Event;

use crate::battle_input_states::BattleUserInputHandler;
use crate::flyout_actions::*;
use crate::minion::{update_minions, MinionSpawner};
use crate::projectile::update_projectiles;
use crate::render2d::draw_world;
use crate::shared_traits::*;
use crate::tower::update_towers;
use crate::world::World;

use generational_arena::Arena;

pub struct BattleState {
  minion_spawner: MinionSpawner,
  clock: Clock,
  input_state_handler: BattleUserInputHandler,
}

impl BattleState {
  pub fn new() -> Self {
    BattleState {
      minion_spawner: MinionSpawner::new(),
      clock: Clock::start(), //TODO: Check std clock or something
      input_state_handler: BattleUserInputHandler::new(),
    }
  }
}

impl<'b> GameState for BattleState {
  fn run(&mut self, window: &mut RenderWindow, world: &mut World) -> Transition {
    let elapsed = self.clock.restart().as_seconds();
    self.clock.restart();
    self.minion_spawner.update(world);
    update_towers(world, elapsed);
    update_projectiles(world, elapsed);
    update_minions(&mut world.minions, elapsed);
    update_active_flyout_actions(&mut world.active_flyout_actions, elapsed);

    remove_dead(&mut world.towers);
    remove_dead(&mut world.projectiles);
    remove_dead(&mut world.minions);
    world.active_flyout_actions.retain(|elem| !elem.dead());

    draw_world(window, world);

    while let Some(event) = window.poll_event() {
      match event {
        Event::Closed => {
          window.close();
          return Transition::Quit;
        }
        _ => {
          self.input_state_handler.process_event(&event, world);
          ()
        }
      }
    }

    Transition::KeepState
  }
}

fn remove_dead<T: ?Sized + MortalTrait>(container: &mut Arena<Box<T>>) {
  container.retain(|_i, elem| !elem.dead());
}

fn update_active_flyout_actions(active_flyout_actions: &mut Vec<Box<FlyoutAction>>, elapsed: f32) {
  for action in active_flyout_actions {
    action.update(elapsed);
  }
}
