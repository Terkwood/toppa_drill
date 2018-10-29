use amethyst::{
    assets::ProgressCounter,
    core::{
        transform::components::Transform,
        cgmath::Vector2,
    },
    ecs::prelude::*,
    prelude::*,
    renderer::{SpriteRender, Transparent},
    shrev::EventChannel,
};

use {
    components::{
        for_characters::{
            player::Position, 
            TagGenerator,
            Engine,
            FuelTank,
        },
        for_ground_entities::TileBase,
        physics::{
            PhysicalProperties,
            Dynamics,
        },
    },
    entities::{camera, EntitySpriteRender},
    events::planet_events::ChunkEvent,
    resources::{
        ingame::{
            planet::{ChunkIndex, TileGenerationStorages, TileIndex},
            GameSessionData, GameSprites,
        },
        RenderConfig, ToppaSpriteSheet,
    },
    utilities::{load_sprites_from_spritesheet, SpriteLoaderInfo},
};

pub fn init(world: &mut World, progress_counter_ref_opt: Option<&mut ProgressCounter>) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.
    let loader_info = SpriteLoaderInfo {
        tex_id: ToppaSpriteSheet::Player as u64,
        image_size: (128, 128),
        sprite_count: (1, 1),
        sprite_render_size: (64.0, 64.0),
    };

    if let Some(ss_handle) = load_sprites_from_spritesheet(
        world,
        "Assets/Textures/Drill.png",
        loader_info,
        progress_counter_ref_opt,
    ) {
        let mut game_sprites = world.write_resource::<GameSprites>();
        let drill_sprite = SpriteRender {
            sprite_sheet: ss_handle,
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };

        game_sprites.add(EntitySpriteRender::Player, drill_sprite);
    }
}
/// Creates a new player and returns his ID.
/// If `0`(Zero) is returned, the player has not been created.
/// Also loads the chunk the player stands on.
pub fn new(world: &mut World, transform: &Transform, sprite: &SpriteRender) -> usize {
    let player_tag = {
        let mut tag_resource = world.write_resource::<TagGenerator>();
        tag_resource.new_player_tag()
    };

    let (position_opt, view_dim) = {
        let render_config = &world.read_resource::<RenderConfig>();
        let planet = &world.read_resource::<GameSessionData>().planet;
        (
            Position::from_transform(&transform, render_config, planet),
            render_config.view_dim,
        )
    };

    let physical_properties = PhysicalProperties::new(1000.0, 1000.0, 0.3);
    let dynamics = Dynamics::default();
    let engine = Engine::new(Vector2::new(150000.0, 250000.0), 0.90, 0.0005);
    let fuel_tank = FuelTank::new(5000.0, 5000.0, 0.1);

    if let Some(position) = position_opt {
        #[cfg(feature = "debug")]
        debug!("Initial player position from transform.");

        {
            let mut chunk_event_channel = world.write_resource::<EventChannel<ChunkEvent>>();
            chunk_event_channel.single_write(ChunkEvent::RequestingLoad(position.chunk));
        }

        let player = world
            .create_entity()
            .with(transform.clone())
            .with(Transparent)
            .with(sprite.clone())
            .with(player_tag)
            .with(position)
            .with(physical_properties)
            .with(dynamics)
            .with(engine)
            .with(fuel_tank)
        .build();

        camera::init(world, view_dim, player, transform);
    } else {
        #[cfg(feature = "debug")]
        debug!("Initial player position from default.");

        let position = Position::default();
        {
            let mut chunk_event_channel = world.write_resource::<EventChannel<ChunkEvent>>();
            chunk_event_channel.single_write(ChunkEvent::RequestingLoad(position.chunk));
        }

        let player = world
            .create_entity()
            .with(transform.clone())
            .with(Transparent)
            .with(sprite.clone())
            .with(player_tag)
            .with(position)
            .with(physical_properties)
            .with(dynamics)
            .with(engine)
            .with(fuel_tank)
        .build();

        camera::init(world, view_dim, player, transform);
    }
    player_tag.id
}
