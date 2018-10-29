/// An enum of all spritesheets used by ToppaDrill.
/// Casted to u64 they act as the texture-ids.
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum ToppaSpriteSheet {
    // Player
    Player,
    Drill,
    Tracks,
    Rotor,

    // Environment
    Tiles,
}
