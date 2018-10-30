use amethyst::{
    assets::ProgressCounter,
    core::{
        cgmath::Vector2,
        transform::components::{Parent, Transform},
    },
    ecs::prelude::*,
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle, Transparent},
    shrev::EventChannel,
};

use components::{
    for_characters::{player::Position, Engine, FuelTank, TagGenerator},
    for_ground_entities::TileBase,
    physics::{Dynamics, PhysicalProperties},
};
use entities::{camera, EntityError, EntitySpriteRender};
use events::planet_events::ChunkEvent;
use resources::{
    ingame::{
        add_spriterender, get_spriterender,
        planet::{ChunkIndex, TileGenerationStorages, TileIndex},
        GameSessionData, GameSprites,
    },
    RenderConfig, ToppaSpriteSheet,
};
use utilities::{load_sprites_from_spritesheet, SpriteLoaderInfo};

use super::PlayerParts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TracksError {
    NotImplemented,
    NoPositionFromTransform,
    MissingSpriteRender,
}

pub fn init_tracks(world: &mut World, progress_counter_ref_opt: Option<&mut ProgressCounter>) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.
    let loader_info = SpriteLoaderInfo {
        tex_id: ToppaSpriteSheet::Drill as u64,
        image_size: (128, 128),
        sprite_count: (3, 2),
        sprite_render_size: (64.0, 64.0),
    };

    if let Some(ss_handle) = load_sprites_from_spritesheet(
        world,
        "Assets/Textures/PlayerTracks.png",
        loader_info,
        progress_counter_ref_opt,
    ) {
        let mut game_sprites = world.write_resource::<GameSprites>();

        let sprites = [(0, EntitySpriteRender::Player(PlayerParts::Tracks))];

        for (sprite_number, entity_sprite_render) in sprites.iter() {
            add_spriterender(
                *entity_sprite_render,
                &mut game_sprites,
                ss_handle.clone(),
                *sprite_number,
                false,
                false,
            );
        }
    }
}

/// Creates new tracks associated with a player,
/// requires the player-Entity-Struct to be passed as a parameter.
/// TODO: Make tracks animated.
pub fn new_tracks(world: &mut World, parent: Entity) -> Result<(), EntityError> {
    #[cfg(feature = "debug")]
    debug!("Creating tracks for player {:?}.", parent);

    let sprite_render_opt =
        get_spriterender(world, EntitySpriteRender::Player(PlayerParts::Tracks));

    if let Some(sprite_render) = sprite_render_opt {
        let physical_properties = PhysicalProperties::new(500.0, None, Some(0.3), None);
        let transform = Transform::default();

        let drill = world
            .create_entity()
            .with(transform)
            .with(Transparent)
            .with(sprite_render)
            .with(physical_properties)
            .build();

        Ok(())
    } else {
        Err(EntityError::TracksProblem(TracksError::MissingSpriteRender))
    }
}
