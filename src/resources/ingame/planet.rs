//! This module contains everything necessary for the planet.
//! - Planet
//! - ChunkIndex
//! - Chunk
//! - TileIndex

use std::{
    collections::{btree_map, hash_map, BTreeMap, HashMap},
    fmt,
};

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
///
/// Uses (rows, columns).
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct ChunkIndex(pub u32, pub u32);

impl fmt::Display for ChunkIndex {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("ChunkIndex(")?;
        fmt.write_str(&self.0.to_string())?;
        fmt.write_str(", ")?;
        fmt.write_str(&self.1.to_string())?;
        fmt.write_str(")")?;

        Ok(())
    }
}

/// This is a resource.
/// The planet a player resides on.
/// Consists of individual chunks of tile entities.
#[derive(Debug, Serialize, Deserialize)]
pub struct Planet {
    /// The dimension of a planet expressed in the count of chunks in x and y direction.
    /// Differs based on the setting `Planet size` when creating a new game.
    pub planet_dim: (u32, u32),
    // TODO: Make adjustable while playing. Requires reassigning tiles to new chunks.
    /// The dimension of a chunk expressed in tilecount in x and y direction.
    /// Cannot be changed once the game was created (at least for now).
    pub chunk_dim: (u32, u32),
    // A map of individual chunks of the planet, only a small number is loaded at a time.
    // Chunks that are too far from the player get serialized and stored to the disk.
    // Private to prevent users from meddling with it.
    #[serde(skip_serializing, skip_deserializing)]
    chunks: HashMap<ChunkIndex, Chunk>,
}

// public interface
impl Planet {
    pub fn new(planet_dim: (u32, u32), chunk_dim: (u32, u32), render_config: &RenderConfig) -> Planet {
        // Chunk of the player + render distance in two directions (left+right | top+bottom)
        let chunk_count_per_direction = 1 + 2 * render_config.chunk_render_distance;
        let chunk_count = chunk_count_per_direction * chunk_count_per_direction;
        let mut rv = Planet {
            planet_dim,
            chunk_dim,
            chunks: HashMap::with_capacity(chunk_count as usize),
        };

        // <DEBUG>
        // clamp chunks to 0 <= x <= u32::MAX && 0 <= y <= u32::MAX

        // Testing wrapping
        for y in ({rv.planet_dim.1 - render_config.chunk_render_distance})..=({rv.planet_dim.1 + render_config.chunk_render_distance})  {
            for x in ({rv.planet_dim.0 - render_config.chunk_render_distance})..=({rv.planet_dim.0 + render_config.chunk_render_distance}) {
                if let Some(chunk_id) = {
                    let mut rv = ChunkIndex(y, x);
                    if rv.0 >= planet_dim.0{
                        rv.0 = (rv.0 % planet_dim.0); // + self.planet_dim.1;
                        info!(
                            "chunk y-index originally {:?} is out of bounds.",
                            y
                        );
                        None
                    }
                    else{
                        if rv.1 >= planet_dim.1{
                            rv.1 = (rv.1 % planet_dim.1); // + self.planet_dim.0;
                            info!(
                                "chunk x-index originally was: {:?}, got clamped to: {:?}, with planet_dim.0: {:?}",
                                x, rv.1, planet_dim.1
                            );
                        }
                        Some(rv)
                    }
                }{
                    debug!("+----------");
                    debug!("| chunk number {}, {:?}", { chunk_id.0 * rv.planet_dim.1 + chunk_id.1 }, chunk_id);
                    rv.new_chunk(chunk_id);
                }
                else{
                    warn!("{:?} is out of bounds.", ChunkIndex(y,x));
                }
            }
        }/*
        for y in 0..chunk_count_per_direction {
            for x in 0..chunk_count_per_direction {
                debug!("+----------");
                debug!("| chunk number {}", { y * rv.planet_dim.0 + x });
                rv.new_chunk(ChunkIndex(y, x));
            }
        }*/
        debug!("+----------");
        // </DEBUG>

        rv
    }

    /// Tries to fetch a chunk from the HashMap.
    /// If the given index exceeds the planet-dim bounds, it gets [clamped](struct.Planet.html#method.clamp_chunk_index).
    /// Returns `None` if no chunk at the given index exists.
    /// Try calling `new_chunk()` in that case.
    pub fn get_chunk(&mut self, index: ChunkIndex) -> Option<&Chunk> {
        if let Some(clamped_index) = self.clamp_chunk_index(index){
            self.chunks.get(&clamped_index)
        }
        else{
            None
        }
    }

    /// Tile indexes out of tile-dim bounds return `None`.
    pub fn get_tiletype(chunk: ChunkIndex, tile: TileIndex) -> Option<TileTypes> {
        // TODO:
        error!("Not implemented yet.");
        None
    }

    /// The given chunk index gets clamped to the planet-dim by wrapping it in x-direction.
    /// Returns none if the index is out of bounds in y-direction.
    pub fn clamp_chunk_index(&self, index: ChunkIndex) -> Option<ChunkIndex> {
        let mut rv = index;
        if rv.0 >= self.planet_dim.0{
            rv.0 = (rv.0 % self.planet_dim.0); // + self.planet_dim.1;
            info!(
                "chunk Y-index originally  {:?} is out of bounds.",
                index.0
            );
            None
        }
        else{
            if rv.1 >= self.planet_dim.1{
                rv.1 = (rv.1 % self.planet_dim.1); // + self.planet_dim.0;
                info!(
                    "chunk X-index originally was: {:?}, got clamped to: {:?}, with planet_dim.0: {:?}",
                    index.1, rv.1, self.planet_dim.1
                );
            }
            Some(rv)
        }
    }

    /// Creates a new chunk at the given index. The chunk dimension and tile render sizes are taken from the RenderConfig-resource,
    /// which can either be fetched from the world, or from its storage.
    pub fn new_chunk(&mut self, chunk_id: ChunkIndex) {
        // TODO: everything, maybe different tiles not only based on depth, but also x-pos?
        self.chunks.insert(chunk_id, Chunk::new(chunk_id.0, self.chunk_dim));
    }

    /// Drains all chunks currently stored in planet, useful when `save & exit` happens.
    pub fn drain_chunks(&mut self) -> hash_map::Drain<ChunkIndex, Chunk> {
        self.chunks.drain()
    }

    /// Returns an iterator over all chunks currently stored in planet
    /// mapping `ChunkIndex <-> Chunk`.
    pub fn iter_chunks(&self) -> hash_map::Iter<ChunkIndex, Chunk> {
        self.chunks.iter()
    }
}

/// The Index of a tile in a [Chunk](struct.Chunk.html).
/// Used to calculate the render-position of a tile,
/// and to figure out which tile the player currently stands on.
///
/// Uses (rows, columns).
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct TileIndex(pub u32, pub u32);

impl fmt::Display for TileIndex {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("TileIndex(")?;
        fmt.write_str(&self.0.to_string())?;
        fmt.write_str(", ")?;
        fmt.write_str(&self.1.to_string())?;
        fmt.write_str(")")?;

        Ok(())
    }
}

/// This is not a resource, it is  wrapped in the [planet](struct.Planet.html).
/// Small patches of tile entities of a planet.
/// Does not implement `Default`, because its contents are based on the depth it is placed at.
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
    pub fn new(depth: u32, chunk_dim: (u32, u32)) -> Chunk {
        // TODO: create tiles according to render_configs chunk_dim and tile_base_render_dim using `self.add_tiles`
        let mut rv = Chunk {
            tile_entities: BTreeMap::new(),
            tile_index: BTreeMap::new(),
            tile_type: BTreeMap::new(),
        };

        // TODO: Actual tile generation algorithm
        for i in 0..chunk_dim.0 {
            for j in 0..chunk_dim.1 {
                debug!("|\ttile number {}", { i + j });
                rv.tile_type.insert(TileIndex(i, j), TileTypes::Dirt);
            }
        }

        rv
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

    /// Returns an iterator over the `tile_types` field,
    /// which maps `TileIndex <-> TileTypes`.
    pub fn iter_tiles(&self) -> btree_map::Iter<TileIndex, TileTypes> {
        self.tile_type.iter()
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
