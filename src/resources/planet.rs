//! This module contains everything necessary for the planet.
//! - Chunk
//! - TileIndex
//! - ChunkIndex
//! - PlanetIndex (in case multiple planets might be available later)

use std::collections::HashMap;

use amethyst::{
    ecs::{World, Entity, Builder,},
};

use super::chunk::{Chunk, TileIndex};
use components::for_ground_entities::TileTypes;


/// The Index of a chunk in a [Planet](struct.Planet.html).
/// Used to calculate the render-position of a chunk,
/// and to figure out which chunk the player currently resides in.
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub struct ChunkIndex(u32, u32);

/// The planet a player resides on.
/// Consists of individual chunks of tile entities.
#[derive(Debug)]
pub struct Planet {
    /// The dimension of a planet expressed in the count of chunks in x and y direction.
    /// Differs based on the setting `Planet size` when creating a new game.
    pub planet_dim: (u32, u32),
    // A map of individual chunks of the planet, only a small number is loaded at a time.
    // Chunks that are too far from the player get serialized and stored to the disk.
    chunks: HashMap<ChunkIndex, Chunk>,
}

impl Default for Planet{
    fn default()-> Self{
        let planet_dim = (128, 128);
        Planet::new(planet_dim)
    }
}

impl<'a> Planet{
    pub fn new(
        planet_dim: (u32, u32), 
    )-> Planet{
        Planet{
            planet_dim,
            chunks: HashMap::with_capacity(9),
        }
    }

    /// Tries to fetch a chunk from the HashMap.
    /// If the given index exceeds the planet-dim bounds, it gets [clamped](struct.Planet.html#method.clamp_chunk_index).
    /// Returns `None` if no chunk at the given index exists.
    /// Try calling `new_chunk()` in that case.
    pub fn get_chunk(&mut self, index: ChunkIndex) -> Option<&Chunk>{
        let clamped_index = self.clamp_chunk_index(index);
        self.chunks.get(&clamped_index)
    }

    /// Tile indexes out of tile-dim bounds return `None`.
    pub fn get_tiletype(chunk: ChunkIndex, tile: TileIndex) -> Option<TileTypes>{
        // TODO:
        error!("Not implemented yet.");
        None
    }

    /// The given chunk index gets clamped to the planet-dim by wrapping it in x-direction and cutting it off in y-direction.
    pub fn clamp_chunk_index(&self, index: ChunkIndex) -> ChunkIndex{
        let mut rv = index;
        if rv.0 > self.planet_dim.0 || rv.0 < self.planet_dim.0{
            rv.0 = (rv.0 % self.planet_dim.0);// + self.planet_dim.0;
            info!("chunk X-index originally was: {:?}, got clamped to: {:?}, with planet_dim.0: {:?}", index.0, rv.0, self.planet_dim.0);
        }
        if rv.1 > self.planet_dim.1 || rv.1 < self.planet_dim.1{
            rv.1 = (rv.1 % self.planet_dim.1);// + self.planet_dim.1;
            info!("chunk Y-index originally was: {:?}, got clamped to: {:?}, with planet_dim.1: {:?}", index.1, rv.1, self.planet_dim.1);
        }
        rv
    }

    pub fn new_chunk(
        &mut self,
        chunk_id: ChunkIndex,
    ){
        // TODO:
        error!("Not implemented yet.");
        self.chunks.insert(ChunkIndex(1, 1), Chunk::new(1, (32, 64)));
    }
}
