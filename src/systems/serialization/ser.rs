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
        if let (Some(session_data), Some(render_config), Some(paths)) = (session_data, render_config, paths) {
            let save_data = &session_data.deref();
            let savegame_planet = &save_data.planet;

            #[cfg(feature = "debug")]
            warn!("Starting to serialize savegame.");
            #[cfg(feature = "debug")]
            warn!("serializing game data.");
            let mut ser_planet = ron::ser::Serializer::new(Some(Default::default()), true);
            {
                // TODO: Error handling. Why doesn't `?` work, even though the specs example uses it?
                use serde::ser::SerializeSeq;
                if let Ok(mut serseq) = ser_planet.serialize_seq(None) {
                    if let Err(e) = serseq.serialize_element(&session_data.deref()) {
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
            warn!("serializing planet.");
            for (chunk_index, chunk) in savegame_planet.iter_chunks() {
                let mut ser_chunk = ron::ser::Serializer::new(Some(Default::default()), true);
                /* NOTE: Use this to save disk space!
                let mut ser_chunk = ron::ser::Serializer::new(Some(Default::default()), false);
                */
                {
                    use serde::ser::SerializeMap;
                    #[cfg(feature = "debug")]
                    warn!("serializing {:?}", chunk_index);

                    if let Ok(mut serseq) = ser_chunk.serialize_map(None) {
                        for (tile_index, tile_type) in chunk.iter_tiles() {
                            if let Err(e) = serseq.serialize_key::<TileIndex>(&tile_index) {
                                error!(
                                    "Error serializing key of Tile {:?} in Chunk {:?}: {:?}",
                                    tile_index, chunk_index, e
                                );
                            }
                            if let Err(e) = serseq.serialize_value::<TileTypes>(&tile_type) {
                                error!(
                                    "Error serializing value of Tile {:?} in Chunk {:?}: {:?}",
                                    tile_index, chunk_index, e
                                );
                            }
                            /* NOTE: Use this to save disk space!
                            serseq.serialize_key::<u64>(&{(tile_index.1 * render_config.chunk_render_dim.0 + tile_index.0) as u64}).unwrap();
                            serseq.serialize_value::<u64>(&{*tile_type as u64}).unwrap();
                            */
                        }
                        if let Err(e) = serseq.end() {
                            error!(
                                "Error ending serialize for chunk {:?}: {:?}",
                                chunk_index, e
                            );
                        }
                    } else {
                        error!("Error starting serialize for chunk {:?}.", chunk_index);
                    }
                }
                let mut chunk_file_path = paths.chunk_dir_path.clone();
                chunk_file_path = chunk_file_path.join(Path::new(
                    &{ (chunk_index.1 * savegame_planet.planet_dim.0 + chunk_index.0) as u64 }
                        .to_string(),
                ));
                chunk_file_path.set_extension("ron");

                if let Err(e) = fs::write(chunk_file_path.clone(), ser_chunk.into_output_string()) {
                    error!(
                        "Writing chunk {:?} at '{:?}' resulted in {:?}",
                        chunk_index, chunk_file_path, e
                    );
                }
            }
            #[cfg(feature = "debug")]
            warn!("Finished serializing savegame.");
        }
        else{
            error!("Resources not found.")
        }
    }
}
