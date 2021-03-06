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
    },
    ui::{RenderUi, UiBundle}
};

mod systems;
mod states;
//mod rend;
mod tools;
mod infection;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir =    app_root.join("assets");
    let config_dir =    app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    //let binding_config_path = app_root.join("config").join("bindings.ron");

    // Insert bundles
    //let input_bundle = InputBundle::<StringBindings>::new()
    //    .with_bindings_from_file(binding_config_path)
    //    .expect("Failed to get bindings file!");



    let game_data = GameDataBuilder::default()
        .with(
            systems::time::GameTimeSystem::default(),
            "game_time_system",
            &[]
        )
        .with(
            systems::camera::CameraMovementSystem::default(),
            "camera_movement_system",
            &[]
        )
        .with(
            systems::animation::SpriteAnimationSystem::default(),
            "sprite_animation_system",
            &[]
        )
        .with(
            systems::path_planning::PathPlanningSystem::default(),
            "path_planning_system",
            &[]
        )
        .with(
            amethyst::utils::ortho_camera::CameraOrthoSystem::default(),
            "ortho_camera_system",
            &[]
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.005, 0.005, 0.005, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
        )?
        .with_bundle(InputBundle::<StringBindings>::new())?
        //.with_bundle(input_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let mut game = Application::new(
        assets_dir, 
        //states::game::GameState::default(),
        states::welcome::WelcomeState::default(), 
        game_data
    ).expect("Failed to create new Game application");
    game.run();

    Ok(())
}
