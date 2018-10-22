use amethyst::{
    assets::ProgressCounter,
    core::{cgmath::Vector3, transform::components::Transform},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};

use {
    components::for_characters::{player::Position, TagGenerator, TagPlayer},
    entities::{camera, EntitySpriteRender},
    resources::{
        ingame::{GameSessionData, GameSprites},
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
pub fn new(world: &mut World, transform: &Transform, sprite: &SpriteRender) -> usize {
    let mut player_tag = TagPlayer { id: 0 };
    {
        let mut player_tag_resource = world.write_resource::<TagGenerator>();
        player_tag = player_tag_resource.new_player_tag();
    }
    if player_tag.id == 0 {
        return 0;
    }

    let mut position_opt = None;
    let mut view_dim = (960, 540);
    {
        let render_config = &world.read_resource::<RenderConfig>();
        let planet = &world.read_resource::<GameSessionData>().planet;
        position_opt = Position::from_transform(&transform, render_config, planet);
        view_dim = render_config.view_dim;
    }

    if let Some(position) = position_opt {
        let player = world
            .create_entity()
            .with(transform.clone())
            .with(Transparent)
            .with(sprite.clone())
            .with(player_tag)
            .with(position)
            .build();

        camera::init(world, view_dim, player, transform);
    } else {
        let player = world
            .create_entity()
            .with(transform.clone())
            .with(Transparent)
            .with(sprite.clone())
            .with(player_tag)
            .with(Position::default())
            .build();

        camera::init(world, view_dim, player, transform);
    }

    player_tag.id
}
