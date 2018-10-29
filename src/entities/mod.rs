pub mod player_parts;
pub mod camera;
pub mod tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EntitySpriteRender {
    Player(player_parts::PlayerParts),
    Ore(tile::TileTypes),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EntityError {
    NotImplemented,
    PlayerProblem(player_parts::PlayerError),
    DrillProblem(player_parts::DrillError),
    TracksProblem(player_parts::TracksError),
}
