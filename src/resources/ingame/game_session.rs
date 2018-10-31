use std::{
    fs,
};

use ron;
use serde::Serializer;

use amethyst::ecs::prelude::{Read};

use resources::RenderConfig;

use super::{
    planet::Planet,
    SavegamePaths,
};

/// Data specific to the current game,
/// gets loaded or created when the User starts a new game.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameSessionData {
    /// The name of this game, also used as the savegame's name, and should be individual each time,
    /// lest another savegame gets overwritten.
    pub game_name: &'static str,

    /// The planet is basically a container for chunks, which hold the different tiles.
    /// This enables loading and unloading areas in larger bits than single entites, helping with performance,
    /// while also sparing memory.
    pub planet: Planet,
}

impl GameSessionData {
    pub fn new(
        name: &'static str,
        planet_dim: (u64, u64),
        chunk_dim: (u64, u64),
        render_config: &RenderConfig,
    ) -> GameSessionData {
        GameSessionData {
            game_name: name,
            planet: Planet::new(planet_dim, chunk_dim, render_config),
        }
    }

    /// TODO: Error handling
    pub fn save(&self, paths: &Read<'_, SavegamePaths>) {
        #[cfg(feature = "debug")]
        debug!("| Starting to serialize savegame.");

        #[cfg(feature = "debug")]
        debug!("| Serializing game data.");

        let planet = &self.planet;

        let mut ser_planet = ron::ser::Serializer::new(Some(Default::default()), true);
        {
            use serde::ser::SerializeSeq;
            if let Ok(mut serseq) = ser_planet.serialize_seq(None) {
                if let Err(e) = serseq.serialize_element(&self) {
                    error!("| Error serializing element planet: {:?}", e);
                }
                if let Err(e) = serseq.end() {
                    error!("| Error ending serialize for planet: {:?}", e);
                }
            } else {
                error!("| Error starting serialize for planet.");
            }
        }
        // TODO: Write to file `{$savegame_name}/planet.ron`
        if let Err(e) = fs::write(
            paths.planet_file_path.clone(),
            ser_planet.into_output_string(),
        ) {
            error!(
                "| Writing savegame planet at '{:?}' threw error: {:?}",
                paths.planet_file_path.clone(),
                e
            );
        }

        #[cfg(feature = "debug")]
        debug!("| serializing chunks.");

        for (&chunk_index, _) in planet.iter_chunks() {
            planet.save_chunk(chunk_index, paths.chunk_dir_path.clone());
        }

        #[cfg(feature = "debug")]
        debug!("| Finished serializing savegame.");
    }
}
