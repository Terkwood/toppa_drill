use std::{
    fs,
    ops::{Deref, DerefMut},
    path::*,
    string::ToString,
};

use ron;
use serde::Serializer;

use amethyst::{
    core::{
        specs::saveload::{
            DeserializeComponents, SerializeComponents, U64Marker, U64MarkerAllocator,
        },
        timing::Time,
    },
    ecs::prelude::*,
};

use components::for_characters::TagPlayer;
use entities::tile::TileTypes;
use resources::{
    ingame::{
        planet::{Chunk, ChunkIndex, Planet, TileIndex},
        GameSessionData,
    },
    RenderConfig,
};

/// TODO: Everything
/// Serializes chunks and stores them in `.ron` format.
pub struct SerChunkSystem;

impl<'a> System<'a> for SerChunkSystem {
    type SystemData = (ReadExpect<'a, GameSessionData>,);

    fn run(&mut self, (session_data,): Self::SystemData) {}
}

pub struct DeChunkSystem;

impl<'a> System<'a> for DeChunkSystem {
    type SystemData = (Read<'a, Time>,);

    fn run(&mut self, (time,): Self::SystemData) {}
}
