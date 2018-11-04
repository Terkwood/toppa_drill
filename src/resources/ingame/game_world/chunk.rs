use std::{
    collections::{btree_map, BTreeMap,},
    fmt,
};

use rand::*;

use amethyst::{
    core::{cgmath::Vector3, transform::components::Transform},
    ecs::prelude::*,
};

use crate::{
    components::{
        for_ground_entities::TileBase,
        IsIngameEntity,
    },
    entities::{tile::TileTypes, EntitySpriteRender},
    resources::RenderConfig,
};

use super::{
    Planet,
    GameWorldError,ChunkError, TileIndex, TileGenerationStorages,TileError,
};

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
    ) -> Result<Self, GameWorldError> {
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

            return Err(GameWorldError::ChunkProblem(ChunkError::IndexOutOfBounds));
        }

        let chunk_x = chunk_x_f32.trunc();
        let chunk_y = chunk_y_f32.trunc();
        let chunk_id = ChunkIndex(chunk_y as u64, chunk_x as u64);

        match Planet::clamp_chunk_index(planet, chunk_id) {
            Ok(chunk_id) => {
                #[cfg(feature = "debug")]
                debug!("| From transform: {:?}", chunk_id);

                Ok(chunk_id)
            }
            Err(e) => Err(e),
        }
    }
}

impl fmt::Display for ChunkIndex {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("ChunkIndex(")?;
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
    pub fn empty() -> Chunk {
        Chunk {
            tile_index: BTreeMap::new(),
            tile_entities: BTreeMap::new(),
            tile_type: BTreeMap::new(),
        }
    }

    pub fn new(
        planet: &Planet,
        chunk_id: ChunkIndex,
        chunk_dim: (u64, u64),
        // NOTE: This is pretty ugly
        storages: &mut TileGenerationStorages<'_>,
    ) -> Chunk {
        let mut rv = Chunk {
            tile_entities: BTreeMap::new(),
            tile_index: BTreeMap::new(),
            tile_type: BTreeMap::new(),
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
        for y in 0..chunk_dim.0 {
            for x in 0..chunk_dim.1 {
                #[cfg(feature = "trace")]
                trace!("|\ttile number {}", { y * chunk_dim.1 + x });

                let tile_id = TileIndex(y, x);
                if let Err(e) =
                    Self::add_tile(planet, &mut rv, &base_transform, tile_id, None, storages)
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
    pub fn iter_tiletypes(&self) -> btree_map::Iter<'_, TileIndex, TileTypes> {
        self.tile_type.iter()
    }

    /// Returns an iterator over the `tile_entities` field,
    /// which maps `TileIndex <-> TileTypes`.
    pub fn iter_tile_entities(&self) -> btree_map::Iter<'_, TileIndex, Entity> {
        self.tile_entities.iter()
    }

    /// The given tile index gets clamped to the chunk-dim by cutting it off in all directions.
    /// Returns none if the index is out of bounds.
    pub fn clamp_tile_index(planet: &Planet, index: TileIndex) -> Result<TileIndex, GameWorldError> {
        let rv = index;
        if rv.0 >= planet.chunk_dim.0 || rv.1 >= planet.chunk_dim.1 {
            #[cfg(feature = "debug")]
            debug!("tile index originally {:?} is out of bounds.", index);
            Err(GameWorldError::TileProblem(TileError::IndexOutOfBounds))
        } else {
            Ok(rv)
        }
    }

    pub fn add_tile(
        planet: &Planet,
        chunk: &mut Chunk,
        base_transform: &Transform,
        tile_id: TileIndex,
        tile_type_opt: Option<TileTypes>,
        // NOTE: This is pretty ugly
        storages: &mut TileGenerationStorages<'_>,
    ) -> Result<(), self::GameWorldError> {
        match Self::create_tile(planet, base_transform, tile_id, tile_type_opt, storages) {
            Ok((tile_type, entity)) => {
                chunk.tile_type.insert(tile_id, tile_type);
                chunk.tile_index.insert(entity, tile_id);
                chunk.tile_entities.insert(tile_id, entity);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

// private methods
impl Chunk {
    // Creates a new tile at the given Index
    // Does not clamp the TileIndex, you have to do this yourself first.
    #[allow(dead_code)]
    fn create_tile(
        planet: &Planet,
        base_transform: &Transform,
        tile_id: TileIndex,
        tile_type_opt: Option<TileTypes>,
        // NOTE: This is pretty ugly
        storages: &mut TileGenerationStorages<'_>,
    ) -> Result<(TileTypes, Entity), self::GameWorldError> {
        let entities = &storages.entities;
        let sprite_render_storage = &mut storages.sprite_render;
        let tile_base_storage = &mut storages.tile_base;
        let transform_storage = &mut storages.transform;
        let ingame_entity = &mut storages.ingame_entity;
        let game_sprites = &storages.game_sprites;
        let render_config = &storages.render_config;

        match Self::clamp_tile_index(planet, tile_id) {
            Ok(tile_id) => {
                // TODO: Proper algorithm to determine `TileTypes`, based on depth, etc.
                let tile_type = match tile_type_opt {
                    Some(val) => val,
                    None => match rand::thread_rng().gen_range(0, 16) {
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
                        }
                    },
                };

                let entity_sprite_render = EntitySpriteRender::Ore(tile_type);
                match game_sprites.get(&entity_sprite_render) {
                    Some(sprite_render) => {
                        let mut transform = base_transform.clone();
                        transform.translation += Vector3::new(
                            tile_id.1 as f32 * render_config.tile_base_render_dim.1,
                            tile_id.0 as f32 * render_config.tile_base_render_dim.0,
                            0.0,
                        );
                        let tile_base = TileBase { kind: tile_type };

                        #[cfg(feature = "trace")]
                        trace!("|\t{:?},\t{:?}", tile_type, transform.translation.clone());

                        let entity = entities
                            .build_entity()
                            .with(tile_base, tile_base_storage)
                            .with(sprite_render.clone(), sprite_render_storage)
                            .with(transform, transform_storage)
                            .with(IsIngameEntity, ingame_entity)
                            .build();

                        Ok((tile_type, entity))
                    }
                    None => Err(GameWorldError::TileProblem(TileError::SpriteRenderNotFound(
                        entity_sprite_render,
                    ))),
                }
            }
            Err(e) => Err(e),
        }
    }
}