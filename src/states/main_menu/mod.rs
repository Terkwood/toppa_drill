//! Note: I'm not happy with the code re-use here. 
//! It's basically copy-paste, changing the state's name and altering the `fn btn_` implementations.
//! 
//! Maybe have a single EndState with functions taking in Enums for the MenuState and Button, then creating an instance of this for each MenuState?

mod centre;
mod credits;
mod load_menu;
mod new_game;
mod options;

pub use self::{
    centre::CentreState,
    credits::CreditsState,
    load_menu::LoadMenuState,
    new_game::NewGameState,
    options::OptionsState,
};

/// Enum of all Menus in the Main Menu available in this game.
#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum MenuScreens{
    NewGame,
    LoadGame,
    Options,
    Credits,
    Centre,
}