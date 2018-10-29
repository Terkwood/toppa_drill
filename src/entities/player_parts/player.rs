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
    entities::{
        camera, 
        EntitySpriteRender,
        EntityError,
    },
    events::planet_events::ChunkEvent,
    resources::{
        ingame::{
            planet::{ChunkIndex, TileGenerationStorages, TileIndex},
            GameSessionData, GameSprites, add_spriterender, get_spriterender,
        },
        RenderConfig, ToppaSpriteSheet,
    },
    utilities::{load_sprites_from_spritesheet, SpriteLoaderInfo,},
};

use super::PlayerParts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerError {
    NotImplemented,
    NoPositionFromTransform,
    MissingSpriteRender(ShipTypes),
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

pub fn init(world: &mut World, progress_counter_ref_opt: Option<&mut ProgressCounter>) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.
    let loader_info = SpriteLoaderInfo {
        tex_id: ToppaSpriteSheet::Player as u64,
        image_size: (128, 128),
        sprite_count: (2, 2),
        sprite_render_size: (64.0, 64.0),
    };

    if let Some(ss_handle) = load_sprites_from_spritesheet(
        world,
        "Assets/Textures/Drill.png",
        loader_info,
        progress_counter_ref_opt,
    ) {
        let mut game_sprites = world.write_resource::<GameSprites>();
        
        let sprites = [
            (0, EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::NotImplemented))),
            (1, EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::Mk1506))),
            (2, EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::Albatros))),
            (3, EntitySpriteRender::Player(PlayerParts::Ship(ShipTypes::L14Ultra))),
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
/// Creates a new player and returns his ID.
/// If `0`(Zero) is returned, the player has not been created.
/// Also loads the chunk the player stands on.
pub fn new(world: &mut World, transform: &Transform, ship_type: ShipTypes) -> Result<(), EntityError> {
    #[cfg(feature = "debug")]
    debug!("Creating player with ship type {:?}.", ship_type);

    let sprite_render_opt = get_spriterender(world, EntitySpriteRender::Player(PlayerParts::Ship(ship_type)));

    if let Some(sprite_render) = sprite_render_opt {
        let player_tag = {
            let mut tag_resource = world.write_resource::<TagGenerator>();
            tag_resource.new_player_tag()
        };
        let (position_opt, view_dim, chunk_render_distance) = {
            let ren_con = &world.read_resource::<RenderConfig>();
            let planet = &world.read_resource::<GameSessionData>().planet;
            (
                Position::from_transform(&transform, ren_con, planet),
                ren_con.view_dim,
                ren_con.chunk_render_distance,
            )
        };

        let physical_properties = PhysicalProperties::new(1000.0, Some(1000.0), None, Some(0.6));
        let dynamics = Dynamics::default();
        let engine = Engine::new(Vector2::new(150000.0, 250000.0), 0.90, 0.0005);
        let fuel_tank = FuelTank::new(5000.0, 5000.0, 0.1);

        if let Some(position) = position_opt {
            #[cfg(feature = "debug")]
            debug!("Initial player position from transform.");

            {
                let mut chunk_event_channel = world.write_resource::<EventChannel<ChunkEvent>>();
                
                use std::u64;
                //dealing with over- and underflow
                let lower_y = {
                    if position.chunk.0 >= chunk_render_distance{
                        position.chunk.0 - chunk_render_distance
                    }
                    else{
                        0
                    }
                };
                let lower_x = {
                    if position.chunk.1 >= chunk_render_distance{
                        position.chunk.1 - chunk_render_distance
                    }
                    else{
                        // TODO: World-wrapping
                        0
                    }
                };
                
                let upper_y = {
                    let buff = position.chunk.0 + chunk_render_distance;
                    if buff >= position.chunk.0 {
                        buff
                    }
                    else{
                        u64::MAX
                    }
                };
                let upper_x = {
                    let buff = position.chunk.1 + chunk_render_distance;
                    if buff >= position.chunk.1 {
                        buff
                    }
                    else{
                        // TODO: World-wrapping
                        u64::MAX
                    }
                };

                for y in lower_y..=upper_y
                {
                    for x in lower_x..=upper_x
                    {
                        chunk_event_channel.single_write(ChunkEvent::RequestingLoad(ChunkIndex(y, x)));
                    }
                }
            }

            let player = world
                .create_entity()
                .with(transform.clone())
                .with(Transparent)
                .with(sprite_render)
                .with(player_tag)
                .with(position)
                .with(physical_properties)
                .with(dynamics)
                .with(engine)
                .with(fuel_tank)
            .build();

            camera::init(world, view_dim, player, transform);
            Ok(())
        } else {
            Err(EntityError::PlayerProblem(PlayerError::NoPositionFromTransform))
        }
    }
    else{
        Err(EntityError::PlayerProblem(PlayerError::MissingSpriteRender(ship_type)))
    }
}
