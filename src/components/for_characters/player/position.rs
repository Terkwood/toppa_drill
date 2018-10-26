use amethyst::{
    core::transform::components::Transform,
    ecs::prelude::{Component, VecStorage},
};

use resources::{
    ingame::planet::{ChunkIndex, Planet, TileIndex},
    RenderConfig,
};

/// This component stores the current chunk and tile the player resides on.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
                let rv = Position {
                    chunk: chunk_index,
                    tile: tile_index,
                };
                
                {/*turn back to debug later*/}warn!("Created position from transform. {:?}", rv);
                Some(rv)
            } else {
                {/*turn back to debug later*/}warn!("Could not create position from transform.");
                None
            }
        } else {
            None
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        {/*turn back to debug later*/}warn!("Created default position.");
        Self::new(ChunkIndex(0, 0), TileIndex(0, 0))
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
