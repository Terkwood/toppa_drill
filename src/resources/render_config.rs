use std::default::Default;

/// A resource holding the render settings that can be adjusted by the player.
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// The size of a tile in Pixels in f32. Needed to calculate offsets,
    /// otherwise width and height could only be retrieved from the Sprite itself, which is not available most of the time.
    /// (y, x)
    pub tile_size: (f32, f32,),

    /// The dimension of a chunk expressed in the count of tiles in x and y direction.
    /// The bigger a chunk is, the more tiles have to be rendered, which takes up both memory and computing time.
    /// A distance of 0 would mean that only the chunk the player currently is on would be rendered.
    pub chunk_render_distance: u64,
    /// Current rendered screen size
    pub view_dim: (u32, u32,),
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig {
            tile_size:  (128.0, 128.0,),
            chunk_render_distance: 1,
            view_dim:              (1920, 1080,),
        }
    }
}

impl RenderConfig {
    /// Creates a new RenderConfig based on input parameters.
    /// (Does a new function even make sense if all members are public?)
    pub fn new(tile_size: (f32, f32), chunk_render_distance: u64, view_dim: (u32, u32)) -> Self {
        RenderConfig {
            tile_size,
            chunk_render_distance,
            view_dim
        }
    }
    /// Sets the tile size, should be done when loading the Tilesheet.
    /// This is not visible to they player, as its an internal measure.
    pub fn set_tile_size(&mut self, width: f32, height: f32) {
        self.tile_size = (width, height);
    }

    /// Sets the number of chunks in each direction to be rendered,
    /// zero would mean only the chunk the player stands on is visible.
    /// Should be adjustable via ingame options.
    pub fn set_chunk_render_distance(&mut self, render_distance: u64) {
        self.chunk_render_distance = render_distance;
    }
}
