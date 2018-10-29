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
        match ChunkIndex::from_transform(transform, render_config, planet) {
            Ok(chunk_index) => {
                match TileIndex::from_transform(transform, chunk_index, render_config, planet) {
                    Ok(tile_index) => {
                        let rv = Position {
                            chunk: chunk_index,
                            tile: tile_index,
                        };

                        #[cfg(feature = "debug")]
                        debug!("Created position from transform. {:?}", rv);
                        Some(rv)
                    }
                    Err(e) => {
                        #[cfg(feature = "debug")]
                        debug!("No valid TileIndex from transform found: {:?}", e);
                        None
                    }
                }
            }
            Err(e) => {
                #[cfg(feature = "debug")]
                debug!("No valid ChunkIndex from transform found: {:?}", e);
                None
            }
        }
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
