use amethyst::{
    core::transform::components::Transform,
    ecs::prelude::{Component, VecStorage},
};

use resources::{
    ingame::planet::{ChunkIndex, Planet, PlanetError, TileIndex},
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
    ) -> Result<Self, PlanetError> {
        let chunk_id = ChunkIndex::from_transform(transform, render_config, planet)?;
        let tile_id = TileIndex::from_transform(transform, chunk_id, render_config, planet)?;
        let rv = Position {
            chunk: chunk_id,
            tile: tile_id,
        };

        #[cfg(feature = "debug")]
        debug!("| Created position from transform. {:?}", rv);
        Ok(rv)
    }
}

impl Default for Position {
    fn default() -> Self {
        #[cfg(feature = "debug")]
        debug!("Created default position.");
        Self::new(ChunkIndex(0, 0), TileIndex(0, 0))
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
