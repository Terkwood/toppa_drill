use amethyst::{
    assets::ProgressCounter,
    core::{
        cgmath::Vector3,
        transform::components::{Parent, Transform},
    },
    ecs::prelude::*,
    renderer::Transparent,
};

use components::physics::PhysicalProperties;
use entities::{EntityError, EntitySpriteRender};
use resources::{
    add_spriterender, get_spriterender, GameSprites,
    ToppaSpriteSheet,
};
use utilities::{load_sprites_from_spritesheet, SpriteLoaderInfo};

use super::PlayerParts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DrillError {
    #[allow(dead_code)]
    NotImplemented,
    MissingSpriteRender(DrillTypes),
}

/// Different types of drill provide different drilling speeds and durability
/// TODO: Make Drill retractable, retract when it is not used.
/// --: Make Drill animated.
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

pub fn init_drill(world: &mut World, progress_counter_ref_opt: Option<&mut ProgressCounter>) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.
    let loader_info = SpriteLoaderInfo {
        tex_id: ToppaSpriteSheet::Drill as u64,
        image_size: (96, 64),
        sprite_count: (3, 2),
        sprite_render_size: (32.0, 32.0),
    };

    if let Some(ss_handle) = load_sprites_from_spritesheet(
        world,
        "Assets/Textures/PlayerDrills.png",
        loader_info,
        progress_counter_ref_opt,
    ) {
        let mut game_sprites = world.write_resource::<GameSprites>();

        let sprites = [
            (
                0,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::NotImplemented)),
            ),
            (
                1,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::C45U)),
            ),
            (
                2,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::C105U)),
            ),
            (
                3,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::HS6_5_2C)),
            ),
            (
                4,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::HS6_5_2_5)),
            ),
            (
                5,
                EntitySpriteRender::Player(PlayerParts::Drill(DrillTypes::HS6_5_2_5Diamond)),
            ),
        ];

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

/// Creates a new drill associated with a player,
/// requires the player-Entity-Struct to be passed as a parameter.
pub fn new_drill(
    world: &mut World,
    parent: Entity,
    drill_type: DrillTypes,
    parent_transform: &Transform,
) -> Result<(), EntityError> {
    #[cfg(feature = "debug")]
    debug!("Creating drill for player {:?}.", parent);

    let sprite_render_opt = get_spriterender(
        world,
        EntitySpriteRender::Player(PlayerParts::Drill(drill_type)),
    );

    if let Some(sprite_render) = sprite_render_opt {
        let physical_properties = PhysicalProperties::new(250.0, None, Some(0.8), None);
        let mut transform = Transform::default();
        transform.translation += Vector3::new(22.0, -32.0, parent_transform.translation[2] - 1.0);

        world
            .create_entity()
            .with(Parent { entity: parent })
            .with(transform)
            .with(Transparent)
            .with(sprite_render)
            .with(physical_properties)
            .build();

        Ok(())
    } else {
        Err(EntityError::DrillProblem(DrillError::MissingSpriteRender(
            drill_type,
        )))
    }
}
