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

use components::for_characters::TagPlayer;
use entities::tile::TileTypes;
use resources::{
    ingame::{
        planet::{Chunk, ChunkIndex, Planet, TileIndex},
        GameSessionData,
    },
    RenderConfig,
};

/// Serializes chunks and stores them in `.ron` format.
pub struct SerChunkSystem;

impl<'a> System<'a> for SerChunkSystem {
    type SystemData = (Read<'a, Planet>,);

    fn run(&mut self, (planet,): Self::SystemData) {}
}

/// Creates a savegame by calling different serialization systems, based on the current [GameSessionData](struct.GameSessionData.html).
/// Uses `.ron` format.
pub struct SerSavegameSystem;

impl<'a> System<'a> for SerSavegameSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, GameSessionData>,
        Read<'a, Planet>,
        Read<'a, RenderConfig>,
    );

    fn run(&mut self, (time, session_data, planet, render_config): Self::SystemData) {
        let mut commence_serializing = false;

        let save_data = &session_data.deref();
        let savegame_name = &save_data.game_name;
        let savegame_planet = &save_data.planet;

        // Directory of all savegames
        let dir_path = Path::new("E:/Workspaces/ToppaSavegame");

        // Directory of this savegame
        let mut savegame_dir_path = PathBuf::new();
        savegame_dir_path = savegame_dir_path.join(dir_path);
        savegame_dir_path = savegame_dir_path.join(Path::new(savegame_name));
        warn!("savegame_dir_path: {:?}", savegame_dir_path.clone());

        // Filepath for the serialized planet
        let mut planet_file_path = PathBuf::new();
        planet_file_path = planet_file_path.join(savegame_dir_path.clone());
        planet_file_path = planet_file_path.join(Path::new("planet"));
        planet_file_path.set_extension("ron");

        // Directory-path for the serialized chunks, need to append the individual chunks Id
        let mut chunk_dir_path = PathBuf::new();
        chunk_dir_path = chunk_dir_path.join(savegame_dir_path.clone());
        chunk_dir_path = chunk_dir_path.join(Path::new("chunks"));

        let mut dir_exists = dir_path.is_dir();
        if !dir_exists {
            if let Ok(_) = fs::create_dir(dir_path) {
                debug!("Savegame dir has been created at {:?}.", dir_path);
            } else {
                error!("Failed to create savegame dir at {:?}", dir_path);
            }
        }

        dir_exists = savegame_dir_path.exists();
        if dir_exists {
            warn!("Overwriting old savegame: {:?}.", savegame_name);
            dir_exists = chunk_dir_path.exists();
            if dir_exists {
                commence_serializing = true;
            } else {
                if let Ok(_) = fs::create_dir(chunk_dir_path.clone()) {
                    debug!(
                        "Savegame's chunk dir has been created at {:?}.",
                        chunk_dir_path.clone()
                    );
                    commence_serializing = true;
                } else {
                    error!(
                        "Failed to create savegame's chunk dir at {:?}",
                        chunk_dir_path.clone()
                    );
                }
            }
        } else {
            if let Ok(_) = fs::create_dir(savegame_dir_path.clone()) {
                dir_exists = chunk_dir_path.exists();
                if dir_exists {
                    commence_serializing = true;
                } else {
                    if let Ok(_) = fs::create_dir(chunk_dir_path.clone()) {
                        debug!(
                            "Savegame's chunk dir has been created at {:?}.",
                            chunk_dir_path.clone()
                        );

                        use std::thread;
                        thread::sleep(Duration::from_millis(1000));

                        commence_serializing = true;
                    } else {
                        error!(
                            "Failed to create savegame's chunk dir at {:?}",
                            chunk_dir_path.clone()
                        );
                    }
                }
            } else {
                error!(
                    "Failed to create savegame '{:?}'s dir at {:?}",
                    savegame_name,
                    savegame_dir_path.clone()
                );
            }
        }

        use std::{thread, time::Duration};
        thread::sleep(Duration::from_millis(500));

        if commence_serializing {
            debug!("Starting to serialize savegame.");

            let mut ser_planet = ron::ser::Serializer::new(Some(Default::default()), true);
            {
                // TODO: Error handling. Why doesn't `?` work, even though the specs example uses it?
                use serde::ser::SerializeSeq;
                debug!("serializing game data.");

                let mut serseq = ser_planet.serialize_seq(None).unwrap();
                serseq.serialize_element(&session_data.deref()).unwrap();
                serseq.end().unwrap();
            }
            // TODO: Write to file `{$savegame_name}/planet.ron`
            if let Err(e) = fs::write(planet_file_path.clone(), ser_planet.into_output_string()) {
                error!(
                    "Writing savegame planet at '{:?}' threw error: {:?}",
                    planet_file_path, e
                );
            }

            for (chunk_index, chunk) in planet.iter_chunks() {
                let mut ser_chunk = ron::ser::Serializer::new(Some(Default::default()), true);
                /* NOTE: Use this to save disk space!
                let mut ser_chunk = ron::ser::Serializer::new(Some(Default::default()), false);
                */
                {
                    use serde::ser::SerializeMap;
                    debug!("serializing chunk {:?}", chunk_index);

                    let mut serseq = ser_chunk.serialize_map(None).unwrap();
                    for (tile_index, tile_type) in chunk.iter_tiles() {
                        serseq.serialize_key::<TileIndex>(&tile_index).unwrap();
                        serseq.serialize_value::<TileTypes>(&tile_type).unwrap();
                        /* NOTE: Use this to save disk space!
                        serseq.serialize_key::<u64>(&{(tile_index.1 * render_config.chunk_render_dim.0 + tile_index.0) as u64}).unwrap();
                        serseq.serialize_value::<u64>(&{*tile_type as u64}).unwrap();
                        */
                    }
                    serseq.end().unwrap();
                }
                let mut chunk_file_path = chunk_dir_path.clone();
                chunk_file_path = chunk_file_path.join(Path::new(
                    &{ (chunk_index.1 * planet.planet_dim.0 + chunk_index.0) as u64 }.to_string(),
                ));
                chunk_file_path.set_extension("ron");

                if let Err(e) = fs::write(chunk_file_path.clone(), ser_chunk.into_output_string()) {
                    error!(
                        "Writing chunk {:?} at '{:?}' resulted in {:?}",
                        chunk_index, chunk_file_path, e
                    );
                }
            }

            debug!("Finished serializing savegame.");
        }
    }
}

/// Serializes the player entity/entities in `.ron` format.
pub struct SerPlayerSystem;

impl<'a> System<'a> for SerPlayerSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, TagPlayer>);

    fn run(&mut self, (entities, player): Self::SystemData) {}
}
