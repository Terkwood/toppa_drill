#![allow(unused_imports)]
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

use crate::{
    components::for_characters::PlayerBase,
    entities::tile::TileTypes,
    resources::{
        ingame::{
            planet::{Chunk, ChunkIndex, Planet, TileIndex},
            GameSessionData, SavegamePaths,
        },
        RenderConfig,
    },
};

/// TODO: Serialize players.
/// Creates a savegame by calling different serialization systems, based on the current [GameSessionData](struct.GameSessionData.html).
/// Uses `.ron` format.
pub struct SerSavegameSystem;

impl<'a,> System<'a,> for SerSavegameSystem {
    type SystemData = (
        Option<Read<'a, GameSessionData,>,>,
        Option<Read<'a, RenderConfig,>,>,
        Option<Read<'a, SavegamePaths,>,>,
    );

    fn run(&mut self, (session_data, render_config, paths,): Self::SystemData,) {
        #[cfg(feature = "debug")]
        debug!("+------------");

        if let (Some(session_data,), Some(_render_config,), Some(paths,),) =
            (session_data, render_config, paths,)
        {
            session_data.save(&paths,);
        }
        else {
            error!("| Resources not found.")
        }
        #[cfg(feature = "debug")]
        debug!("+------------");
    }
}
