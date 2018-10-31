use std::{
    fs,
    ops::{Deref, DerefMut},
    path::*,
    string::ToString,
    vec::Vec,
};

use ron;
use serde::Serializer;

use amethyst::{
    core::{timing::Time, transform::components::Transform},
    ecs::prelude::*,
    renderer::SpriteRender,
    shred::Resources,
    shrev::EventChannel,
};

use components::{for_characters::PlayerBase, for_ground_entities::TileBase};
use entities::tile::TileTypes;
use events::planet_events::ChunkEvent;
use resources::{
    ingame::{
        planet::{Chunk, ChunkIndex, Planet, TileGenerationStorages, TileIndex},
        GameSessionData, GameSprites, SavegamePaths,
    },
    RenderConfig,
};

/// TODO: Everything.
#[allow(dead_code)]
pub struct DeSavegameSystem;

impl<'a> System<'a> for DeSavegameSystem {
    type SystemData = (
        Option<Write<'a, GameSessionData>>,
        Option<Read<'a, SavegamePaths>>,
    );

    fn run(&mut self,(
            session_data,
            paths,
        ): Self::SystemData,
    ) {
        #[cfg(feature = "debug")]
        debug!("+------------");

        match (session_data, paths) {
            (Some(mut session_data), Some(paths)) => {
                match GameSessionData::load(&paths){
                    Ok(data) => {
                        let mut buffer = session_data.deref_mut();
                        *buffer = data;
                    },
                    Err(e) => {
                        error!("Error loading savegame data at {:?}: {:?}", paths.savegame_file_path.clone(), e);
                    }
                }
            },
            (None, Some(paths)) => {
                match GameSessionData::load(&paths){
                    Ok(data) => {
                        /*
                        let mut buffer = session_data;
                        buffer = Some(data);
                        */
                        error!("Found savegame data at {:?}, but cannot add resource from inside system.", paths.savegame_file_path.clone());
                    },
                    Err(e) => {
                        error!("Error loading savegame data at {:?}: {:?}", paths.savegame_file_path.clone(), e);
                    }
                }
            },
            _ => {
                error!("| SavegamePaths not found.");
            }
        }

        #[cfg(feature = "debug")]
        debug!("+------------");
    }
}
