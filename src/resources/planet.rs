//! This module contains everything necessary for the planet.
//! - Chunk
//! - TileIndex
//! - ChunkIndex
//! - PlanetIndex (in case multiple planets might be available later)

#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub struct PlanetIndex(i32, i32);

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub struct ChunkIndex(i32, i32);

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub struct TileIndex(i32, i32);

/// Small patches of tile entities of a planet.
/// To avoid consuming gigabytes of RAM.
pub struct Chunk {}

/// The planet a player resides on.
/// Consists of individual chunks of tile entities.
pub struct Planet {}

/// A galaxy can fit a large number of planets.
pub struct Galaxy {}
