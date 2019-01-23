use std::{
    fmt,
};



use amethyst::{
    core::transform::components::Transform,
};

use crate::resources::RenderConfig;

use super::{
    ChunkIndex, Planet, GameWorldError, TileError,
};

/// The Index of a tile in a [Chunk](struct.Chunk.html).
/// Used to calculate the render-position of a tile,
/// and to figure out which tile the player currently stands on.
///
/// Uses (rows, columns).
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct TileIndex(pub u64, pub u64);

impl TileIndex {
    /// Convenience function returning only the TileIndex. Best used when Chunk Index is known
    #[allow(dead_code)]
    pub fn from_transform(
        transform: &Transform,
        chunk_index: ChunkIndex,
        render_config: &RenderConfig,
        planet: &Planet,
    ) -> Result<Self, GameWorldError> {
        let x_transl = transform.translation().x;
        let y_transl = transform.translation().y;

        let tile_width_f32 = render_config.tile_size.1;
        let tile_height_f32 = render_config.tile_size.0;
        let chunk_width_f32 = planet.chunk_dim.1 as f32 * tile_width_f32;
        let chunk_height_f32 = planet.chunk_dim.0 as f32 * tile_height_f32;

        let chunk_offset_x = chunk_index.1 as f32 * chunk_width_f32;
        let chunk_offset_y = chunk_index.0 as f32 * chunk_height_f32;

        let x_chunk_transl = x_transl - (chunk_offset_x);
        let y_chunk_transl = y_transl - (chunk_offset_y);
        // Supposedly more accurate, but is it necessary?
        /*let x_chunk_transl = chunk_x.mul_add(-chunk_width_f32, x_transl);
        let y_chunk_transl = chunk_y.mul_add(-chunk_height_f32, y_transl);*/

        let tile_id_x_f32 = (x_chunk_transl / tile_width_f32).trunc();
        let tile_id_y_f32 = (y_chunk_transl / tile_height_f32).trunc();
        let tile_id_x = tile_id_x_f32.trunc() as u64;
        let tile_id_y = tile_id_y_f32.trunc() as u64;
        if tile_id_x_f32.is_sign_negative()
            || tile_id_y_f32.is_sign_negative()
            || tile_id_x >= planet.chunk_dim.1
            || tile_id_y >= planet.chunk_dim.0
        {
            return Err(GameWorldError::TileProblem(TileError::IndexOutOfBounds));
        }

        Ok(TileIndex(tile_id_y, tile_id_x))
    }
}

impl fmt::Display for TileIndex {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("TileIndex(")?;
        fmt.write_str(&self.0.to_string())?;
        fmt.write_str(", ")?;
        fmt.write_str(&self.1.to_string())?;
        fmt.write_str(")")?;
        Ok(())
    }
}