mod engine;
mod fuel_tank;

pub mod player;
pub use self::{engine::Engine, fuel_tank::FuelTank};

use amethyst::ecs::prelude::{Component, VecStorage};

/// This component is meant for player entities.
#[derive(Debug, Clone, Copy)]
pub struct PlayerBase {
    pub id: usize,
}

impl Component for PlayerBase {
    type Storage = VecStorage<Self>;
}

/// This component is meant for npc entities.
#[derive(Debug, Clone, Copy)]
pub struct NPCBase {
    pub id: usize,
}

impl Component for NPCBase {
    type Storage = VecStorage<Self>;
}

/// A resource to generate new player and NPC tags with correct, unique IDs.
/// Can run out of IDs, since no tracking regarding freed NPC or Player IDs happens.
/// The lowest valid ID is `1`, `0` is an invalid ID.
#[derive(Debug, Default)]
pub struct TagGenerator {
    player_count: usize,
    npc_count: usize,
}

impl TagGenerator {
    pub fn new_player_tag(&mut self) -> PlayerBase {
        self.player_count += 1;
        PlayerBase {
            id: self.player_count,
        }
    }

    pub fn new_npc_tag(&mut self) -> NPCBase {
        self.npc_count += 1;
        NPCBase { id: self.npc_count }
    }
}
