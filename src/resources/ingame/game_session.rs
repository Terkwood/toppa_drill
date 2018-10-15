use super::planet::Planet;
/// Data specific to the current game,
/// gets loaded or created when the User starts a new game.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameSessionData {
    /// The name of this game, also used as the savegame's name, and should be individual each time,
    /// lest another savegame gets overwritten.
    pub game_name: &'static str,

    /// The planet is basically a container for chunks, which hold the different tiles. 
    /// This enables loading and unloading areas in larger bits than single entites, helping with performance,
    /// while also sparing memory.
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
