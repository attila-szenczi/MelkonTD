use amethyst::{
    core::transform::TransformBundle,
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
mod z_layer;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
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
        .with(
            minion_move_system::MinionMoveSystem,
            "minion_move_system",
            &[],
        );

    let mut game = Application::build(resources, loading_state::LoadingState)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            25,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
