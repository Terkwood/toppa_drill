use std::{fs, path::PathBuf};

use ron;
use serde::{ser::SerializeStruct, Serializer};

use amethyst::ecs::prelude::{Read, Write};

use crate::resources::{ingame::planet::TileGenerationStorages, RenderConfig};

use super::{planet::Planet, SavegamePaths};

/// Data specific to the current game,
/// gets loaded or created when the User starts a new game.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameSessionData {
    /// The name of this game, also used as the savegame's name, and should be individual each time,
    /// lest another savegame gets overwritten.
    pub game_name: String,

    /// The planet is basically a container for chunks, which hold the different tiles.
    /// This enables loading and unloading areas in larger bits than single entites, helping with performance,
    /// while also sparing memory.
    pub planet: Planet,
}

impl GameSessionData {
    pub fn new(
        name: String,
        planet_dim: (u64, u64,),
        chunk_dim: (u64, u64,),
        render_config: &RenderConfig,
    ) -> GameSessionData {
        GameSessionData {
            game_name: name,
            planet:    Planet::new(planet_dim, chunk_dim, render_config,),
        }
    }

    /// TODO: Error handling
    pub fn save(&self, paths: &Read<'_, SavegamePaths,>,) {
        #[cfg(feature = "debug")]
        debug!("| Starting to serialize savegame.");

        #[cfg(feature = "debug")]
        debug!("| Serializing game data.");

        let planet = &self.planet;

        let mut ser_planet = ron::ser::Serializer::new(Some(Default::default(),), true,);
        {
            if let Ok(mut serseq,) = ser_planet.serialize_struct("GameSessionData", 2,) {
                if let Err(e,) = serseq.serialize_field("game_name", &self.game_name,) {
                    error!("| Error serializing element planet: {:?}", e);
                }
                if let Err(e,) = serseq.serialize_field("planet", &self.planet,) {
                    error!("| Error serializing element planet: {:?}", e);
                }
                if let Err(e,) = serseq.end() {
                    error!("| Error ending serialize for planet: {:?}", e);
                }
            }
            else {
                error!("| Error starting serialize for planet.");
            }
        }
        // TODO: Write to file `{$savegame_name}/planet.ron`
        if let Err(e,) = fs::write(
            paths.savegame_file_path.clone(),
            ser_planet.into_output_string(),
        ) {
            error!(
                "| Writing savegame at '{:?}' threw error: {:?}",
                paths.savegame_file_path.clone(),
                e
            );
        }

        #[cfg(feature = "debug")]
        debug!("| serializing chunks.");

        for (&chunk_index, _,) in planet.iter_chunks() {
            planet.save_chunk(chunk_index, paths.chunk_dir_path.clone(),);
        }

        #[cfg(feature = "debug")]
        debug!("| Finished serializing savegame.");
    }

    /// TODO: Error handling
    pub fn load(
        savegame_file_path: PathBuf,
        render_config: &RenderConfig,
    ) -> Result<GameSessionData, (),> {
        #[cfg(feature = "debug")]
        debug!("| Starting to deserialize savegame.");

        #[cfg(feature = "debug")]
        debug!("| Deserializing game data.");

        let file = match fs::File::open(&savegame_file_path,) {
            Ok(rv,) => rv,
            Err(e,) => {
                error!(
                    "| Could not open {:?}: {:?}.",
                    savegame_file_path.clone(),
                    e
                );
                return Err((),);
            },
        };
        /*
        let dummy_data: DummyStructOne = match ron::de::from_reader(&file) {
            Ok(data) => data,
            Err(e) => {
                error!(
                    "| Error deserializing {:?}: {:?}.",
                    savegame_file_path.clone(),
                    e
                );
                return Err(());
            }
        };
        
        #[cfg(feature = "debug")]
        debug!("| Finished deserializing savegame.");
        
        Ok(
            GameSessionData{
                game_name: dummy_data.game_name,
                planet: Planet::new(
                    dummy_data.planet.planet_dim,
                    dummy_data.planet.chunk_dim,
                    render_config
                )
            }
        )
        */
        let session_data: GameSessionData = match ron::de::from_reader(&file,) {
            Ok(data,) => data,
            Err(e,) => {
                error!(
                    "| Error deserializing {:?}: {:?}.",
                    savegame_file_path.clone(),
                    e
                );
                return Err((),);
            },
        };

        #[cfg(feature = "debug")]
        debug!("| Finished deserializing savegame.");

        Ok(session_data,)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DummyStructOne {
    /// The name of this game, also used as the savegame's name, and should be individual each time,
    /// lest another savegame gets overwritten.
    pub game_name: String,

    /// The planet is basically a container for chunks, which hold the different tiles.
    /// This enables loading and unloading areas in larger bits than single entites, helping with performance,
    /// while also sparing memory.
    pub planet: DummyStructTwo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DummyStructTwo {
    /// The dimension of a planet expressed in the count of chunks in x and y direction.
    /// Differs based on the setting `Planet size` when creating a new game.
    pub planet_dim: (u64, u64,),
    // TODO: Make adjustable while playing. Requires reassigning tiles to new chunks.
    /// The dimension of a chunk expressed in tilecount in x and y direction.
    /// Cannot be changed once the game was created (at least for now).
    pub chunk_dim: (u64, u64,),
}
