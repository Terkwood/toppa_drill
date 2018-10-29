mod drill;
mod tracks;
mod player;

pub use self::{
    drill::*,
    tracks::*,
    player::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerParts {
    Ship(player::ShipTypes),
    Drill(drill::DrillTypes),
    Tracks,
}