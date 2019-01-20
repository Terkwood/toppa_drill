// delete before releasing !!
#![allow(unused_imports, unused_variables, unused_mut, unused_parens)]
#![allow(dead_code)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
// Enable before releasing!
// #![deny(missing_docs, warnings)]

// TODO: Errorhandling! Seperate error module?
// TODO: Reduce file operations in HotChunk and Serialize/DeserializeSystem
// TODO: Wrap world in x-direction.
// TODO: Planet surface -> Hills, mountains with empty tiles?
// TODO: Empty tiles -> Procedural structered look instead of a tile with a hole

// TODO: Creating game-> Set Name, planet_dim, chunk_dim, 
// TODO: Parse "./savegames" for all existing savegames (<- containing savegame.ron) and display them in LoadGameState

// TODO: aabb collision -> blocking bounding-box-component, overlapping bounding-box-component 
// TODO: aabb collision -> collisions-system for all tiles surrounding player (check-distance based on vel?).
// TODO: blocking bounding-box-component -> collision-forces: Damage to player based on vel on impact
// TODO: overlapping bounding-box-component -> Explosion for gas-rock (overlapping bounding-box), heat for lava-rock

// TODO: Drilling into entities -> when player is colliding with entity && on ground, moving in tile-dir should init drill-state
// TODO: drill-state -> an ingame state, player loses movement control. Drags player to tile-middle based on force of drill
// TODO: add tile-entities that were drilled to inventory, if they have TagCarriable; add TagInInventory, once it is in inventory

// TODO: shopping-state for all shops -> selling and buying stuff (depends on each shop)

// TODO: Sun -> with directional light (is lighting a thing in sprite-pass?)
// TODO: SunSystem -> Move sun depending on ingame time
// TODO: Ingame time!
// (TODO: surrounding temperature based on sun?, far off in the future)

// TODO: Fog of war -> initially only the surrounding "eye-visible"-tiles are visible (uncovered based on radius around player), 
// TODO: Fog of war -> with scanner-module-component underground detection possible

// TODO: re-enable gravity in GravitySystem

extern crate amethyst;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate ron;
extern crate serde;

mod components;
mod entities;
mod events;
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

// CONSTANTS
/// Eulers number e
pub const EULER_NUMBER: f32 = 2.718281828459045;
/// Circular number pi
pub const PI: f32 = 3.14159265358979323846264338327950288419716939937510;
/// Earth's gravitational constant in `m/(s^2)`
pub const GRAVITATION: f32 = 9.81;

use amethyst::core::specs::error::NoError;

#[derive(Debug)]
enum ErrorDisplay {
    RonError(ron::ser::Error,),
    IoError(std::io::Error,),
    // Add other error types here
}

impl std::fmt::Display for ErrorDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
        match *self {
            ErrorDisplay::RonError(ref e,) => write!(f, "{}", e),
            ErrorDisplay::IoError(ref e,) => write!(f, "{}", e),
        }
    }
}

impl From<ron::ser::Error,> for ErrorDisplay {
    fn from(x: ron::ser::Error) -> Self {
        ErrorDisplay::RonError(x,)
    }
}

impl From<NoError,> for ErrorDisplay {
    fn from(e: NoError) -> Self {
        match e {}
    }
}

impl From<std::io::Error,> for ErrorDisplay {
    fn from(e: std::io::Error) -> Self {
        ErrorDisplay::IoError(e,)
    }
}
