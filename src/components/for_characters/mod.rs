use amethyst::ecs::prelude::{Component, VecStorage};

/// This component is meant for player entities.
#[derive(Debug, Clone, Copy)]
pub struct TagPlayer {
    pub id: usize,
}

impl Component for TagPlayer {
    type Storage = VecStorage<Self>;
}

/// This component is meant for npc entities.
#[derive(Debug, Clone, Copy)]
pub struct TagNPC {
    pub id: usize,
}

impl Component for TagNPC {
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
    pub fn new_player_tag(&mut self) -> TagPlayer {
        self.player_count += 1;
        TagPlayer {
            id: self.player_count,
        }
    }

    pub fn new_npc_tag(&mut self) -> TagNPC {
        self.npc_count += 1;
        TagNPC { id: self.npc_count }
    }
}
