use super::planet::Planet;
/// Data specific to the current game,
/// gets loaded or created when the User starts a new game.
#[derive(Debug)]
pub struct GameSessionData {
    /// The name of this game, also used as the savegame's name, and should be individual each time,
    /// lest another savegame gets overwritten.
    pub game_name: &'static str,

    pub planet: Planet,
}

impl GameSessionData {
    pub fn new(name: &'static str) -> GameSessionData {
        GameSessionData {
            game_name: name,
            planet: Planet::default(),
        }
    }
}

impl Default for GameSessionData {
    fn default() -> Self {
        GameSessionData::new("anonymous")
    }
}
