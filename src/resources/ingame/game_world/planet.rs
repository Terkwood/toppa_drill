use std::{
    collections::{hash_map, BTreeMap, HashMap},
    fs,
    path::*,
};

use ron;
use serde::Serializer;

use amethyst::{
    core::{cgmath::Vector3, transform::components::Transform},
    ecs::{prelude::*, world::EntitiesRes,},
    shred::DefaultProvider,
};

use crate::{
    entities::tile::TileTypes,
    resources::RenderConfig,
};

use super::{
    ChunkIndex, Chunk, GameWorldError, TileIndex, ChunkError, TileGenerationStorages, 
};

/// This is a resource.
/// The planet a player resides on.
/// Consists of individual chunks of tile entities.
#[derive(Debug, Serialize, Deserialize)]
pub struct Planet {
    /// The dimension of a planet expressed in the count of chunks in x and y direction.
    /// Differs based on the setting `Planet size` when creating a new game.
    pub planet_dim: (u64, u64),
    /// TODO: Make adjustable while playing. Requires reassigning tiles to new chunks.
    /// The dimension of a chunk expressed in tilecount in x and y direction.
    /// Cannot be changed once the game was created (at least for now).
    pub chunk_dim: (u64, u64),
    // A map of individual chunks of the planet, only a small number is loaded at a time.
    // Chunks that are too far from the player get serialized and stored to the disk.
    // Private to prevent users from meddling with it.
    #[serde(skip_serializing, default = "serde_de_empty_hash_map")]
    chunks: HashMap<ChunkIndex, Chunk>,
}

pub fn serde_de_empty_hash_map() -> HashMap<ChunkIndex, Chunk> {HashMap::with_capacity(9)}

// public interface
impl Planet {
    pub fn new(
        planet_dim: (u64, u64),
        chunk_dim: (u64, u64),
        render_config: &RenderConfig,
    ) -> Planet {
        // Chunk of the player + render distance in two directions (left+right | top+bottom)
        let chunk_count_per_direction = 1 + 2 * render_config.chunk_render_distance;
        let chunk_count = chunk_count_per_direction * chunk_count_per_direction;
        Planet {
            planet_dim,
            chunk_dim,
            chunks: HashMap::with_capacity(chunk_count as usize),
        }
    }

    /// Tries to fetch a chunk from the HashMap.
    /// If the given index exceeds the planet-dim bounds, it gets [clamped](struct.Planet.html#method.clamp_chunk_index).
    /// Returns either a reference to a chunk, if it found one, or an error.
    /// This Error could be `GameWorldError::ChunkProblem(ChunkError::NotFound))`,
    /// if that's the case try calling `new_chunk()`.
    #[allow(dead_code)]
    pub fn get_chunk(&self, index: ChunkIndex) -> Result<Option<&Chunk>, GameWorldError> {
        let clamped_id_result = Self::clamp_chunk_index(&self, index);
        match clamped_id_result {
            Ok(clamped_index) => Ok(self.chunks.get(&clamped_index)),
            Err(e) => {
                warn!("| Error clamping chunk index: {:?}", e);
                Err(GameWorldError::ChunkProblem(ChunkError::NotFound))
            }
        }
    }

    /// Tile indexes out of tile-dim bounds return `None`.
    #[allow(dead_code)]
    pub fn get_tiletype(_chunk: ChunkIndex, _tile: TileIndex) -> Option<TileTypes> {
        // TODO:
        error!("Not implemented yet.");
        None
    }

    /// The given chunk index gets clamped to the planet-dim by wrapping it in x-direction.
    /// Returns none if the index is out of bounds in y-direction.
    pub fn clamp_chunk_index(
        planet: &Planet,
        index: ChunkIndex,
    ) -> Result<ChunkIndex, GameWorldError> {
        #[cfg(feature = "trace")]
        trace!("clamping: {}, planet_dim.0: {}", index, planet.planet_dim.0);

        let mut rv = index;
        if rv.0 >= planet.planet_dim.0 {
            Err(GameWorldError::ChunkProblem(ChunkError::IndexOutOfBounds))
        } else {
            if rv.1 >= planet.planet_dim.1 {
                #[cfg(feature = "debug")]
                let buff = rv.1;
                rv.1 = rv.1 % planet.planet_dim.1;
                #[cfg(feature = "debug")]
                debug!("| Wrapping index from {:?} to {:?}.", buff, rv.1);
            }
            Ok(rv)
        }
    }

    /// Saves a chunk in the specified directory without removing it from the planet.
    /// TODO: Save with less space-usage
    pub fn save_chunk(&self, chunk_id: ChunkIndex, chunk_dir_path: PathBuf) {
        let planet_width_in_chunks = self.planet_dim.0;
        match self.get_chunk(chunk_id) {
            Ok(chunk_opt) => {
                if let Some(chunk) = chunk_opt {
                    let mut ser_chunk = ron::ser::Serializer::new(Some(Default::default()), true);
                    //NOTE: Use this to save disk space!
                    //let mut ser_chunk = ron::ser::Serializer::new(Some(Default::default()), false);
                    {
                        use serde::ser::SerializeMap;
                        #[cfg(feature = "debug")]
                        debug!("| serializing {:?}", chunk_id);

                        if let Ok(mut serseq) = ser_chunk.serialize_map(None) {
                            for (tile_index, tile_type) in chunk.iter_tiletypes() {
                                if let Err(e) = serseq.serialize_key::<TileIndex>(&tile_index) {
                                    error!(
                                        "| Error serializing key of Tile {:?} in Chunk {:?}: {:?}",
                                        tile_index, chunk_id, e
                                    );
                                }
                                if let Err(e) = serseq.serialize_value::<TileTypes>(&tile_type) {
                                    error!(
                                        "| Error serializing value of Tile {:?} in Chunk {:?}: {:?}",
                                        tile_index, chunk_id, e
                                    );
                                }                           }
                            if let Err(e) = serseq.end() {
                                error!(
                                    "| Error ending serialize for chunk {:?}: {:?}",
                                    chunk_id, e
                                );
                            }
                        } else {
                            error!("| Error starting serialize for chunk {:?}.", chunk_id);
                        }
                    }

                    let mut chunk_file_path = chunk_dir_path.clone();
                    chunk_file_path = chunk_file_path.join(Path::new(
                        &{ (chunk_id.1 * planet_width_in_chunks + chunk_id.0) as u64 }.to_string(),
                    ));
                    chunk_file_path.set_extension("ron");

                    if let Err(e) =
                        fs::write(chunk_file_path.clone(), ser_chunk.into_output_string())
                    {
                        error!(
                            "| Writing chunk {:?} at '{:?}' resulted in {:?}",
                            chunk_id, chunk_file_path, e
                        );
                    }
                } else {
                    #[cfg(feature = "debug")]
                    debug!("| Removing {:?} failed, since it was not found.", chunk_id);
                }
            }
            Err(e) => {
                error!("| Error getting chunk from planet: {:?}.", e);
            }
        }
    }

    /// TODO: When save_chunk uses less-space-variant, custom deserializer needed.
    pub fn load_chunk(
        &mut self,
        chunk_id: ChunkIndex,
        chunk_file_path: PathBuf,
        storages: &mut TileGenerationStorages<'_>,
    ) {
        #[cfg(feature = "debug")]
        debug!("+------------");
        #[cfg(feature = "debug")]
        debug!("| chunk_file_path: {:?}", chunk_file_path);

        let file = match fs::File::open(&chunk_file_path) {
            Ok(rv) => rv,
            Err(e) => {
                error!("| Could not open {:?}: {:?}.", chunk_file_path.clone(), e);
                return;
            }
        };

        let tiles: BTreeMap<TileIndex, TileTypes> = match ron::de::from_reader(&file) {
            Ok(tiles) => tiles,
            Err(e) => {
                error!(
                    "| Error deserializing {:?}: {:?}.",
                    chunk_file_path.clone(),
                    e
                );
                return;
            }
        };

        let mut resulting_chunk = Chunk::empty();
        let base_transform = {
            let render_config = &storages.render_config;
            let mut transform = Transform::default();
            transform.translation = Vector3::new(
                chunk_id.1 as f32
                    * (self.chunk_dim.1 as f32 * render_config.tile_base_render_dim.1),
                chunk_id.0 as f32
                    * (self.chunk_dim.0 as f32 * render_config.tile_base_render_dim.0),
                0.0,
            );
            transform
        };

        #[cfg(feature = "trace")]
        trace!(
            "|\tbase translation: {:?}",
            base_transform.translation.clone()
        );

        for (&tile_id, &tile_type) in tiles.iter() {
            if let Err(e) = Chunk::add_tile(
                &self,
                &mut resulting_chunk,
                chunk_id,
                &base_transform,
                tile_id,
                Some(tile_type),
                storages,
            ) {
                error!(
                    "| ABORTING! Error adding loaded {:?} to loaded {:?}: {:?}.",
                    tile_id, chunk_id, e
                );
                return;
            }
        }

        self.chunks.insert(chunk_id, resulting_chunk);

        #[cfg(feature = "debug")]
        debug!("+------------");
    }

    /// Creates a new chunk at the given index. The chunk dimension and tile render sizes are taken from the RenderConfig-resource,
    /// which can either be fetched from the world, or from its storage.
    ///
    /// Needs access to the storages of all components used by `Tile`'s, since it creates new `Tile`-entities,
    /// preferably use from inside a system.

    pub fn new_chunk(
        &mut self,
        chunk_id: ChunkIndex,
        // NOTE: This is pretty ugly
        storages: &mut TileGenerationStorages<'_>,
    ) {
        // TODO: everything, maybe different tiles not only based on depth, but also x-pos?
        match Self::clamp_chunk_index(&self, chunk_id) {
            Ok(clamped_id) => {
                #[cfg(feature = "debug")]
                debug!("| Creating {:?}.", clamped_id);

                let chunk = Chunk::new(&self, chunk_id, self.chunk_dim, storages);
                self.chunks.insert(clamped_id, chunk);
            }
            Err(e) => {
                error!("| Requested {:?}: {:?}.", chunk_id, e);
            }
        };
    }

    /// Drains all chunks currently stored in planet, useful when `save & exit` happens.
    pub fn drain_chunks(&mut self) -> hash_map::Drain<'_, ChunkIndex, Chunk> {
        #[cfg(feature = "debug")]
        debug!("| Draining chunks.");
        self.chunks.drain()
    }

    /// Removes a `Chunk` from the world and destroys all its `Tile`s without prior saving.
    pub fn delete_chunk(
        &mut self,
        index: ChunkIndex,
        entities: &Read<'_, EntitiesRes, DefaultProvider>,
    ) {
        #[cfg(feature = "debug")]
        debug!("| Deleting {:?}.", index);
        if let Some(chunk) = self.chunks.remove(&index) {
            for (&tile_id, &entity) in chunk.iter_tile_entities() {
                if let Err(e) = entities.delete(entity) {
                    error!("| Error deleting {:?}: {:?}", tile_id, e);
                }
            }
        } else {
            error!("| Tried to delete non-existing {:?}", index);
        }
    }

    /// Returns an iterator over all chunks currently stored in planet
    /// mapping `ChunkIndex <-> Chunk`.
    pub fn iter_chunks(&self) -> hash_map::Iter<'_, ChunkIndex, Chunk> {
        #[cfg(feature = "debug")]
        debug!("| Iterating over chunks.");
        self.chunks.iter()
    }

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }
}