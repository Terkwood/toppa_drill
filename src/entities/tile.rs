use amethyst::{
    assets::ProgressCounter,
    core::{
        cgmath::Vector3,
        transform::components::{Transform},
    },
    prelude::*,
    renderer::{SpriteRender, Transparent},
};

use utilities::{load_sprites_from_spritesheet, SpriteLoaderInfo};
use resources::{
    ToppaSpriteSheet,
    ingame::{
        chunk::TileIndex,
        planet::ChunkIndex,
    },
};
use components::for_ground_entities::TileTypes;

/// Should run in a State and only once.
/// Loads the spritesheet from the hardcoded path "Assets/Textures/Ores.png"
/// and calculates the sprite-vec.
pub fn prepare_spritesheet(world: &mut World, progress_counter_ref: &mut ProgressCounter) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.
    let loader_info = SpriteLoaderInfo {
        tex_id: ToppaSpriteSheet::Tiles as u64,
        image_size: (128, 128),
        sprite_count: (4, 4),
        sprite_render_size: (64.0, 64.0),
    };

    if let Some(ss_handle) = load_sprites_from_spritesheet(
        world,
        "Assets/Textures/Ores.png",
        loader_info,
        progress_counter_ref,
    ){
        
    }
}

pub fn create_ore_with_world(world: &mut World, chunk: ChunkIndex, index: TileIndex, tiletype: TileTypes){
    // TODO: Get spriteSheetHandle from texture id.
    // TODO: Get sprite from TileType
    // TODO: Calculate transform based on TileIndex & ChunkIndex
    /*
    let drill_sprite = SpriteRender {
        sprite_sheet: ss_handle,
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };
    let mut transform = Transform::default();
    transform.translation = Vector3::new(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(transform)
        .with(Transparent)
        .with(drill_sprite)
        .build();
    */
    error!("Not implemented yet.");
}
