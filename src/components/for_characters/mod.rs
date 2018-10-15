use amethyst::ecs::prelude::{Component, VecStorage};

/// This component is meant for player entities.
#[derive(Default, Debug)]
pub struct TagPlayer {/* TODO: Add id*/}

impl TagPlayer {
    pub fn new() -> TagPlayer {
        TagPlayer {}
    }
}

impl Component for TagPlayer {
    type Storage = VecStorage<Self>;
}

/// This component is meant for npc entities.
#[derive(Default, Debug)]
pub struct TagNPC {/* TODO: Add id*/}

impl TagNPC {
    pub fn new() -> TagNPC {
        TagNPC {}
    }
}

impl Component for TagNPC {
    type Storage = VecStorage<Self>;
}
