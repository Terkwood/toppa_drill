use std::{
    collections::{btree_map, BTreeMap,},
    fmt,
};

use rand::*;

use amethyst::{
    core::{nalgebra::Vector3, transform::components::Transform},
    ecs::prelude::*,
    renderer::Flipped,
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
        let x_transl = transform.translation().x;
        let y_transl = transform.translation().y;

        let tile_width_f32 = render_config.tile_size.1;
        let tile_height_f32 = render_config.tile_size.0;
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
            transform.set_position(Vector3::new(
                chunk_id.1 as f32
                    * (planet.chunk_dim.1 as f32 * render_config.tile_size.1),
                chunk_id.0 as f32
                    * (planet.chunk_dim.0 as f32 * render_config.tile_size.0),
                0.0,
            ));
            transform
        };
        #[cfg(feature = "trace")]
        trace!(
            "|\tbase translation: {:?}",
            base_transform.translation().clone()
        );

        // TODO: Actual tile generation algorithm
        for y in 0..chunk_dim.0 {
            for x in 0..chunk_dim.1 {
                #[cfg(feature = "trace")]
                trace!("|\ttile number {}", { y * chunk_dim.1 + x });

                let tile_id = TileIndex(y, x);
                if let Err(e) =
                    Self::add_tile(planet, &mut rv, chunk_id, &base_transform, tile_id, None, storages)
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
        chunk_id: ChunkIndex,
        base_transform: &Transform,
        tile_id: TileIndex,
        tile_type_opt: Option<TileTypes>,
        // NOTE: This is pretty ugly
        storages: &mut TileGenerationStorages<'_>,
    ) -> Result<(), self::GameWorldError> {
        match Self::create_tile(planet, chunk_id, base_transform, tile_id, tile_type_opt, storages) {
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
    // TODO: Guard against existing tiles.
    #[allow(dead_code)]
    fn create_tile(
        planet: &Planet,
        chunk_id: ChunkIndex,
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
        let flipped_storage = &mut storages.flipped_vertical;

        match Self::clamp_tile_index(planet, tile_id) {
            Ok(tile_id) => {
                // TODO: Proper algorithm to determine `TileTypes`, based on depth, etc.
                let tile_type = match tile_type_opt {
                    Some(val) => val,
                    None => {                        
                        //if chunk_id.0 < (planet.planet_dim.0 - 1) {
                        if chunk_id.0 > 0 {
                            let chunk_count_y = planet.planet_dim.0 as f32;
                            let relative_depth = (chunk_id.0 as f32) / chunk_count_y;

                            // TODO: Meh.... <TEST>
                            random_tile(relative_depth)
                            // <\TEST>
                        }
                        else{
                            // Upmost Chunk is always empty.
                            TileTypes::Empty
                        }
                    },
                };

                let entity_sprite_render = EntitySpriteRender::Ore(tile_type);
                match game_sprites.get(&entity_sprite_render) {
                    Some(sprite_render) => {
                        let mut transform = base_transform.clone();
                        transform.move_global(Vector3::new(
                            tile_id.1 as f32 * render_config.tile_size.1,
                            tile_id.0 as f32 * render_config.tile_size.0,
                            0.0,
                        ));
                        let tile_base = TileBase { kind: tile_type };

                        #[cfg(feature = "trace")]
                        trace!("|\t{:?},\t{:?}", tile_type, transform.translation().clone());

                        let entity = entities
                            .build_entity()
                            .with(tile_base, tile_base_storage)
                            .with(sprite_render.clone(), sprite_render_storage)
                            .with(transform, transform_storage)
                            .with(IsIngameEntity, ingame_entity)
                            .with(Flipped::Vertical, flipped_storage) 
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


/// TODO: Banish this evil from our world!
/// TODO: No. seriously.
fn random_tile(rel_depth: f32) -> TileTypes {
    let rng_section: usize = {rel_depth * 10000.0}.trunc() as usize;

    let upper_bound = match rng_section {
        section if section < 1250 => {
            3
        },
        section if section >= 1250 && section < 2500 => {
            4
        },
        section if section >= 2500 && section < 3750 => {
            5
        },
        section if section >= 3750 && section < 5000 => {
            6
        },
        section if section >= 5000 && section < 6250 => {
            7
        },
        section if section >= 6250 && section < 7500 => {
            8
        },
        section if section >= 7500 && section < 8750 => {
            9
        },
        section if section >= 8750 && section < 10000 => {
            10
        },
        _ => {
            11
        },
    };

    let out_of_hundred_one = rand::thread_rng().gen_range(0, 100);
    let out_of_hundred_two = rand::thread_rng().gen_range(0, 100);
    
    match rand::thread_rng().gen_range(0, upper_bound) {
        low if (low <= 3) => {
            match out_of_hundred_one {
                useless if useless < 35 => {
                    match out_of_hundred_two {
                        a if a < 99 => TileTypes::Dirt,
                        _ => TileTypes::Fossile,
                    }
                },
                ore if (ore >= 35) && (ore < 92) => {
                    match out_of_hundred_two {
                        a if a < 50 => TileTypes::Magnetite,
                        _ => TileTypes::Pyrolusite,
                    }
                },
                _ => TileTypes::Empty,
            }
        },
        4 => {
            match out_of_hundred_one {
                useless if useless < 30 => {
                    match out_of_hundred_two {
                        a if a < 90 => TileTypes::Dirt,
                        b if (b >= 90) && (b < 98) => TileTypes::Rock,
                        _ => TileTypes::Fossile,
                    }
                },
                ore if (ore >= 30) && (ore < 93) => {
                    match out_of_hundred_two {
                        a if a < 40 => TileTypes::Magnetite,
                        b if (b >= 40) && (b < 80) => TileTypes::Pyrolusite,
                        _ => TileTypes::Bauxite,
                    }
                },
                _ => TileTypes::Empty,
            }
        },
        5 => {
            match out_of_hundred_one {
                useless if useless < 30 => {
                    match out_of_hundred_two {
                        a if a < 85 => TileTypes::Dirt,
                        b if (b >= 85) && (b < 98) => TileTypes::Rock,
                        _ => TileTypes::Fossile,
                    }
                },
                ore if (ore >= 30) && (ore < 94) => {
                    match out_of_hundred_two {
                        a if a < 30 => TileTypes::Magnetite,
                        b if (b >= 30) && (b < 60) => TileTypes::Pyrolusite,
                        c if (c >= 60) && (c < 80) => TileTypes::Bauxite,
                        _ => TileTypes::Cassiterite,
                    }
                },
                _ => TileTypes::Empty,
            }
        },
        6 => {
            match out_of_hundred_one {
                useless if useless < 30 => {
                    match out_of_hundred_two {
                        a if a < 70 => TileTypes::Dirt,
                        b if (b >= 70) && (b < 94) => TileTypes::Rock,
                        b if (b >= 94) && (b < 98) => TileTypes::Lava,
                        _ => TileTypes::Fossile,
                    }
                },
                ore if (ore >= 30) && (ore < 95) => {
                    match out_of_hundred_two {
                        a if a < 20 => TileTypes::Magnetite,
                        b if (b >= 20) && (b < 40) => TileTypes::Pyrolusite,
                        c if (c >= 40) && (c < 62) => TileTypes::Bauxite,
                        d if (d >= 62) && (d < 84) => TileTypes::Cassiterite,
                        _ => TileTypes::Chromite,
                    }
                },
                _ => TileTypes::Empty,
            }
        },
        7 => {
            match out_of_hundred_one {
                useless if useless < 30 => {
                    match out_of_hundred_two {
                        a if a < 50 => TileTypes::Dirt,
                        b if (b >= 50) && (b < 88) => TileTypes::Rock,
                        b if (b >= 88) && (b < 97) => TileTypes::Lava,
                        _ => TileTypes::Fossile,
                    }
                },
                ore if (ore >= 30) && (ore < 97) => {
                    match out_of_hundred_two {
                        a if a < 16 => TileTypes::Magnetite,
                        b if (b >= 16) && (b < 32) => TileTypes::Pyrolusite,
                        c if (c >= 32) && (c < 50) => TileTypes::Bauxite,
                        d if (d >= 50) && (d < 68) => TileTypes::Cassiterite,
                        e if (e >= 68) && (e < 84) => TileTypes::Chromite,
                        _ => TileTypes::Bornite,
                    }
                },
                _ => TileTypes::Empty,
            }
        },
        8 => {
            match out_of_hundred_one {
                useless if useless < 40 => {
                    match out_of_hundred_two {
                        a if a < 36 => TileTypes::Dirt,
                        b if (b >= 36) && (b < 86) => TileTypes::Rock,
                        b if (b >= 86) && (b < 96) => TileTypes::Lava,
                        b if (b == 96) => TileTypes::Gas,
                        _ => TileTypes::Fossile,
                    }
                },
                ore if (ore >= 40) && (ore < 98) => {
                    match out_of_hundred_two {
                        a if a < 8 => TileTypes::Magnetite,
                        b if (b >= 8) && (b < 16) => TileTypes::Pyrolusite,
                        c if (c >= 16) && (c < 32) => TileTypes::Bauxite,
                        d if (d >= 32) && (d < 48) => TileTypes::Cassiterite,
                        e if (e >= 48) && (e < 68) => TileTypes::Chromite,
                        f if (f >= 68) && (f < 88) => TileTypes::Bornite,
                        _ => TileTypes::Galena,
                    }
                },
                _ => TileTypes::Empty,
            }
        },
        9 => {
            match out_of_hundred_one {
                useless if useless < 55 => {
                    match out_of_hundred_two {
                        a if a < 15 => TileTypes::Dirt,
                        b if (b >= 15) && (b < 65) => TileTypes::Rock,
                        c if (c >= 65) && (c < 82) => TileTypes::Lava,
                        d if (d >= 82) && (d < 95) => TileTypes::Gas,
                        _ => TileTypes::Fossile,
                    }
                },
                ore if (ore >= 55) && (ore < 99) => {
                    match out_of_hundred_two {
                        a if a < 4 => TileTypes::Magnetite,
                        b if (b >= 4) && (b < 8) => TileTypes::Pyrolusite,
                        c if (c >= 8) && (c < 14) => TileTypes::Bauxite,
                        d if (d >= 14) && (d < 22) => TileTypes::Cassiterite,
                        e if (e >= 22) && (e < 42) => TileTypes::Chromite,
                        f if (f >= 42) && (f < 62) => TileTypes::Bornite,
                        g if (g >= 62) && (g < 90) => TileTypes::Galena,
                        _ => TileTypes::Molybdenite,
                    }
                },
                _ => TileTypes::Empty,
            }
        },
        10 => {
            match out_of_hundred_one {
                useless if useless < 64 => {
                    match out_of_hundred_two {
                        a if a < 10 => TileTypes::Dirt,
                        b if (b >= 10) && (b < 55) => TileTypes::Rock,
                        c if (c >= 55) && (c < 77) => TileTypes::Lava,
                        d if (d >= 77) && (d < 97) => TileTypes::Gas,
                        _ => TileTypes::Fossile,
                    }
                },
                ore if (ore >= 64) && (ore < 100) => {
                    match out_of_hundred_two {
                        a if a < 1 => TileTypes::Magnetite,
                        b if (b >= 1) && (b < 2) => TileTypes::Pyrolusite,
                        c if (c >= 2) && (c < 7) => TileTypes::Bauxite,
                        d if (d >= 7) && (d < 12) => TileTypes::Cassiterite,
                        e if (e >= 12) && (e < 32) => TileTypes::Chromite,
                        f if (f >= 32) && (f < 52) => TileTypes::Bornite,
                        g if (g >= 52) && (g < 70) => TileTypes::Galena,
                        h if (h >= 70) && (h < 88) => TileTypes::Molybdenite,
                        _ => TileTypes::Gold,
                    }
                },
                _ => TileTypes::Empty,
            }
        },
        11 => {
            match out_of_hundred_one {
                useless if useless < 72 => {
                    match out_of_hundred_two {
                        a if a < 1 => TileTypes::Dirt,
                        b if (b >= 1) && (b < 42) => TileTypes::Rock,
                        c if (c >= 42) && (c < 65) => TileTypes::Lava,
                        d if (d >= 65) && (d < 99) => TileTypes::Gas,
                        _ => TileTypes::Fossile,
                    }
                },
                ore if (ore >= 72) && (ore < 100) => {
                    match out_of_hundred_two {
                        a if a < 1 => TileTypes::Magnetite,
                        b if (b >= 1) && (b < 2) => TileTypes::Pyrolusite,
                        c if (c >= 2) && (c < 4) => TileTypes::Bauxite,
                        d if (d >= 4) && (d < 10) => TileTypes::Cassiterite,
                        e if (e >= 10) && (e < 24) => TileTypes::Chromite,
                        f if (f >= 24) && (f < 40) => TileTypes::Bornite,
                        g if (g >= 40) && (g < 64) => TileTypes::Galena,
                        h if (h >= 64) && (h < 80) => TileTypes::Molybdenite,
                        _ => TileTypes::Gold,
                    }
                },
                _ => TileTypes::Empty,
            }
        },
        _ => {
            #[cfg(feature = "debug")]
            debug!("Non-implemented TileType requested in `Chunk::create_tile`, defaulting to `Empty`.");
            TileTypes::Empty
        }
    }
    /* Levels of ores
    TileTypes::Empty, 0

    TileTypes::Fossile, 0

    TileTypes::Dirt, 0
    TileTypes::Lava, 6
    TileTypes::Rock, 4
    TileTypes::Gas, 8
    
    TileTypes::Magnetite, 3
    TileTypes::Pyrolusite, 3
    TileTypes::Molybdenite, 9 
    TileTypes::Galena, 8
    TileTypes::Bornite, 7
    TileTypes::Chromite, 6
    TileTypes::Cassiterite, 5
    TileTypes::Cinnabar, 5
    TileTypes::Bauxite, 4
    TileTypes::Gold, 10
    */
}
