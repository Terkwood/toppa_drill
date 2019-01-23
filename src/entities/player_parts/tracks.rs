use amethyst::{
    assets::ProgressCounter,
    core::{
        nalgebra::Vector3,
        transform::components::{Parent, Transform},
    },
    ecs::prelude::*,
    renderer::{Transparent,Flipped},
};

use crate::{
    components::{physics::PhysicalProperties, IsIngameEntity},
    entities::{EntityError, EntitySpriteRender},
    resources::{add_spriterender, get_spriterender, GameSprites, ToppaSpriteSheet},
    utilities::{load_spritesheet, load_spritesheet_tracked},
};

use super::PlayerParts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TracksError {
    #[allow(dead_code)]
    NotImplemented,
    MissingSpriteRender,
}

pub fn init_tracks(world: &mut World, progress_counter_ref_opt: Option<&mut ProgressCounter,>,) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.

    if let Some(pc_ref) = progress_counter_ref_opt  {
        let ss_handle = load_spritesheet_tracked(
            world,
            "Assets/Textures/PlayerTracks".to_string(),
            pc_ref
        );
        let mut game_sprites = world.write_resource::<GameSprites>();

        let sprites = [(0, EntitySpriteRender::Player(PlayerParts::Tracks,),),];

        for (sprite_number, entity_sprite_render,) in sprites.iter() {
            add_spriterender(
                *entity_sprite_render,
                &mut game_sprites,
                ss_handle.clone(),
                *sprite_number,
            );
        }
    }
    else {
        let ss_handle = load_spritesheet(
            world,
            "Assets/Textures/PlayerTracks".to_string(),
        );
        let mut game_sprites = world.write_resource::<GameSprites>();

        let sprites = [(0, EntitySpriteRender::Player(PlayerParts::Tracks,),),];

        for (sprite_number, entity_sprite_render,) in sprites.iter() {
            add_spriterender(
                *entity_sprite_render,
                &mut game_sprites,
                ss_handle.clone(),
                *sprite_number,
            );
        }
    }
}

/// Creates new tracks associated with a player,
/// requires the player-Entity-Struct to be passed as a parameter.
/// TODO: Make tracks animated.
pub fn new_tracks(world: &mut World, parent: Entity,) -> Result<(), EntityError,> {
    #[cfg(feature = "debug")]
    debug!("Creating tracks for player {:?}.", parent);

    let sprite_render_opt =
        get_spriterender(world, EntitySpriteRender::Player(PlayerParts::Tracks,),);

    if let Some(sprite_render,) = sprite_render_opt {
        let physical_properties = PhysicalProperties::new(500.0, None, Some(0.3,), None,);
        let mut transform = Transform::default();
        transform.move_global(Vector3::new(0.0, 56.0, 5.0,));

        world
            .create_entity()
            .with(IsIngameEntity,)
            .with(Parent {
                entity: parent,
            },)
            .with(transform,)
            .with(Transparent,)
            .with(sprite_render,)
            .with(physical_properties,)
            .with(Flipped::Vertical) //.... why do i need this
            .build();

        Ok((),)
    }
    else {
        Err(EntityError::TracksProblem(TracksError::MissingSpriteRender,),)
    }
}
