/// A resource holding the render settings that can be adjusted by the player.
pub struct RenderConfig{
    /// The base render dimension of each tile, can be different from the size of the image.
    /// Scaling is based on this value.
    pub tile_base_render_dim: (f32, f32),
}