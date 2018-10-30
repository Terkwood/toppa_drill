#![allow(unused_imports)]
use std::{
    fs,
    ops::{Deref, DerefMut},
    path::*,
    string::ToString,
};

use ron;
use serde::Serializer;

use amethyst::{
    core::{
        specs::saveload::{
            DeserializeComponents, SerializeComponents, U64Marker, U64MarkerAllocator,
        },
        timing::Time,
    },
    ecs::prelude::*,
};

use components::for_characters::PlayerBase;
use entities::tile::TileTypes;
use resources::{
    ingame::{
        planet::{Chunk, ChunkIndex, Planet, TileIndex},
        GameSessionData, SavegamePaths,
    },
    RenderConfig,
};

/// TODO: Serialize players.
/// Creates a savegame by calling different serialization systems, based on the current [GameSessionData](struct.GameSessionData.html).
/// Uses `.ron` format.
pub struct SerSavegameSystem;

impl<'a> System<'a> for SerSavegameSystem {
    type SystemData = (
        Option<Read<'a, GameSessionData>>,
        Option<Read<'a, RenderConfig>>,
        Option<Read<'a, SavegamePaths>>,
    );

    fn run(&mut self, (session_data, render_config, paths): Self::SystemData) {
        if let (Some(session_data), Some(render_config), Some(paths)) =
            (session_data, render_config, paths)
        {
            #[cfg(feature = "debug")]
            debug!("Starting to serialize savegame.");

            #[cfg(feature = "debug")]
            debug!("Serializing game data.");

            let session_data = session_data.deref();
            let planet = &session_data.planet;

            let mut ser_planet = ron::ser::Serializer::new(Some(Default::default()), true);
            {
                use serde::ser::SerializeSeq;
                if let Ok(mut serseq) = ser_planet.serialize_seq(None) {
                    if let Err(e) = serseq.serialize_element(session_data) {
                        error!("Error serializing element planet: {:?}", e);
                    }
                    if let Err(e) = serseq.end() {
                        error!("Error ending serialize for planet: {:?}", e);
                    }
                } else {
                    error!("Error starting serialize for planet.");
                }
            }
            // TODO: Write to file `{$savegame_name}/planet.ron`
            if let Err(e) = fs::write(
                paths.planet_file_path.clone(),
                ser_planet.into_output_string(),
            ) {
                error!(
                    "Writing savegame planet at '{:?}' threw error: {:?}",
                    paths.planet_file_path.clone(),
                    e
                );
            }

            #[cfg(feature = "debug")]
            debug!("serializing planet.");

            for (&chunk_index, _) in planet.iter_chunks() {
                planet.save_chunk(chunk_index, paths.chunk_dir_path.clone());
            }

            #[cfg(feature = "debug")]
            debug!("Finished serializing savegame.");
        } else {
            error!("Resources not found.")
        }
    }
}
