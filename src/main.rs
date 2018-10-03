extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{
        DisplayConfig, Pipeline,
        RenderBundle, Stage, 
        DrawSprite, ColorMask, ALPHA, DepthMode,
    },
    core::{
        transform::bundle::TransformBundle,
    },
    ui::{
        DrawUi, UiBundle,
    },
    input::InputBundle,
};

extern crate toppa_drill_lib;
use toppa_drill_lib::{
    StartupState,
    ToppaGameDataBuilder,
};


fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let display_config_path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let display_config = DisplayConfig::load(&display_config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(
                DrawSprite::new()
                    .with_transparency(
                        ColorMask::all(), 
                        ALPHA, 
                        Some(DepthMode::LessEqualWrite)
                    ),
            )
            .with_pass(
                DrawUi::new()
            ),
    );

    let toppa_game_data = ToppaGameDataBuilder::default()
        .with_core_bundle(InputBundle::<String, String>::new())?
        .with_core_bundle(TransformBundle::new())?
        .with_core_bundle(UiBundle::<String, String>::new())?
        .with_core_bundle(
            RenderBundle::new(
                pipe,
                Some(display_config)
            )
            .with_sprite_sheet_processor()
            .with_sprite_visibility_sorting(&["transform_system"]),
        )?;

    let mut game = Application::new("./", StartupState::new(), toppa_game_data)?;

    game.run();

    Ok(())
}
