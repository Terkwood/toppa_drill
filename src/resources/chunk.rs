use std::collections::BTreeMap;
use amethyst::{
    ecs::prelude::Entity,
};

use components::for_ground_entities::TileTypes;

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
    /// The dimension of a chunk expressed in the count of tiles in x and y direction.
    /// Chunks all have the same dimension, therefore it is stored in the planet, isntead of in every chunk.
    pub chunk_dim: (u32, u32),
    // A map of individual tiles of the chunk.
    tiles: BTreeMap<TileIndex, Entity>,
    // Grants access to the TileIndex via the Entitiy (which is returned by collision).
    tiles_inversed: BTreeMap<Entity, TileIndex>,
}

impl Chunk{
    pub fn new(depth: u32, dim: (u32, u32)) -> Chunk{
        Chunk{
            chunk_dim: dim,
            tiles: BTreeMap::new(),
            tiles_inversed: BTreeMap::new(),
        }
    }

    /// Tries to fetch a tile from the BTreeMap.
    /// If the given index exceeds the chunk-dim bounds, returns `None`.
    pub fn get_tiletype(&mut self, index: TileIndex) -> Option<TileTypes>{
        if index.0 > self.chunk_dim.0 || index.1 > self.chunk_dim.1 {return None};

        error!("Not implemented yet.");
        None
    }
}