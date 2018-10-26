//! This module contains resources specific to a game.
//! They should be set up/added to the world when creating a new game, or loading a savegame,
//! and be removed when the player exits to the MainMenu or ends the application.
mod game_session;
mod sprite_renders;
mod savegame_path;

pub mod planet;
pub use self::{
    game_session::GameSessionData, 
    sprite_renders::GameSprites,
    savegame_path::SavegamePaths,
};
