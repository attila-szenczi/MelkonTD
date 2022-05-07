mod animation;
mod battle_input_states;
mod flyout_actions;
mod game_state;
mod game_states;
mod hierarchy_lookup;
mod input_system;
mod load_image;
mod loading_state;
mod minion;
mod projectile;
mod render2d;
mod shared_traits;
mod simple_animation;
mod simple_animation_system;
mod texture_lookup;
mod texture_storage;
mod tile_map;
mod tower;
mod world;
mod z_layer;

fn main() -> Result<(), std::io::Error> {
  let mut game = game_states::Game::new();

  game.run();

  // amethyst::start_logger(Default::default());

  // let app_root = application_root_dir()?;

  // let resources = app_root.join("resources");
  // let display_config = resources.join("display_config.ron");
  // let game_data = GameDataBuilder::default().with_bundle(
  //   RenderingBundle::<DefaultBackend>::new()
  //     .with_plugin(
  //       RenderToWindow::from_config_path(display_config)?.with_clear([0.34, 0.36, 0.52, 1.0]),
  //     )
  //     .with_plugin(RenderFlat2D::default()),
  // )?;
  // let mut game = Application::build(resources, loading_state::LoadingState)?
  //   .with_frame_limit(
  //     FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
  //     25,
  //   )
  //   .build(game_data)?;
  //game.run();

  Ok(())
}
