use std::env;
use std::time::Duration;

extern crate amethyst;
extern crate pretty_env_logger;

use amethyst::{
    core::{
        transform::bundle::TransformBundle,
        frame_limiter::FrameRateLimitStrategy
    },
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA,
    },
    ui::{DrawUi, UiBundle},
};

extern crate toppa_drill_lib;
use toppa_drill_lib::{StartupState, ToppaGameDataBuilder};

fn main() -> Result<(), amethyst::Error,> {
    match env::var("RUST_LOG",) {
        Err(env::VarError::NotPresent,) => {
            env::set_var("RUST_LOG", "debug,gfx_device_gl=warn,amethyst_assets=info",);
        },
        _ => {
            env::set_var("RUST_LOG", "debug,gfx_device_gl=warn,amethyst_assets=info",);
        },
    }

    pretty_env_logger::init();

    let display_config_path = format!("{}/Prefabs/display_config.ron", env!("CARGO_MANIFEST_DIR"));
    let display_config = DisplayConfig::load(&display_config_path,);

    let input_config = format!("./Prefabs/input_bindings.ron");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0,], 1.0,)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite,),
            ),)
            .with_pass(DrawUi::new(),),
    );

    let toppa_game_data = ToppaGameDataBuilder::default()
        .with_core_bundle(
            InputBundle::<String, String,>::new().with_bindings_from_file(&input_config,)?,
        )?
        .with_core_bundle(TransformBundle::new(),)?
        .with_core_bundle(UiBundle::<String, String,>::new(),)?
        .with_core_bundle(
            RenderBundle::new(pipe, Some(display_config,),)
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system", "ui_transform",],)
                .with_hide_hierarchy_system(),
        )?;

    let mut game = Application::build("./", StartupState::new(2.0,))?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            60
        )
        .build(toppa_game_data)?;

    game.run();

    Ok((),)
}
