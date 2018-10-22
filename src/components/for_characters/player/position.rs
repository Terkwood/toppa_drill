use amethyst::{
    core::transform::components::Transform,
    ecs::prelude::{Component, VecStorage},
};

use resources::{
    ingame::planet::{ChunkIndex, Planet, TileIndex},
    RenderConfig,
};

/// This component stores the current chunk and tile the player resides on.
#[derive(Debug, Clone)]
pub struct Position {
    pub chunk: ChunkIndex,
    pub tile: TileIndex,
}

impl Position {
    pub fn new(chunk_index: ChunkIndex, tile_index: TileIndex) -> Self {
        Position {
            chunk: chunk_index,
            tile: tile_index,
        }
    }

    pub fn from_transform(
        transform: &Transform,
        render_config: &RenderConfig,
        planet: &Planet,
    ) -> Option<Self> {
        if let Some(chunk_index) = ChunkIndex::from_transform(transform, render_config, planet) {
            if let Some(tile_index) =
                TileIndex::from_transform(transform, chunk_index, render_config, planet)
            {
                Some(Position {
                    chunk: chunk_index,
                    tile: tile_index,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(ChunkIndex(0, 0), TileIndex(0, 0))
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
