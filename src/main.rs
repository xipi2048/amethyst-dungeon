mod game;

use amethyst::{
    animation::AnimationBundle,
    assets::PrefabLoaderSystem,
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use crate::game::*;

    let app_root = application_root_dir()?;

    let asset_dir = app_root.join("assets");
    let display_config_path = app_root.join("config/display.ron");

    let game_data = GameDataBuilder::default()
        .with(
            PrefabLoaderSystem::<MyPrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path).with_clear([0, 0, 0, 1]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(asset_dir, GameState::default(), game_data)?;
    game.run();

    Ok(())
}
