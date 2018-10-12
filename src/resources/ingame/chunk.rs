use std::collections::BTreeMap;
use amethyst::{
    ecs::prelude::Entity,
};

use components::for_ground_entities::TileTypes;
use resources::RenderConfig;

/// The Index of a tile in a [Chunk](struct.Chunk.html).
/// Used to calculate the render-position of a tile,
/// and to figure out which tile the player currently stands on.
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub struct TileIndex(u32, u32);

/// Small patches of tile entities of a planet.
/// To avoid consuming gigabytes of RAM.
/// Does not implement `Default`, because it's contents are based on the depth it is placed at.
#[derive(Debug)]
pub struct Chunk {
    // A map of individual tiles of the chunk.
    tiles: BTreeMap<TileIndex, Entity>,
    // Grants access to the TileIndex via the Entitiy (which is returned by collision).
    tiles_inversed: BTreeMap<Entity, TileIndex>,
}

impl Chunk{
    pub fn new(depth: u32, render_config: RenderConfig) -> Chunk{
        // TODO: create tiles according to render_configs chunk_dim and tile_base_render_dim using `self.add_tiles`
        Chunk{
            tiles: BTreeMap::new(),
            tiles_inversed: BTreeMap::new(),
        }
    }

    /// Creates 
    pub fn add_tile(&mut self, index: TileIndex, tiletype: TileTypes){
        // TODO: call into Tiles::new()
        // TODO: populate `self.tiles` & `self.tiles_inversed`
        error!("Not implemented yet.");
    }

    /// Tries to figure out the `TileType` from the BTreeMap `tiles` at the given Index.
    /// If the given index exceeds the chunk-dim bounds, returns `None`.
    pub fn get_tile_type(&mut self, index: TileIndex) -> Option<TileTypes>{
        if index.0 > self.chunk_dim.0 || index.1 > self.chunk_dim.1 {return None};

        // TODO: check `self.tiles` for index and figure out the tiletype, use `self.get_tile_entity`

        error!("Not implemented yet.");
        None
    }

    /// Tries to fetch a tile entity from the BTreeMap `tiles` at the given Index.
    /// If the given index exceeds the chunk-dim bounds, returns `None`.
    pub fn get_tile_entity(&mut self, index: TileIndex) -> Option<Entity>{
        if index.0 > self.chunk_dim.0 || index.1 > self.chunk_dim.1 {return None};

        // TODO: check `self.tiles` for index and return the entity.

        error!("Not implemented yet.");
        None
    }

    /// Tries to fetch a tile from the BTreeMap `tiles_inversed` with the given entity.
    /// If the given entity is not part of this chunk, returns `None`.
    pub fn get_tile_index(&mut self, tile: Entity) -> Option<TileIndex>{
        // TODO: Get TileIndex for the given entity, or return `None`
        error!("Not implemented yet.");
        None
    }
}