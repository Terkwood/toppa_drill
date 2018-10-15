// delete before releasing !!
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unused_parens
)]
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
extern crate ron;

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

use amethyst::core::specs::error::NoError;

#[derive(Debug)]
enum ErrorDisplay {
    RonError(ron::ser::Error),
    IoError(std::io::Error),
    // Add other error types here
}

impl std::fmt::Display for ErrorDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ErrorDisplay::RonError(ref e) => write!(f, "{}", e),
            ErrorDisplay::IoError(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<ron::ser::Error> for ErrorDisplay {
    fn from(x: ron::ser::Error) -> Self {
        ErrorDisplay::RonError(x)
    }
}

impl From<NoError> for ErrorDisplay {
    fn from(e: NoError) -> Self {
        match e {}
    }
}

impl From<std::io::Error> for ErrorDisplay {
    fn from(e: std::io::Error) -> Self {
        ErrorDisplay::IoError(e)
    }
}
