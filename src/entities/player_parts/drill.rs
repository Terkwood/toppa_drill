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
pub enum DrillError {
    #[allow(dead_code)]
    NotImplemented,
    MissingSpriteRender(DrillTypes,),
}

/// Different types of drill provide different drilling speeds and durability
/// TODO: Make Drill retractable, retract when it is not used.
/// TODO: Make Drill animated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DrillTypes {
    /// Dummy, if a drill has no implementation yet
    #[allow(dead_code)]
    NotImplemented,
    /// Cold work steel, lowest performance
    C45U,
    /// Cold work steel, very low performance
    C105U,
    /// Speed steel, low performance
    HS6_5_2C,
    /// Speed steel, high performance
    HS6_5_2_5,
    /// High Speed steel, highest performance
    HS6_5_2_5Diamond,
}

pub fn init_drill(world: &mut World, progress_counter_ref_opt: Option<&mut ProgressCounter,>,) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.

    if let Some(pc_ref) = progress_counter_ref_opt {
        let ss_handle = load_spritesheet_tracked(
            world,
            "Assets/Textures/PlayerDrills".to_string(),
            pc_ref,
        );
        let mut game_sprites = world.write_resource::<GameSprites>();

        let sprites = [
            (
                0,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::NotImplemented,),),
            ),
            (
                1,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::C45U,),),
            ),
            (
                2,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::C105U,),),
            ),
            (
                3,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::HS6_5_2C,),),
            ),
            (
                4,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::HS6_5_2_5,),),
            ),
            (
                5,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::HS6_5_2_5Diamond,),),
            ),
        ];

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
            "Assets/Textures/PlayerDrills".to_string(),
        );
        let mut game_sprites = world.write_resource::<GameSprites>();

        let sprites = [
            (
                0,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::NotImplemented,),),
            ),
            (
                1,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::C45U,),),
            ),
            (
                2,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::C105U,),),
            ),
            (
                3,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::HS6_5_2C,),),
            ),
            (
                4,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::HS6_5_2_5,),),
            ),
            (
                5,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::HS6_5_2_5Diamond,),),
            ),
        ];

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

/// Creates a new drill associated with a player,
/// requires the player-Entity-Struct to be passed as a parameter.
pub fn new_drill(
    world: &mut World,
    parent: Entity,
    drill_type: DrillTypes,
) -> Result<(), EntityError,> {
    #[cfg(feature = "debug")]
    debug!("Creating drill for player {:?}.", parent);

    let sprite_render_opt = get_spriterender(
        world,
        EntitySpriteRender::Player(PlayerParts::Drill(drill_type,),),
    );

    if let Some(sprite_render,) = sprite_render_opt {
        let physical_properties = PhysicalProperties::new(250.0, None, Some(0.8,), None,);
        let mut transform = Transform::default();
        transform.move_global(Vector3::new(22.0, 32.0, -1.0,));

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
            .with(Flipped::Vertical) //.... why do i need this?
            .build();

        Ok((),)
    }
    else {
        Err(EntityError::DrillProblem(DrillError::MissingSpriteRender(
            drill_type,
        ),),)
    }
}
