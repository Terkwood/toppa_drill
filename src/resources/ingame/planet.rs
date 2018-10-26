//! This module contains everything necessary for the planet.
//! - Planet
//! - ChunkIndex
//! - Chunk
//! - TileIndex

use std::{
    collections::{btree_map, hash_map, BTreeMap, HashMap},
    fmt,
};

use rand::*;

use amethyst::{core::transform::components::Transform, ecs::Entity};

use entities::tile::TileTypes;
use resources::RenderConfig;

// Currently not used. Only one planet available for the beginning.
/*
/// The Index of a planet in the [Galaxy](struct.Galaxy.html).
#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub struct PlanetIndex(u64, u64);



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
pub struct ChunkIndex(pub u64, pub u64);

impl ChunkIndex {
    pub fn from_transform(
        transform: &Transform,
        render_config: &RenderConfig,
        planet: &Planet,
    ) -> Option<Self> {
        // TODO: Clamp to planet.planet_dim !
        let x_transl = transform.translation[0];
        let y_transl = transform.translation[1];

        let tile_width_f32 = render_config.tile_base_render_dim.1;
        let tile_height_f32 = render_config.tile_base_render_dim.0;
        let chunk_width_f32 = planet.chunk_dim.1 as f32 * tile_width_f32;
        let chunk_height_f32 = planet.chunk_dim.0 as f32 * tile_height_f32;

        let chunk_x_f32 = (x_transl / chunk_width_f32).trunc();
        let chunk_y_f32 = (y_transl / chunk_height_f32).trunc();
        if chunk_x_f32.is_sign_negative() || chunk_y_f32.is_sign_negative() {
            return None;
        }

        let chunk_x = chunk_x_f32.trunc();
        let chunk_y = chunk_y_f32.trunc();
        Some(ChunkIndex(chunk_y as u64, chunk_x as u64))
    }
}

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
    pub planet_dim: (u64, u64),
    // TODO: Make adjustable while playing. Requires reassigning tiles to new chunks.
    /// The dimension of a chunk expressed in tilecount in x and y direction.
    /// Cannot be changed once the game was created (at least for now).
    pub chunk_dim: (u64, u64),
    // A map of individual chunks of the planet, only a small number is loaded at a time.
    // Chunks that are too far from the player get serialized and stored to the disk.
    // Private to prevent users from meddling with it.
    #[serde(skip_serializing, skip_deserializing)]
    chunks: HashMap<ChunkIndex, Chunk>,
}

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
        let mut rv = Planet {
            planet_dim,
            chunk_dim,
            chunks: HashMap::with_capacity(chunk_count as usize),
        };

        /*
        // <DEBUG>
        // clamp chunks to 0 <= x <= u64::MAX && 0 <= y <= u64::MAX

        // Testing wrapping
        for y in ({ rv.planet_dim.1 - render_config.chunk_render_distance })..=({
            rv.planet_dim.1 + render_config.chunk_render_distance
        }) {
            for x in ({ rv.planet_dim.0 - render_config.chunk_render_distance })..=({
                rv.planet_dim.0 + render_config.chunk_render_distance
            }) {
                if let Some(chunk_id) = {
                    let mut rv = ChunkIndex(y, x);
                    if rv.0 >= planet_dim.0 {
                        rv.0 = rv.0 % planet_dim.0;
                        {/*turn back to debug later*/}warn!("chunk y-index originally {:?} is out of bounds.", y);
                        None
                    } else {
                        if rv.1 >= planet_dim.1 {
                            rv.1 = rv.1 % planet_dim.1;
                            {/*turn back to debug later*/}warn!(
                                "chunk x-index originally was: {:?}, got clamped to: {:?}, with planet_dim.0: {:?}",
                                x, rv.1, planet_dim.1
                            );
                        }
                        Some(rv)
                    }
                } {
                    #[cfg(feature = "debug")]
                    {/*turn back to debug later*/}warn!("+----------");
                    #[cfg(feature = "debug")]
                    {/*turn back to debug later*/}warn!(
                        "| chunk number {}, {:?}",
                        { chunk_id.0 * rv.planet_dim.1 + chunk_id.1 },
                        chunk_id
                    );
                    rv.new_chunk(chunk_id);
                } else {
                    warn!("{:?} is out of bounds.", ChunkIndex(y, x));
                }
            }
        }
        #[cfg(feature = "debug")]
        {/*turn back to debug later*/}warn!("+----------");
        // </DEBUG>*/

        rv
    }

    /// Tries to fetch a chunk from the HashMap.
    /// If the given index exceeds the planet-dim bounds, it gets [clamped](struct.Planet.html#method.clamp_chunk_index).
    /// Returns `None` if no chunk at the given index exists.
    /// Try calling `new_chunk()` in that case.
    #[allow(dead_code)]
    pub fn get_chunk(&mut self, index: ChunkIndex) -> Option<&Chunk> {
        if let Some(clamped_index) = self.clamp_chunk_index(index) {
            self.chunks.get(&clamped_index)
        } else {
            None
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
    #[allow(dead_code)]
    pub fn clamp_chunk_index(&self, index: ChunkIndex) -> Option<ChunkIndex> {
        let mut rv = index;
        if rv.0 >= self.planet_dim.0 {
            rv.0 = rv.0 % self.planet_dim.0;
            {/*turn back to debug later*/}warn!("chunk Y-index originally  {:?} is out of bounds.", index.0);
            None
        } else {
            if rv.1 >= self.planet_dim.1 {
                rv.1 = rv.1 % self.planet_dim.1;
                {/*turn back to debug later*/}warn!(
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
        warn!("Creating new chunk at {:?}.", chunk_id);
        self.chunks
            .insert(chunk_id, Chunk::new(chunk_id.0, self.chunk_dim));
    }

    /// Drains all chunks currently stored in planet, useful when `save & exit` happens.
    #[allow(dead_code)]
    pub fn drain_chunks(&mut self) -> hash_map::Drain<ChunkIndex, Chunk> {
        self.chunks.drain()
    }

    pub fn remove_chunk(&mut self, key: &ChunkIndex) -> Option<Chunk> {
        self.chunks.remove(key)
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
pub struct TileIndex(pub u64, pub u64);

impl TileIndex {
    /// Convenience function returning only the TileIndex. Best used when Chunk Index is known
    pub fn from_transform(
        transform: &Transform,
        chunk_index: ChunkIndex,
        render_config: &RenderConfig,
        planet: &Planet,
    ) -> Option<Self> {
        let x_transl = transform.translation[0];
        let y_transl = transform.translation[1];

        let tile_width_f32 = render_config.tile_base_render_dim.1;
        let tile_height_f32 = render_config.tile_base_render_dim.0;
        let chunk_width_f32 = planet.chunk_dim.1 as f32 * tile_width_f32;
        let chunk_height_f32 = planet.chunk_dim.0 as f32 * tile_height_f32;

        let x_chunk_transl = x_transl - (chunk_index.1 as f32 * chunk_width_f32);
        let y_chunk_transl = y_transl - (chunk_index.0 as f32 * chunk_height_f32);
        // Supposedly more accurate, but is it necessary?
        /*let x_chunk_transl = chunk_x.mul_add(-chunk_width_f32, x_transl);
        let y_chunk_transl = chunk_y.mul_add(-chunk_height_f32, y_transl);*/

        let tile_x_f32 = (x_chunk_transl / tile_width_f32).trunc();
        let tile_y_f32 = (y_chunk_transl / tile_height_f32).trunc();
        if tile_x_f32.is_sign_negative()
            || tile_y_f32.is_sign_negative()
            || tile_x_f32 > (x_chunk_transl + chunk_width_f32)
            || tile_y_f32 > (y_chunk_transl + chunk_height_f32)
        {
            return None;
        }

        let tile_x = tile_x_f32.trunc();
        let tile_y = tile_y_f32.trunc();
        Some(TileIndex(tile_y as u64, tile_x as u64))
    }
}

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
    pub fn new(_depth: u64, chunk_dim: (u64, u64)) -> Chunk {
        // TODO: create tiles according to render_configs chunk_dim and tile_base_render_dim using `self.add_tiles`
        let mut rv = Chunk {
            tile_entities: BTreeMap::new(),
            tile_index: BTreeMap::new(),
            tile_type: BTreeMap::new(),
        };

        // TODO: Actual tile generation algorithm
        for y in 0..chunk_dim.0 {
            for x in 0..chunk_dim.1 {
                {/*turn back to debug later*/}warn!("|\ttile number {}", { y * chunk_dim.1 + x });

                let tile_number = thread_rng().gen_range(0, 16);
                Self::add_tile(&mut rv, TileIndex(y, x), tile_number);
            }
        }

        rv
    }

    /// Tries to figure out the `TileType` from the BTreeMap `tiles` at the given Index.
    /// If the given index exceeds the chunk-dim bounds, returns `None`.
    #[allow(dead_code)]
    pub fn get_tile_type(&mut self, _index: TileIndex) -> Option<TileTypes> {
        // TODO: Make this dependent on RenderConfig resource: `if index.0 > self.chunk_dim.0 || index.1 > self.chunk_dim.1 {return None};`
        // TODO: check `self.tiles` for index and figure out the tiletype, use `self.get_tile_entity`
        error!("Not implemented yet.");
        None
    }

    /// Tries to fetch a tile entity from the BTreeMap `tiles` at the given Index.
    /// If the given index exceeds the chunk-dim bounds, returns `None`.
    #[allow(dead_code)]
    pub fn get_tile_entity(&mut self, _index: TileIndex) -> Option<Entity> {
        // TODO: Make this dependent on RenderConfig resource: `if index.0 > self.chunk_dim.0 || index.1 > self.chunk_dim.1 {return None};`
        // TODO: check `self.tiles` for index and return the entity.
        error!("Not implemented yet.");
        None
    }

    /// Tries to fetch a tile from the BTreeMap `tiles_inversed` with the given entity.
    /// If the given entity is not part of this chunk, returns `None`.
    #[allow(dead_code)]
    pub fn get_tile_index(&mut self, _tile: Entity) -> Option<TileIndex> {
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
    #[allow(dead_code)]
    fn add_tile(chunk: &mut Chunk, index: TileIndex, tile_number: usize) {
        // TODO: call into Tiles::new()
        // TODO: populate `self.tiles` & `self.tiles_inversed`
        let tile_type = match tile_number {
            0 => {TileTypes::Magnetite},
            1 => {TileTypes::Pyrolusite},
            2 => {TileTypes::Fossile},
            3 => {TileTypes::Molybdenite},
            4 => {TileTypes::Lava},
            5 => {TileTypes::Rock},
            6 => {TileTypes::Gas},
            7 => {TileTypes::Galena},
            8 => {TileTypes::Bornite},
            9 => {TileTypes::Chromite},
            10 => {TileTypes::Cassiterite},
            11 => {TileTypes::Cinnabar},
            12 => {TileTypes::Dirt},
            13 => {TileTypes::Gold},
            14 => {TileTypes::Empty},
            15 => {TileTypes::Bauxite},
            _ => {return;},
        };

        info!("|\t\t{:?}", tile_type);
        chunk.tile_type.insert(index, tile_type);
    }
}
