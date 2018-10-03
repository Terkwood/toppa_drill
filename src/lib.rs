// delete before releasing !!
#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
// maybe keep?
#![allow(unreachable_patterns)]

extern crate amethyst;
#[macro_use]
extern crate log;

mod states;
mod systems;
mod components;
mod entities;
mod resources;
mod toppa_game_data;

mod utilities;

// public stuff

pub use self::{
    states::StartupState,
    toppa_game_data::{
        ToppaGameData,
        ToppaGameDataBuilder,
    }
};
