use super::planet::Planet;
use resources::RenderConfig;
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
    pub fn new(
        name: &'static str,
        planet_dim: (u32, u32),
        chunk_dim: (u32, u32),
        render_config: &RenderConfig,
    ) -> GameSessionData {
        GameSessionData {
            game_name: name,
            planet: Planet::new(planet_dim, chunk_dim, render_config),
        }
    }
}
