// delete before releasing !!
#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens)]
// maybe keep?
#![allow(unreachable_patterns)]
// Enable before releasing!
// #![deny(missing_docs, warnings)]

extern crate amethyst;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod components;
mod entities;
mod resources;
mod states;
mod systems;
mod toppa_game_data;

mod utilities;

// public stuff

pub use self::{
    states::StartupState,
    toppa_game_data::{ToppaGameData, ToppaGameDataBuilder},
};
