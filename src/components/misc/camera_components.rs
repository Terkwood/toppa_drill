use amethyst::ecs::prelude::{Component, VecStorage};

use resources::RenderConfig;

/// This component stores some data for the camera.
/// - Zoom
/// - - current
/// - - min
/// - - max
/// - Offset to player
/// TODO: Use as component.
#[derive(Debug)]
pub struct CameraProperties {
    pub player_id: usize,

    pub zoom_cur: f32,
    pub zoom_max: f32,
    pub zoom_min: f32,

    pub offset_to_player: (f32, f32),
}

impl Default for CameraProperties {
    fn default() -> Self {
        CameraProperties {
            player_id: 0,

            zoom_cur: 1.0,
            zoom_max: 2.0,
            zoom_min: 0.5,

            offset_to_player: (-480.0, -270.0),
        }
    }
}

impl CameraProperties {
    #[allow(dead_code)]
    pub fn new(
        player_id: usize,
        zoom_min: f32,
        zoom_max: f32,
        render_config: &RenderConfig,
    ) -> CameraProperties {
        CameraProperties {
            player_id,

            zoom_cur: 1.0,
            zoom_min,
            zoom_max,

            offset_to_player: (
                -(render_config.view_dim.0 as f32) / 2.0,
                -(render_config.view_dim.1 as f32) / 2.0,
            ),
        }
    }
}

impl Component for CameraProperties {
    type Storage = VecStorage<Self>;
}
