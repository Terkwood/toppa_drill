use amethyst::{
    assets::ProgressCounter,
    core::{
        cgmath::Vector3,
        transform::components::{Parent, Transform},
    },
    ecs::prelude::*,
    renderer::Transparent,
};

use components::{
    physics::PhysicalProperties,
    IsIngameEntity,
};
use entities::{EntityError, EntitySpriteRender};
use resources::{
    add_spriterender, get_spriterender, GameSprites,
    ToppaSpriteSheet,
};
use utilities::{load_sprites_from_spritesheet, SpriteLoaderInfo};

use super::PlayerParts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TracksError {
    #[allow(dead_code)]
    NotImplemented,
    MissingSpriteRender,
}

pub fn init_tracks(world: &mut World, progress_counter_ref_opt: Option<&mut ProgressCounter>) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.
    let loader_info = SpriteLoaderInfo {
        tex_id: ToppaSpriteSheet::Tracks as u64,
        image_size: (64, 16),
        sprite_count: (1, 1),
        sprite_render_size: (64.0, 16.0),
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
pub fn new_tracks(
    world: &mut World,
    parent: Entity,
) -> Result<(), EntityError> {
    #[cfg(feature = "debug")]
    debug!("Creating tracks for player {:?}.", parent);

    let sprite_render_opt =
        get_spriterender(world, EntitySpriteRender::Player(PlayerParts::Tracks));

    if let Some(sprite_render) = sprite_render_opt {
        let physical_properties = PhysicalProperties::new(500.0, None, Some(0.3), None);
        let mut transform = Transform::default();
        transform.translation += Vector3::new(0.0, -56.0, 5.0);

        world
            .create_entity()
            .with(IsIngameEntity)
            .with(Parent { entity: parent })
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
