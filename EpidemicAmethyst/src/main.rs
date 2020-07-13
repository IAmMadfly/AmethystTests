use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{
        InputBundle,
        StringBindings
    }
};

mod systems;
mod states;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir =    app_root.join("assets");
    let config_dir =    app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_config_path = app_root.join("config").join("bindings.ron");

    // Insert bundles
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_config_path)
        .expect("Failed to get bindings file!");

    let game_data = GameDataBuilder::default()
        .with(
            systems::camera::CameraMovementSystem::default(),
            "camera_movement",
            &[]
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(input_bundle).expect("Failed to bind input bundle")
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(
        assets_dir, 
        states::game::GameState, 
        game_data
    )?;
    game.run();

    Ok(())
}
