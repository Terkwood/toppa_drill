//! This module contains everything necessary for the planet.
//! - Planet
//! - ChunkIndex
//! - Chunk
//! - TileIndex

use std::{
    collections::{btree_map, hash_map, BTreeMap, HashMap},
    fmt, fs,
    path::*,
};

use rand::*;
use ron;
use serde::Serializer;

use amethyst::{
    core::{cgmath::Vector3, transform::components::Transform},
    ecs::{prelude::*, storage::MaskedStorage, world::EntitiesRes, Storage},
    renderer::SpriteRender,
    shred::{DefaultProvider, FetchMut},
};

use crate::{
    components::{for_ground_entities::TileBase, IsIngameEntity},
    entities::{tile::TileTypes, EntitySpriteRender},
    resources::{GameSprites, RenderConfig},
};

/// Internal use only (for the Chunk-Hotloading), do not use!
pub struct TileGenerationStorages<'a,> {
    pub entities:      Read<'a, EntitiesRes,>,
    pub tile_base:     Storage<'a, TileBase, FetchMut<'a, MaskedStorage<TileBase,>,>,>,
    pub sprite_render: Storage<'a, SpriteRender, FetchMut<'a, MaskedStorage<SpriteRender,>,>,>,
    pub transform:     Storage<'a, Transform, FetchMut<'a, MaskedStorage<Transform,>,>,>,
    pub ingame_entity: Storage<'a, IsIngameEntity, FetchMut<'a, MaskedStorage<IsIngameEntity,>,>,>,
    pub game_sprites:  Read<'a, GameSprites, DefaultProvider,>,
    pub render_config: Read<'a, RenderConfig, DefaultProvider,>,
}

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

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub enum TileError {
    #[allow(dead_code)]
    NotImplemented,
    IndexOutOfBounds,
    SpriteRenderNotFound(EntitySpriteRender,),
}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub enum ChunkIndexError {
    #[allow(dead_code)]
    NotImplemented,
    SkyLimit,
    TooDeep,
}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub enum ChunkError {
    #[allow(dead_code)]
    NotImplemented,
    IndexOutOfBounds(ChunkIndexError,),
    NotFound,
}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub enum PlanetError {
    #[allow(dead_code)]
    NotImplemented,
    ChunkProblem(ChunkError,),
    TileProblem(TileError,),
}

/// The Index of a chunk in a [Planet](struct.Planet.html).
/// Used to calculate the render-position of a chunk,
/// and to figure out which chunk the player currently resides in.
///
/// Uses (rows, columns).
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct ChunkIndex(pub u64, pub u64,);

impl ChunkIndex {
    pub fn from_transform(
        transform: &Transform,
        render_config: &RenderConfig,
        planet: &Planet,
    ) -> Result<Self, PlanetError,> {
        let x_transl = transform.translation[0];
        let y_transl = transform.translation[1];

        let tile_width_f32 = render_config.tile_base_render_dim.1;
        let tile_height_f32 = render_config.tile_base_render_dim.0;
        let chunk_width_f32 = planet.chunk_dim.1 as f32 * tile_width_f32;
        let chunk_height_f32 = planet.chunk_dim.0 as f32 * tile_height_f32;

        let chunk_x_f32 = (x_transl / chunk_width_f32).trunc();
        let chunk_y_f32 = (y_transl / chunk_height_f32).trunc();

        if chunk_x_f32.is_sign_negative() || chunk_y_f32.is_sign_negative() {
            #[cfg(feature = "debug")]
            debug!("| Negative chunk index.");

            return Err(PlanetError::ChunkProblem(ChunkError::IndexOutOfBounds(
                ChunkIndexError::TooDeep,
            ),),);
        }

        let chunk_x = chunk_x_f32.trunc();
        let chunk_y = chunk_y_f32.trunc();
        let chunk_id = ChunkIndex(chunk_y as u64, chunk_x as u64,);

        match Planet::clamp_chunk_index(planet, chunk_id,) {
            Ok(chunk_id,) => {
                #[cfg(feature = "debug")]
                debug!("| From transform: {:?}", chunk_id);

                Ok(chunk_id,)
            },
            Err(e,) => Err(e,),
        }
    }
}

impl fmt::Display for ChunkIndex {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_,>,) -> fmt::Result {
        fmt.write_str("ChunkIndex(",)?;
        fmt.write_str(&self.0.to_string(),)?;
        fmt.write_str(", ",)?;
        fmt.write_str(&self.1.to_string(),)?;
        fmt.write_str(")",)?;

        Ok((),)
    }
}

/// This is a resource.
/// The planet a player resides on.
/// Consists of individual chunks of tile entities.
#[derive(Debug, Serialize, Deserialize)]
pub struct Planet {
    /// The dimension of a planet expressed in the count of chunks in x and y direction.
    /// Differs based on the setting `Planet size` when creating a new game.
    pub planet_dim: (u64, u64,),
    // TODO: Make adjustable while playing. Requires reassigning tiles to new chunks.
    /// The dimension of a chunk expressed in tilecount in x and y direction.
    /// Cannot be changed once the game was created (at least for now).
    pub chunk_dim: (u64, u64,),
    // A map of individual chunks of the planet, only a small number is loaded at a time.
    // Chunks that are too far from the player get serialized and stored to the disk.
    // Private to prevent users from meddling with it.
    #[serde(skip_serializing, default = "serde_de_empty_hash_map")]
    chunks: HashMap<ChunkIndex, Chunk,>,
}

pub fn serde_de_empty_hash_map() -> HashMap<ChunkIndex, Chunk,> {
    HashMap::with_capacity(9,)
}

// public interface
impl Planet {
    pub fn new(
        planet_dim: (u64, u64,),
        chunk_dim: (u64, u64,),
        render_config: &RenderConfig,
    ) -> Planet {
        // Chunk of the player + render distance in two directions (left+right | top+bottom)
        let chunk_count_per_direction = 1 + 2 * render_config.chunk_render_distance;
        let chunk_count = chunk_count_per_direction * chunk_count_per_direction;
        Planet {
            planet_dim,
            chunk_dim,
            chunks: HashMap::with_capacity(chunk_count as usize,),
        }
    }

    /// Tries to fetch a chunk from the HashMap.
    /// If the given index exceeds the planet-dim bounds, it gets [clamped](struct.Planet.html#method.clamp_chunk_index).
    /// Returns either a reference to a chunk, if it found one, or an error.
    /// This Error could be `PlanetError::ChunkProblem(ChunkError::NotFound))`,
    /// if that's the case try calling `new_chunk()`.
    #[allow(dead_code)]
    pub fn get_chunk(&self, index: ChunkIndex,) -> Result<Option<&Chunk,>, PlanetError,> {
        let clamped_id_result = Self::clamp_chunk_index(&self, index,);
        match clamped_id_result {
            Ok(clamped_index,) => Ok(self.chunks.get(&clamped_index,),),
            Err(e,) => {
                warn!("| Error clamping chunk index: {:?}", e);
                Err(PlanetError::ChunkProblem(ChunkError::NotFound,),)
            },
        }
    }

    /// Tile indexes out of tile-dim bounds return `None`.
    #[allow(dead_code)]
    pub fn get_tiletype(_chunk: ChunkIndex, _tile: TileIndex,) -> Option<TileTypes,> {
        // TODO:
        error!("Not implemented yet.");
        None
    }

    /// The given chunk index gets clamped to the planet-dim by wrapping it in x-direction.
    /// Returns none if the index is out of bounds in y-direction.
    pub fn clamp_chunk_index(
        planet: &Planet,
        index: ChunkIndex,
    ) -> Result<ChunkIndex, PlanetError,> {
        #[cfg(feature = "trace")]
        error!("ChunkIndex clamping.");

        let mut rv = index;
        if rv.0 >= planet.planet_dim.0 {
            Err(PlanetError::ChunkProblem(ChunkError::IndexOutOfBounds(
                ChunkIndexError::SkyLimit,
            ),),)
        }
        else {
            if rv.1 >= planet.planet_dim.1 {
                #[cfg(feature = "debug")]
                let buff = rv.1;
                rv.1 = rv.1 % planet.planet_dim.1;
                #[cfg(feature = "debug")]
                debug!("| Wrapping index from {:?} to {:?}.", buff, rv.1);
            }
            Ok(rv,)
        }
    }

    /// Saves a chunk in the specified directory without removing it from the planet.
    /// TODO: Save with less space-usage
    pub fn save_chunk(&self, chunk_id: ChunkIndex, chunk_dir_path: PathBuf,) {
        let planet_width_in_chunks = self.planet_dim.0;
        match self.get_chunk(chunk_id,) {
            Ok(chunk_opt,) => {
                if let Some(chunk,) = chunk_opt {
                    let mut ser_chunk = ron::ser::Serializer::new(Some(Default::default(),), true,);
                    //NOTE: Use this to save disk space!
                    //let mut ser_chunk = ron::ser::Serializer::new(Some(Default::default()), false);
                    {
                        use serde::ser::SerializeMap;
                        #[cfg(feature = "debug")]
                        debug!("| serializing {:?}", chunk_id);

                        if let Ok(mut serseq,) = ser_chunk.serialize_map(None,) {
                            for (tile_index, tile_type,) in chunk.iter_tiletypes() {
                                if let Err(e,) = serseq.serialize_key::<TileIndex>(&tile_index,) {
                                    error!(
                                        "| Error serializing key of Tile {:?} in Chunk {:?}: {:?}",
                                        tile_index, chunk_id, e
                                    );
                                }
                                if let Err(e,) = serseq.serialize_value::<TileTypes>(&tile_type,) {
                                    error!(
                                        "| Error serializing value of Tile {:?} in Chunk {:?}: {:?}",
                                        tile_index, chunk_id, e
                                    );
                                }
                            }
                            if let Err(e,) = serseq.end() {
                                error!(
                                    "| Error ending serialize for chunk {:?}: {:?}",
                                    chunk_id, e
                                );
                            }
                        }
                        else {
                            error!("| Error starting serialize for chunk {:?}.", chunk_id);
                        }
                    }

                    let mut chunk_file_path = chunk_dir_path.clone();
                    chunk_file_path = chunk_file_path.join(Path::new(
                        &{ (chunk_id.1 * planet_width_in_chunks + chunk_id.0) as u64 }.to_string(),
                    ),);
                    chunk_file_path.set_extension("ron",);

                    if let Err(e,) =
                        fs::write(chunk_file_path.clone(), ser_chunk.into_output_string(),)
                    {
                        error!(
                            "| Writing chunk {:?} at '{:?}' resulted in {:?}",
                            chunk_id, chunk_file_path, e
                        );
                    }
                }
                else {
                    #[cfg(feature = "debug")]
                    debug!("| Removing {:?} failed, since it was not found.", chunk_id);
                }
            },
            Err(e,) => {
                error!("| Error getting chunk from planet: {:?}.", e);
            },
        }
    }

    /// TODO: When save_chunk uses less-space-variant, custom deserializer needed.
    pub fn load_chunk(
        &mut self,
        chunk_id: ChunkIndex,
        chunk_file_path: PathBuf,
        storages: &mut TileGenerationStorages<'_,>,
    ) {
        #[cfg(feature = "debug")]
        debug!("+------------");
        #[cfg(feature = "debug")]
        debug!("| chunk_file_path: {:?}", chunk_file_path);

        let file = match fs::File::open(&chunk_file_path,) {
            Ok(rv,) => rv,
            Err(e,) => {
                error!("| Could not open {:?}: {:?}.", chunk_file_path.clone(), e);
                return;
            },
        };

        let tiles: BTreeMap<TileIndex, TileTypes,> = match ron::de::from_reader(&file,) {
            Ok(chunk,) => chunk,
            Err(e,) => {
                error!(
                    "| Error deserializing {:?}: {:?}.",
                    chunk_file_path.clone(),
                    e
                );
                return;
            },
        };

        let mut chunk = Chunk::empty();
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

        for (&tile_id, &tile_type,) in tiles.iter() {
            if let Err(e,) = Chunk::add_tile(
                &self,
                &mut chunk,
                &base_transform,
                tile_id,
                Some(tile_type,),
                storages,
            ) {
                error!(
                    "| ABORTING! Error adding loaded {:?} to loaded {:?}: {:?}.",
                    tile_id, chunk_id, e
                );
                return;
            }
        }

        self.chunks.insert(chunk_id, chunk,);

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
        storages: &mut TileGenerationStorages<'_,>,
    ) {
        // TODO: everything, maybe different tiles not only based on depth, but also x-pos?
        match Self::clamp_chunk_index(&self, chunk_id,) {
            Ok(clamped_id,) => {
                #[cfg(feature = "debug")]
                debug!("| Creating {:?}.", clamped_id);

                let chunk = Chunk::new(&self, chunk_id, self.chunk_dim, storages,);
                self.chunks.insert(clamped_id, chunk,);
            },
            Err(e,) => {
                error!("| Requested index {:?}: {:?}.", chunk_id, e);
            },
        };
    }

    /// Drains all chunks currently stored in planet, useful when `save & exit` happens.
    #[allow(dead_code)]
    pub fn drain_chunks(&mut self) -> hash_map::Drain<'_, ChunkIndex, Chunk,> {
        #[cfg(feature = "debug")]
        debug!("| Draining chunks.");
        self.chunks.drain()
    }

    /// Removes a `Chunk` from the world and destroys all its `Tile`s without prior saving.
    pub fn delete_chunk(
        &mut self,
        index: ChunkIndex,
        entities: &Read<'_, EntitiesRes, DefaultProvider,>,
    ) {
        #[cfg(feature = "debug")]
        debug!("| Deleting {:?}.", index);
        if let Some(chunk,) = self.chunks.remove(&index,) {
            for (&tile_id, &entity,) in chunk.iter_tile_entities() {
                if let Err(e,) = entities.delete(entity,) {
                    error!("| Error deleting {:?}: {:?}", tile_id, e);
                }
            }
        }
        else {
            error!("| Tried to delete non-existing {:?}", index);
        }
    }

    /// Returns an iterator over all chunks currently stored in planet
    /// mapping `ChunkIndex <-> Chunk`.
    pub fn iter_chunks(&self) -> hash_map::Iter<'_, ChunkIndex, Chunk,> {
        #[cfg(feature = "debug")]
        debug!("| Iterating over chunks.");
        self.chunks.iter()
    }

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }
}

/// The Index of a tile in a [Chunk](struct.Chunk.html).
/// Used to calculate the render-position of a tile,
/// and to figure out which tile the player currently stands on.
///
/// Uses (rows, columns).
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct TileIndex(pub u64, pub u64,);

impl TileIndex {
    /// Convenience function returning only the TileIndex. Best used when Chunk Index is known
    pub fn from_transform(
        transform: &Transform,
        chunk_index: ChunkIndex,
        render_config: &RenderConfig,
        planet: &Planet,
    ) -> Result<Self, PlanetError,> {
        let x_transl = transform.translation[0];
        let y_transl = transform.translation[1];

        let tile_width_f32 = render_config.tile_base_render_dim.1;
        let tile_height_f32 = render_config.tile_base_render_dim.0;
        let chunk_width_f32 = planet.chunk_dim.1 as f32 * tile_width_f32;
        let chunk_height_f32 = planet.chunk_dim.0 as f32 * tile_height_f32;

        let chunk_offset_x = chunk_index.1 as f32 * chunk_width_f32;
        let chunk_offset_y = chunk_index.0 as f32 * chunk_height_f32;

        let x_chunk_transl = x_transl - (chunk_offset_x);
        let y_chunk_transl = y_transl - (chunk_offset_y);
        // Supposedly more accurate, but is it necessary?
        /*let x_chunk_transl = chunk_x.mul_add(-chunk_width_f32, x_transl);
        let y_chunk_transl = chunk_y.mul_add(-chunk_height_f32, y_transl);*/

        let tile_id_x_f32 = (x_chunk_transl / tile_width_f32).trunc();
        let tile_id_y_f32 = (y_chunk_transl / tile_height_f32).trunc();
        let tile_id_x = tile_id_x_f32.trunc() as u64;
        let tile_id_y = tile_id_y_f32.trunc() as u64;
        if tile_id_x_f32.is_sign_negative()
            || tile_id_y_f32.is_sign_negative()
            || tile_id_x >= planet.chunk_dim.1
            || tile_id_y >= planet.chunk_dim.0
        {
            return Err(PlanetError::TileProblem(TileError::IndexOutOfBounds,),);
        }

        Ok(TileIndex(tile_id_y, tile_id_x,),)
    }
}

impl fmt::Display for TileIndex {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_,>,) -> fmt::Result {
        fmt.write_str("TileIndex(",)?;
        fmt.write_str(&self.0.to_string(),)?;
        fmt.write_str(", ",)?;
        fmt.write_str(&self.1.to_string(),)?;
        fmt.write_str(")",)?;
        Ok((),)
    }
}

/// This is not a resource, it is  wrapped in the [planet](struct.Planet.html).
/// Small patches of tile entities of a planet.
/// Does not implement `Default`, because its contents are based on the depth it is placed at.
#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    // Grants access to the TileIndex via the entities (which may e.g. be returned by collision).
    #[serde(skip_serializing, skip_deserializing)]
    tile_index: BTreeMap<Entity, TileIndex,>,
    // A map of individual tile entities of the chunk.
    #[serde(skip_serializing, skip_deserializing)]
    tile_entities: BTreeMap<TileIndex, Entity,>,
    // A map of the TileType at a given index.
    tile_type: BTreeMap<TileIndex, TileTypes,>,
}

// public interface
impl Chunk {
    pub fn empty() -> Chunk {
        Chunk {
            tile_index:    BTreeMap::new(),
            tile_entities: BTreeMap::new(),
            tile_type:     BTreeMap::new(),
        }
    }

    pub fn new(
        planet: &Planet,
        chunk_id: ChunkIndex,
        chunk_dim: (u64, u64,),
        // NOTE: This is pretty ugly
        storages: &mut TileGenerationStorages<'_,>,
    ) -> Chunk {
        let mut rv = Chunk {
            tile_entities: BTreeMap::new(),
            tile_index:    BTreeMap::new(),
            tile_type:     BTreeMap::new(),
        };

        let base_transform = {
            let render_config = &storages.render_config;
            let mut transform = Transform::default();
            transform.translation = Vector3::new(
                chunk_id.1 as f32
                    * (planet.chunk_dim.1 as f32 * render_config.tile_base_render_dim.1),
                chunk_id.0 as f32
                    * (planet.chunk_dim.0 as f32 * render_config.tile_base_render_dim.0),
                0.0,
            );
            transform
        };
        #[cfg(feature = "trace")]
        trace!(
            "|\tbase translation: {:?}",
            base_transform.translation.clone()
        );

        // TODO: Actual tile generation algorithm
        for y in 0 .. chunk_dim.0 {
            for x in 0 .. chunk_dim.1 {
                #[cfg(feature = "trace")]
                trace!("|\ttile number {}", { y * chunk_dim.1 + x });

                let tile_id = TileIndex(y, x,);
                if let Err(e,) =
                    Self::add_tile(planet, &mut rv, &base_transform, tile_id, None, storages,)
                {
                    error!("Error creating {:?}: {:?}!", tile_id, e);
                };
            }
        }

        rv
    }

    /// Tries to figure out the `TileType` from the BTreeMap `tiles` at the given Index.
    /// If the given index exceeds the chunk-dim bounds, returns `None`.
    #[allow(dead_code)]
    pub fn get_tile_type(&mut self, _index: TileIndex,) -> Option<TileTypes,> {
        // TODO: Make this dependent on RenderConfig resource: `if index.0 > self.chunk_dim.0 || index.1 > self.chunk_dim.1 {return None};`
        // TODO: check `self.tiles` for index and figure out the tiletype, use `self.get_tile_entity`
        error!("Not implemented yet.");
        None
    }

    /// Tries to fetch a tile entity from the BTreeMap `tiles` at the given Index.
    /// If the given index exceeds the chunk-dim bounds, returns `None`.
    #[allow(dead_code)]
    pub fn get_tile_entity(&mut self, _index: TileIndex,) -> Option<Entity,> {
        // TODO: Make this dependent on RenderConfig resource: `if index.0 > self.chunk_dim.0 || index.1 > self.chunk_dim.1 {return None};`
        // TODO: check `self.tiles` for index and return the entity.
        error!("Not implemented yet.");
        None
    }

    /// Tries to fetch a tile from the BTreeMap `tiles_inversed` with the given entity.
    /// If the given entity is not part of this chunk, returns `None`.
    #[allow(dead_code)]
    pub fn get_tile_index(&mut self, _tile: Entity,) -> Option<TileIndex,> {
        // TODO: Get TileIndex for the given entity, or return `None`
        error!("Not implemented yet.");
        None
    }

    /// Returns an iterator over the `tile_types` field,
    /// which maps `TileIndex <-> TileTypes`.
    pub fn iter_tiletypes(&self) -> btree_map::Iter<'_, TileIndex, TileTypes,> {
        self.tile_type.iter()
    }

    /// Returns an iterator over the `tile_entities` field,
    /// which maps `TileIndex <-> TileTypes`.
    pub fn iter_tile_entities(&self) -> btree_map::Iter<'_, TileIndex, Entity,> {
        self.tile_entities.iter()
    }

    /// The given tile index gets clamped to the chunk-dim by cutting it off in all directions.
    /// Returns none if the index is out of bounds.
    pub fn clamp_tile_index(planet: &Planet, index: TileIndex,) -> Result<TileIndex, PlanetError,> {
        let rv = index;
        if rv.0 >= planet.chunk_dim.0 || rv.1 >= planet.chunk_dim.1 {
            #[cfg(feature = "debug")]
            debug!("tile index originally {:?} is out of bounds.", index);
            Err(PlanetError::TileProblem(TileError::IndexOutOfBounds,),)
        }
        else {
            Ok(rv,)
        }
    }
}

// private methods
impl Chunk {
    fn add_tile(
        planet: &Planet,
        chunk: &mut Chunk,
        base_transform: &Transform,
        tile_id: TileIndex,
        tile_type_opt: Option<TileTypes,>,
        // NOTE: This is pretty ugly
        storages: &mut TileGenerationStorages<'_,>,
    ) -> Result<(), self::PlanetError,> {
        match Self::create_tile(planet, base_transform, tile_id, tile_type_opt, storages,) {
            Ok((tile_type, entity,),) => {
                chunk.tile_type.insert(tile_id, tile_type,);
                chunk.tile_index.insert(entity, tile_id,);
                chunk.tile_entities.insert(tile_id, entity,);
                Ok((),)
            },
            Err(e,) => Err(e,),
        }
    }

    // Creates a new tile at the given Index
    // Does not clamp the TileIndex, you have to do this yourself first.
    #[allow(dead_code)]
    fn create_tile(
        planet: &Planet,
        base_transform: &Transform,
        tile_id: TileIndex,
        tile_type_opt: Option<TileTypes,>,
        // NOTE: This is pretty ugly
        storages: &mut TileGenerationStorages<'_,>,
    ) -> Result<(TileTypes, Entity,), self::PlanetError,> {
        let entities = &storages.entities;
        let sprite_render_storage = &mut storages.sprite_render;
        let tile_base_storage = &mut storages.tile_base;
        let transform_storage = &mut storages.transform;
        let ingame_entity = &mut storages.ingame_entity;
        let game_sprites = &storages.game_sprites;
        let render_config = &storages.render_config;

        match Self::clamp_tile_index(planet, tile_id,) {
            Ok(tile_id,) => {
                // TODO: Proper algorithm to determine `TileTypes`, based on depth, etc.
                let tile_type = match tile_type_opt {
                    Some(val,) => val,
                    None => {
                        match rand::thread_rng().gen_range(0, 16,) {
                            0 => TileTypes::Magnetite,
                            1 => TileTypes::Pyrolusite,
                            2 => TileTypes::Fossile,
                            3 => TileTypes::Molybdenite,
                            4 => TileTypes::Lava,
                            5 => TileTypes::Rock,
                            6 => TileTypes::Gas,
                            7 => TileTypes::Galena,
                            8 => TileTypes::Bornite,
                            9 => TileTypes::Chromite,
                            10 => TileTypes::Cassiterite,
                            11 => TileTypes::Cinnabar,
                            12 => TileTypes::Dirt,
                            13 => TileTypes::Gold,
                            14 => TileTypes::Empty,
                            15 => TileTypes::Bauxite,
                            _ => {
                                #[cfg(feature = "debug")]
                                debug!("Non-implemented TileType requested in `Chunk::create_tile`, defaulting to `Dirt`.");
                                TileTypes::Dirt
                            },
                        }
                    },
                };

                let entity_sprite_render = EntitySpriteRender::Ore(tile_type,);
                match game_sprites.get(&entity_sprite_render,) {
                    Some(sprite_render,) => {
                        let mut transform = base_transform.clone();
                        transform.translation += Vector3::new(
                            tile_id.1 as f32 * render_config.tile_base_render_dim.1,
                            tile_id.0 as f32 * render_config.tile_base_render_dim.0,
                            0.0,
                        );
                        let tile_base = TileBase {
                            kind: tile_type,
                        };

                        #[cfg(feature = "trace")]
                        trace!("|\t{:?},\t{:?}", tile_type, transform.translation.clone());

                        let entity = entities
                            .build_entity()
                            .with(tile_base, tile_base_storage,)
                            .with(sprite_render.clone(), sprite_render_storage,)
                            .with(transform, transform_storage,)
                            .with(IsIngameEntity, ingame_entity,)
                            .build();

                        Ok((tile_type, entity,),)
                    },
                    None => {
                        Err(PlanetError::TileProblem(TileError::SpriteRenderNotFound(
                            entity_sprite_render,
                        ),),)
                    },
                }
            },
            Err(e,) => Err(e,),
        }
    }
}
