use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::transform::{
        TransformBundle
    },
    utils::application_root_dir,
    input::{
        InputBundle,
        StringBindings
    }
};

mod pong;

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
                       .with_bindings_from_file(binding_path).expect("Failed to get bindings file!");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new()).expect("Failed to bind Transform bundle!")
        .with_bundle(input_bundle).expect("Failed to bind Input bundle!")
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0])
                ).with_plugin(
                    RenderFlat2D::default()
                )
        ).expect("Failed to bind Rendering bundle!");
    let assets_dir = app_root.join("assets");

    let mut game = Application::new(assets_dir, Pong, game_data)?;

    game.run();
    /*
    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();
    */

    Ok(())
}
