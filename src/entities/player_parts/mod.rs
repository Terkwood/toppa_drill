mod drill;
mod player;
mod tracks;

pub use self::{drill::*, player::*, tracks::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerParts {
    Ship(player::ShipTypes),
    Drill(drill::DrillTypes),
    Tracks,
}
