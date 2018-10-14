//! This module contains everything necessary for the planet.
//! - Planet
//! - ChunkIndex
//! - Chunk
//! - TileIndex

use std::collections::{BTreeMap, HashMap};

use amethyst::ecs::{Builder, Entity, World};

use entities::tile::TileTypes;
use resources::RenderConfig;

// Currently not used. Only one planet available for the beginning.
/*
/// The Index of a planet in the [Galaxy](struct.Galaxy.html).
#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub struct PlanetIndex(u32, u32);



/// A galaxy can fit a large number of planets.
pub struct Galaxy {
    pub planets: BTreeMap<PlanetIndex, Planet>,
}
*/

/// The Index of a chunk in a [Planet](struct.Planet.html).
/// Used to calculate the render-position of a chunk,
/// and to figure out which chunk the player currently resides in.
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct ChunkIndex(u32, u32);

/// The planet a player resides on.
/// Consists of individual chunks of tile entities.
#[derive(Debug, Serialize, Deserialize)]
pub struct Planet {
    /// The dimension of a planet expressed in the count of chunks in x and y direction.
    /// Differs based on the setting `Planet size` when creating a new game.
    pub planet_dim: (u32, u32),
    // A map of individual chunks of the planet, only a small number is loaded at a time.
    // Chunks that are too far from the player get serialized and stored to the disk.
    chunks: HashMap<ChunkIndex, Chunk>,
}

impl Default for Planet {
    fn default() -> Self {
        let planet_dim = (128, 128);
        Planet::new(planet_dim)
    }
}

// public interface
impl Planet {
    pub fn new(planet_dim: (u32, u32)) -> Planet {
        Planet {
            planet_dim,
            chunks: HashMap::with_capacity(9),
        }
    }

    /// Tries to fetch a chunk from the HashMap.
    /// If the given index exceeds the planet-dim bounds, it gets [clamped](struct.Planet.html#method.clamp_chunk_index).
    /// Returns `None` if no chunk at the given index exists.
    /// Try calling `new_chunk()` in that case.
    pub fn get_chunk(&mut self, index: ChunkIndex) -> Option<&Chunk> {
        let clamped_index = self.clamp_chunk_index(index);
        self.chunks.get(&clamped_index)
    }

    /// Tile indexes out of tile-dim bounds return `None`.
    pub fn get_tiletype(chunk: ChunkIndex, tile: TileIndex) -> Option<TileTypes> {
        // TODO:
        error!("Not implemented yet.");
        None
    }

    /// The given chunk index gets clamped to the planet-dim by wrapping it in x-direction and cutting it off in y-direction.
    pub fn clamp_chunk_index(&self, index: ChunkIndex) -> ChunkIndex {
        let mut rv = index;
        if rv.0 > self.planet_dim.0 || rv.0 < self.planet_dim.0 {
            rv.0 = (rv.0 % self.planet_dim.0); // + self.planet_dim.0;
            info!(
                "chunk X-index originally was: {:?}, got clamped to: {:?}, with planet_dim.0: {:?}",
                index.0, rv.0, self.planet_dim.0
            );
        }
        if rv.1 > self.planet_dim.1 || rv.1 < self.planet_dim.1 {
            rv.1 = (rv.1 % self.planet_dim.1); // + self.planet_dim.1;
            info!(
                "chunk Y-index originally was: {:?}, got clamped to: {:?}, with planet_dim.1: {:?}",
                index.1, rv.1, self.planet_dim.1
            );
        }
        rv
    }

    /// Creates a new chunk at the given index. The chunk dimension and tile render sizes are taken from the RenderConfig-resource,
    /// which can either be fetched from the world, or from its storage.
    pub fn new_chunk(&mut self, chunk_id: ChunkIndex, render_config: RenderConfig) {
        // TODO: everything
        error!("Not implemented yet.");
        self.chunks.insert(chunk_id, Chunk::new(1, render_config));
    }
}

/// The Index of a tile in a [Chunk](struct.Chunk.html).
/// Used to calculate the render-position of a tile,
/// and to figure out which tile the player currently stands on.
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct TileIndex(u32, u32);

/// Small patches of tile entities of a planet.
/// To avoid consuming gigabytes of RAM.
/// Does not implement `Default`, because it's contents are based on the depth it is placed at.
#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    // Grants access to the TileIndex via the entities (which may e.g. be returned by collision).
    #[serde(skip_serializing, skip_deserializing)]
    tile_index: BTreeMap<Entity, TileIndex>,
    // A map of individual tile entities of the chunk.
    #[serde(skip_serializing, skip_deserializing)]
    tile_entities: BTreeMap<TileIndex, Entity>,
    // A map of the TileType at a given index.
    tile_type: BTreeMap<TileIndex, TileTypes>,
}

// public interface
impl Chunk {
    pub fn new(depth: u32, render_config: RenderConfig) -> Chunk {
        // TODO: create tiles according to render_configs chunk_dim and tile_base_render_dim using `self.add_tiles`
        Chunk {
            tile_entities: BTreeMap::new(),
            tile_index: BTreeMap::new(),
            tile_type: BTreeMap::new(),
        }
    }

    /// Tries to figure out the `TileType` from the BTreeMap `tiles` at the given Index.
    /// If the given index exceeds the chunk-dim bounds, returns `None`.
    pub fn get_tile_type(&mut self, index: TileIndex) -> Option<TileTypes> {
        // TODO: Make this dependent on RenderConfig resource: `if index.0 > self.chunk_dim.0 || index.1 > self.chunk_dim.1 {return None};`
        // TODO: check `self.tiles` for index and figure out the tiletype, use `self.get_tile_entity`
        error!("Not implemented yet.");
        None
    }

    /// Tries to fetch a tile entity from the BTreeMap `tiles` at the given Index.
    /// If the given index exceeds the chunk-dim bounds, returns `None`.
    pub fn get_tile_entity(&mut self, index: TileIndex) -> Option<Entity> {
        // TODO: Make this dependent on RenderConfig resource: `if index.0 > self.chunk_dim.0 || index.1 > self.chunk_dim.1 {return None};`
        // TODO: check `self.tiles` for index and return the entity.
        error!("Not implemented yet.");
        None
    }

    /// Tries to fetch a tile from the BTreeMap `tiles_inversed` with the given entity.
    /// If the given entity is not part of this chunk, returns `None`.
    pub fn get_tile_index(&mut self, tile: Entity) -> Option<TileIndex> {
        // TODO: Get TileIndex for the given entity, or return `None`
        error!("Not implemented yet.");
        None
    }
}

// private methods
impl Chunk {
    // Creates a new tile at the given Index
    fn add_tile(&mut self, index: TileIndex, tiletype: TileTypes) {
        // TODO: call into Tiles::new()
        // TODO: populate `self.tiles` & `self.tiles_inversed`
        error!("Not implemented yet.");
    }
}
