//! Makes the camera follow the player.

use std::collections::HashSet;

use amethyst::{
    core::transform::components::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    shrev::EventChannel,
};

use {
    components::for_characters::{player, PlayerBase},
    events::planet_events::ChunkEvent,
    resources::{
        ingame::{
            planet::{ChunkError, ChunkIndex, PlanetError, TileError, TileIndex},
            GameSessionData,
        },
        RenderConfig,
    },
};

/// TODO: Reference swapping instead of toggle-magic! Reduces duplicate code.
/// Calculates the players position expressed in [`ChunkIndex`](struct.ChunkIndex.html) and [`TileIndex`](struct.TileIndex.html).
/// Tries to calculate new `TileIndex` based on current `Transform` and previous `Position.chunk`-ChunkIndex.
/// If that fails, calculates new ChunkIndex based only on current `Transform`, and then the new `TileIndex`.
pub struct PlayerPositionSystem {
    prev_chunks: HashSet<ChunkIndex>,
    cur_chunks: HashSet<ChunkIndex>,
}

impl Default for PlayerPositionSystem {
    fn default() -> Self {
        PlayerPositionSystem {
            prev_chunks: HashSet::with_capacity(9),
            cur_chunks: HashSet::with_capacity(9),
        }
    }
}

impl<'s> System<'s> for PlayerPositionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, PlayerBase>,
        WriteStorage<'s, player::Position>,
        ReadExpect<'s, GameSessionData>,
        ReadExpect<'s, RenderConfig>,
        WriteExpect<'s, EventChannel<ChunkEvent>>,
    );

    fn run(
        &mut self,
        (
            transforms,
            players,
            mut player_positions,
            session_data,
            render_config,
            mut chunk_event_channel,
        ): Self::SystemData,
    ) {
        for (transform, player, mut player_pos) in
            (&transforms, &players, &mut player_positions).join()
        {
            let chunk_index = player_pos.chunk;
            let planet_ref = &session_data.planet;
            match TileIndex::from_transform(transform, chunk_index, &render_config, planet_ref) {
                Ok(tile_index) => {
                    // Player still on the same chunk. Easy-peasy
                    player_pos.tile = tile_index;
                }
                Err(e) => {
                    match e {
                        PlanetError::TileProblem(TileError::IndexOutOfBounds) => {
                            // Player on a new chunk.
                            match ChunkIndex::from_transform(transform, &render_config, planet_ref)
                            {
                                Ok(chunk_index) => {
                                    match TileIndex::from_transform(
                                        transform,
                                        chunk_index,
                                        &render_config,
                                        planet_ref,
                                    ) {
                                        Ok(tile_index) => {
                                            let prev_chunk = player_pos.chunk;
                                            self.prev_chunks.clear();
                                            for index in self.cur_chunks.drain() {
                                                // No need to check the returned boolean, as the HashSet has been `.drain()`ed previously.
                                                self.prev_chunks.insert(index);
                                            }

                                            // Updating player position component
                                            player_pos.tile = tile_index;
                                            player_pos.chunk = chunk_index;

                                            // Populating the current chunk HashSet
                                            for y in (prev_chunk.0
                                                - render_config.chunk_render_distance)
                                                ..=(prev_chunk.0
                                                    + render_config.chunk_render_distance)
                                            {
                                                for x in (prev_chunk.1
                                                    - render_config.chunk_render_distance)
                                                    ..=(prev_chunk.1
                                                        + render_config.chunk_render_distance)
                                                {
                                                    // No need to check the returned boolean, as the HashSet has been `.drain()`ed previously.
                                                    self.cur_chunks.insert(ChunkIndex(y, x));
                                                }
                                            }
                                            // Comparing the current and previous HashSets (`.difference()` returns only those NOT present in the other)
                                            let cur_chunks = self.cur_chunks.clone();
                                            let prev_chunks = self.prev_chunks.clone();
                                            let chunks_to_delete =
                                                self.prev_chunks.difference(&cur_chunks);
                                            let chunks_to_load =
                                                self.cur_chunks.difference(&prev_chunks);

                                            for &index in chunks_to_delete {
                                                #[cfg(feature = "debug")]
                                                warn!("Requesting load for chunk {:?}.", index);
                                                chunk_event_channel.single_write(
                                                    ChunkEvent::RequestingUnload(index),
                                                );
                                            }
                                            for &index in chunks_to_load {
                                                #[cfg(feature = "debug")]
                                                warn!("Requesting unload for chunk {:?}.", index);
                                                chunk_event_channel.single_write(
                                                    ChunkEvent::RequestingLoad(index),
                                                );
                                            }
                                        }
                                        Err(e) => {
                                            error!("Couldn't find TileIndex, although new ChunkIndex was calculated: {:?}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Error calculating ChunkIndex from transform: {:?}", e)
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
