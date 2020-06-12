use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use std::time::Duration;

use amethyst::core::frame_limiter::FrameRateLimitStrategy;

mod game_state;
mod load_image;
mod loading_state;
mod minion;
mod minion_move_system;
mod minion_spawn_system;
mod tile_map;
mod user_input_system;
mod z_layer;
mod tower;
mod tower_update_system;
mod minion_death_system;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let input_bundle = InputBundle::<StringBindings>::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_system_desc(
            minion_spawn_system::MinionSpawnSystemDesc,
            "minion_spawn_system",
            &[],
        )
        .with_system_desc(
            user_input_system::UserInputSystemDesc,
            "user_input_system",
            &["input_system"],
        )
        .with(
            minion_move_system::MinionMoveSystem,
            "minion_move_system",
            &["input_system"],
        )
        .with(
            tower_update_system::TowerUpdateSystem,
            "tower_update_system",
            &["input_system"],
        ).with(
            minion_death_system::MinionDeathSystem,
            "minion_death_system",
            &["input_system", "tower_update_system"],
        )
        ;

    let mut game = Application::build(resources, loading_state::LoadingState)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            25,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
