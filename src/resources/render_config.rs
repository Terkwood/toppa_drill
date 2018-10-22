use std::default::Default;

/// A resource holding the render settings that can be adjusted by the player.
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// The base render dimension of each tile, can be different from the size of the image.
    /// Scaling is based on this value.
    /// (y, x)
    pub tile_base_render_dim: (f32, f32),
    /// The dimension of a chunk expressed in the count of tiles in x and y direction.
    /// The bigger a chunk is, the more tiles have to be rendered, which takes up both memory and computing time.
    pub chunk_render_distance: u64,
    /// Current rendered screen size
    pub view_dim: (u32, u32),
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig {
            tile_base_render_dim: (64.0, 64.0),
            chunk_render_distance: 1,
            view_dim: (1920, 1080),
        }
    }
}
