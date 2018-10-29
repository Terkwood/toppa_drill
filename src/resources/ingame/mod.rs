//! This module contains resources specific to a game.
//! They should be set up/added to the world when creating a new game, or loading a savegame,
//! and be removed when the player exits to the MainMenu or ends the application.
mod game_session;
mod game_sprites;
mod savegame_path;

pub mod planet;
pub use self::{
    game_session::GameSessionData, 
    game_sprites::{
        GameSprites, 
        add_spriterender,
        get_spriterender,
    }, 
    savegame_path::SavegamePaths,
};
