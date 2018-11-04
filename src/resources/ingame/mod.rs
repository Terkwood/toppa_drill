//! This module contains resources specific to a game.
//! They should be set up/added to the world when creating a new game, or loading a savegame,
//! and be removed when the player exits to the MainMenu or ends the application.
mod game_session;
mod savegame_path;

pub mod game_world;
//pub mod planet;
pub use self::{game_session::GameSessionData, savegame_path::SavegamePaths};
