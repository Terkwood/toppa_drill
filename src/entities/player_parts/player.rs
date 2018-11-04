use amethyst::{
    assets::ProgressCounter,
    core::{cgmath::Vector2, transform::components::Transform},
    ecs::prelude::*,
    renderer::Transparent,
    shrev::EventChannel,
};

use crate::{
    components::{
        for_characters::{player::Position, Engine, FuelTank, TagGenerator},
        physics::{Dynamics, PhysicalProperties},
        IsIngameEntity,
    },
    entities::{camera, player_parts::DrillTypes, EntityError, EntitySpriteRender},
    events::planet_events::ChunkEvent,
    resources::{
        add_spriterender, get_spriterender, ingame::GameSessionData, GameSprites, RenderConfig,
        ToppaSpriteSheet,
    },
    utilities::{load_sprites_from_spritesheet, SpriteLoaderInfo},
};

use super::{init_drill, init_tracks, new_drill, new_tracks, PlayerParts};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerError {
    #[allow(dead_code)]
    NotImplemented,
    NoPositionFromTransform,
    MissingSpriteRender(ShipTypes,),
}

/// The hull of the ship, provides resistance against forces and impacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ShipTypes {
    /// Dummy, if a drill has no implementation yet.
    NotImplemented,
    /// Base-model every player starts with, low resistance against all.
    Mk1506,
    /// Mid tier model with moderate resistance against heat, forces and impact.
    Albatros,
    /// Highest end model, providing the highest resistance against heat, forces and impact.
    L14Ultra,
}

/// TODO: Error handling
/// Loads the spritesheet and sprites for the player, adding them to GameSprites.
/// Calls the `init`-functions of all the player sub-entities, like the drill and tracks.
pub fn init_player(world: &mut World, progress_counter_ref_opt: Option<&mut ProgressCounter,>,) {
    // TODO: Not happy with this if-let for duplicating an Option<&mut ProgressCounter>
    if let Some(progress_counter_ref,) = progress_counter_ref_opt {
        // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.
        let loader_info = SpriteLoaderInfo {
            tex_id:             ToppaSpriteSheet::Player as u64,
            image_size:         (128, 128,),
            sprite_count:       (1, 1,),
            sprite_render_size: (64.0, 64.0,),
        };

        if let Some(ss_handle,) = load_sprites_from_spritesheet(
            world,
            "Assets/Textures/Drill.png",
            loader_info,
            Some(progress_counter_ref,),
        ) {
            let mut game_sprites = world.write_resource::<GameSprites>();

            let sprites = [
                (
                    0,
                    EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::NotImplemented,),),
                ), /*
                   (
                       1,
                       EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::Mk1506)),
                   ),
                   (
                       2,
                       EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::Albatros)),
                   ),
                   (
                       3,
                       EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::L14Ultra)),
                   ),*/
            ];

            for (sprite_number, entity_sprite_render,) in sprites.iter() {
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
        init_drill(world, Some(progress_counter_ref,),);
        init_tracks(world, Some(progress_counter_ref,),);
    }
    else {
        let loader_info = SpriteLoaderInfo {
            tex_id:             ToppaSpriteSheet::Player as u64,
            image_size:         (128, 128,),
            sprite_count:       (1, 1,),
            sprite_render_size: (64.0, 64.0,),
        };

        if let Some(ss_handle,) =
            load_sprites_from_spritesheet(world, "Assets/Textures/Drill.png", loader_info, None,)
        {
            let mut game_sprites = world.write_resource::<GameSprites>();

            let sprites = [
                (
                    0,
                    EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::NotImplemented,),),
                ), /*
                   (
                       1,
                       EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::Mk1506)),
                   ),
                   (
                       2,
                       EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::Albatros)),
                   ),
                   (
                       3,
                       EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::L14Ultra)),
                   ),*/
            ];

            for (sprite_number, entity_sprite_render,) in sprites.iter() {
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
        init_drill(world, None,);
        init_tracks(world, None,);
    }
}
/// Creates a new player and returns his ID.
/// If `0`(Zero) is returned, the player has not been created.
/// Also loads the chunk the player stands on.
pub fn new_player(
    world: &mut World,
    transform: &Transform,
    ship_type: ShipTypes,
) -> Result<(), EntityError,> {
    #[cfg(feature = "debug")]
    debug!("Creating player with ship type {:?}.", ship_type);

    let sprite_render_opt = get_spriterender(
        world,
        EntitySpriteRender::Player(PlayerParts::Ship(ship_type,),),
    );

    if let Some(sprite_render,) = sprite_render_opt {
        let player_tag = {
            let mut tag_resource = world.write_resource::<TagGenerator>();
            tag_resource.new_player_tag()
        };
        let (position, view_dim, _chunk_render_distance,) = {
            let ren_con = &world.read_resource::<RenderConfig>();
            let planet = &world.read_resource::<GameSessionData>().planet;
            (
                Position::default(),
                ren_con.view_dim,
                ren_con.chunk_render_distance,
            )
        };

        let physical_properties = PhysicalProperties::new(7000.0, Some(1000.0,), None, Some(125.0,),);
        let dynamics = Dynamics::default();
        let engine = Engine::new(Vector2::new(7200000.0, 4260000.0), 0.90, 0.0001,);
        let fuel_tank = FuelTank::new(50000.0, 50000.0, 0.002);

        #[cfg(feature = "debug")]
        debug!("| Initial player position from transform.");

        let player = world
            .create_entity()
            .with(IsIngameEntity,)
            .with(transform.clone(),)
            .with(Transparent,)
            .with(sprite_render,)
            .with(player_tag,)
            .with(position,)
            .with(physical_properties,)
            .with(dynamics,)
            .with(engine,)
            .with(fuel_tank,)
            .build();

        camera::init(world, view_dim, player,);
        new_drill(world, player, DrillTypes::C45U,)?;
        new_tracks(world, player,)?;
        Ok((),)
    }
    else {
        Err(EntityError::PlayerProblem(
            PlayerError::MissingSpriteRender(ship_type,),
        ),)
    }
}
