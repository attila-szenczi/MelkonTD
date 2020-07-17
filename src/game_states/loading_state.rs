use sfml::window::Event;

use super::battle_state::BattleState;
use super::game_state_trait::{GameState, Transition};
use crate::world::World;
use sfml::graphics::RenderWindow;

pub struct LoadingState;

impl LoadingState {
  pub fn new() -> Self {
    LoadingState {}
  }
}

impl<'b> GameState for LoadingState {
  fn run(&mut self, window: &mut RenderWindow, world: &mut World) -> Transition {
    let textures = &mut world.texture_storage;
    textures.insert_with_key("background", "private_sprites/game_background_1.png");
    textures.insert("private_sprites/5_enemies_1_attack_018.png", 146);
    textures.insert("sprites/healthbar_back.png", 32);
    textures.insert("sprites/healthbar_front.png", 32);
    textures.insert("sprites/healthbar_outline.png", 32);
    textures.insert("private_sprites/electric_tower.png", 177);
    textures.insert_sprite_sheet("private_sprites/pulsing_electric_ball.png", 64, 4, 2);
    textures.insert("sprites/electric_mage_tower_icon.png", 24);
    textures.insert("sprites/locked_icon.png", 24);

    // Event processing
    while let Some(event) = window.poll_event() {
      // Request closing for the window
      if event == Event::Closed {
        window.close();
      }
    }
    return Transition::ReplaceState(Box::new(BattleState::new()));
  }
}