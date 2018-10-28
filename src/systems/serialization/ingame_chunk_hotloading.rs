#![allow(unused_imports)]
use std::{
    fs,
    ops::{Deref, DerefMut},
    path::*,
    string::ToString,
    vec::Vec,
};

use ron;
use serde::Serializer;

use amethyst::{
    core::{
        specs::saveload::{
            DeserializeComponents, SerializeComponents, U64Marker, U64MarkerAllocator,
        },
        timing::Time,
        transform::components::Transform,
    },
    ecs::prelude::*,
    renderer::SpriteRender,
    shred::Resources,
    shrev::EventChannel,
};

use {
    components::{for_characters::PlayerBase, for_ground_entities::TileBase},
    entities::tile::TileTypes,
    events::planet_events::ChunkEvent,
    resources::{
        ingame::{
            planet::{Chunk, ChunkIndex, Planet, TileGenerationStorages, TileIndex},
            GameSessionData, GameSprites, SavegamePaths,
        },
        RenderConfig,
    },
};

/// TODO: Everything
pub struct HotChunkSystem {
    event_reader: Option<ReaderId<ChunkEvent>>,
    chunks_to_load: Vec<ChunkIndex>,
    chunks_to_unload: Vec<ChunkIndex>,
}

impl HotChunkSystem {
    pub fn new() -> Self {
        HotChunkSystem {
            event_reader: None,
            chunks_to_load: Vec::with_capacity(9),
            chunks_to_unload: Vec::with_capacity(9),
        }
    }
}

impl<'a> System<'a> for HotChunkSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, TileBase>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteExpect<'a, GameSessionData>,
        WriteExpect<'a, EventChannel<ChunkEvent>>,
        ReadExpect<'a, SavegamePaths>,
        ReadExpect<'a, GameSprites>,
        ReadExpect<'a, RenderConfig>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut tag_tiles,
            mut sprite_renders,
            mut transforms,
            mut session_data,
            mut chunk_events,
            paths,
            game_sprites,
            render_config,
        ): Self::SystemData,
    ) {
        let mut tile_gen = TileGenerationStorages {
            entities: entities,
            tile_base: tag_tiles,
            sprite_render: sprite_renders,
            transform: transforms,
            game_sprites: game_sprites,
            render_config: render_config,
        };
        //self.chunks_to_load.clear();
        //self.chunks_to_unload.clear();

        if let Some(ref mut event_reader) = self.event_reader {
            for &event in chunk_events.read(event_reader) {
                match event {
                    ChunkEvent::RequestingLoad(chunk_index) => {
                        self.chunks_to_load.push(chunk_index);
                    }
                    ChunkEvent::RequestingUnload(chunk_index) => {
                        self.chunks_to_unload.push(chunk_index);
                    }
                    _ => continue,
                };
            }

            for chunk_id in self.chunks_to_unload.drain(0..) {
                if let Some(chunk) = session_data.planet.remove_chunk(&chunk_id) {
                    let mut ser_chunk = ron::ser::Serializer::new(Some(Default::default()), true);
                    /* NOTE: Use this to save disk space!
                    let mut ser_chunk = ron::ser::Serializer::new(Some(Default::default()), false);
                    */
                    {
                        use serde::ser::SerializeMap;
                        #[cfg(feature = "debug")]
                        warn!("serializing chunk {:?}", chunk_id);

                        if let Ok(mut serseq) = ser_chunk.serialize_map(None) {
                            for (tile_index, tile_type) in chunk.iter_tiles() {
                                if let Err(e) = serseq.serialize_key::<TileIndex>(&tile_index) {
                                    error!(
                                        "Error serializing key of Tile {:?} in Chunk {:?}: {:?}",
                                        tile_index, chunk_id, e
                                    );
                                }
                                if let Err(e) = serseq.serialize_value::<TileTypes>(&tile_type) {
                                    error!(
                                        "Error serializing value of Tile {:?} in Chunk {:?}: {:?}",
                                        tile_index, chunk_id, e
                                    );
                                }
                                /* NOTE: Use this to save disk space!
                                serseq.serialize_key::<u64>(&{(tile_index.1 * render_config.chunk_render_dim.0 + tile_index.0) as u64}).unwrap();
                                serseq.serialize_value::<u64>(&{*tile_type as u64}).unwrap();
                                */
                            }
                            if let Err(e) = serseq.end() {
                                error!("Error ending serialize for chunk {:?}: {:?}", chunk_id, e);
                            }
                        } else {
                            error!("Error starting serialize for chunk {:?}.", chunk_id);
                        }
                    }

                    let mut chunk_file_path = paths.chunk_dir_path.clone();
                    chunk_file_path = chunk_file_path.join(Path::new(
                        &{ (chunk_id.1 * session_data.planet.planet_dim.0 + chunk_id.0) as u64 }
                            .to_string(),
                    ));
                    chunk_file_path.set_extension("ron");

                    if let Err(e) =
                        fs::write(chunk_file_path.clone(), ser_chunk.into_output_string())
                    {
                        error!(
                            "Writing chunk {:?} at '{:?}' resulted in {:?}",
                            chunk_id, chunk_file_path, e
                        );
                    }
                }
            }

            for chunk_id in self.chunks_to_load.drain(0..) {
                // TODO: Use this path to look for chunk
                let mut chunk_file_path = paths.chunk_dir_path.clone();
                chunk_file_path = chunk_file_path.join(Path::new(
                    &{ (chunk_id.1 * session_data.planet.planet_dim.0 + chunk_id.0) as u64 }
                        .to_string(),
                ));
                chunk_file_path.set_extension("ron");

                if chunk_file_path.is_file() {
                    // TODO: Deserialize
                    warn!(
                        "Found a file for chunk {:?} at {:?}.",
                        chunk_id,
                        chunk_file_path.clone()
                    );
                } else {
                    // Create new chunk
                    session_data.planet.new_chunk(chunk_id, &mut tile_gen);
                }
            }
        } else {
            error!("No event ReaderId found for HotChunkSystem.");
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        //self.event_reader = Some(Write::<EventChannel<ChunkEvent>>::fetch(res).register_reader());
        self.event_reader = Some(
            res.fetch_mut::<EventChannel<ChunkEvent>>()
                .register_reader(),
        );
    }
}
