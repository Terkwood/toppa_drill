//! Makes the camera follow the player.

use amethyst::{
    core::transform::components::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::Camera,
};

use {
    components::for_characters::{player, TagPlayer},
    resources::{
        ingame::{
            planet::{ChunkIndex, Planet, TileIndex},
            GameSessionData,
        },
        RenderConfig,
    },
};

/// Calculates the players position expressed in [`ChunkIndex`](struct.ChunkIndex.html) and [`TileIndex`](struct.TileIndex.html).
/// Tries to calculate new `TileIndex` based on current `Transform` and previous `Position.chunk`-ChunkIndex.
/// If that fails, calculates new ChunkIndex based only on current `Transform`, and then the new `TileIndex`.
pub struct PlayerPositionSystem;

impl<'s> System<'s> for PlayerPositionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, TagPlayer>,
        WriteStorage<'s, player::Position>,
        ReadExpect<'s, GameSessionData>,
        ReadExpect<'s, RenderConfig>,
    );

    fn run(
        &mut self,
        (transforms, players, mut player_positions, session_data, render_config): Self::SystemData,
    ) {
        debug!("+------");
        for (transform, player, mut player_pos) in
            (&transforms, &players, &mut player_positions).join()
        {
            debug!(
                "| previous:\tplayer: {:?}\t pos: {:?}",
                player.id, player_pos
            );
            let chunk_index = player_pos.chunk;
            let planet_ref = &session_data.planet;
            if let Some(tile_index) =
                TileIndex::from_transform(transform, chunk_index, &render_config, planet_ref)
            {
                player_pos.tile = tile_index;
            } else {
                if let Some(chunk_index) =
                    ChunkIndex::from_transform(transform, &render_config, planet_ref)
                {
                    if let Some(tile_index) = TileIndex::from_transform(
                        transform,
                        chunk_index,
                        &render_config,
                        planet_ref,
                    ) {
                        player_pos.tile = tile_index;
                        player_pos.chunk = chunk_index;
                    } else {
                        error!("Player {:?}'s TileIndex is out of chunk bounds, although new ChunkIndex was calculated.", player.id);
                    }
                } else {
                    error!("Player {:?}'s ChunkIndex is out of planet bounds, maybe at negative transforms.", player.id);
                }
            }
            debug!("| now:\tplayer: {:?}\t pos: {:?}", player.id, player_pos);
        }
        debug!("+------");
    }
}
