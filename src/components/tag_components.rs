use amethyst::ecs::prelude::{Component, NullStorage};

/// This component is meant for entities that can be stored inside the player's inventory.
#[derive(Default, Debug)]
pub struct TagCarriable {}

impl TagCarriable {
    pub fn new() -> TagCarriable {
        TagCarriable {}
    }
}

impl Component for TagCarriable {
    type Storage = NullStorage<Self>;
}

/// This component is meant for entities that can be bought or sold in the OreShop.
#[derive(Default, Debug)]
pub struct TagOreShopMerch {}

impl TagOreShopMerch {
    pub fn new() -> TagOreShopMerch {
        TagOreShopMerch {}
    }
}

impl Component for TagOreShopMerch {
    type Storage = NullStorage<Self>;
}

/// This component is meant for entities that can be bought or sold in the PartsShop.
#[derive(Default, Debug)]
pub struct TagPartsShopMerch {}

impl TagPartsShopMerch {
    pub fn new() -> TagPartsShopMerch {
        TagPartsShopMerch {}
    }
}

impl Component for TagPartsShopMerch {
    type Storage = NullStorage<Self>;
}

/// This component is meant for player entities.
#[derive(Default, Debug)]
pub struct TagPlayer {}

impl TagPlayer {
    pub fn new() -> TagPlayer {
        TagPlayer {}
    }
}

impl Component for TagPlayer {
    type Storage = NullStorage<Self>;
}

/// This component is meant for npc entities.
#[derive(Default, Debug)]
pub struct TagNPC {}

impl TagNPC {
    pub fn new() -> TagNPC {
        TagNPC {}
    }
}

impl Component for TagNPC {
    type Storage = NullStorage<Self>;
}

/// This component is meant for ground entites the player cannot see yet.
#[derive(Default, Debug)]
pub struct TagFogOfWar {}

impl TagFogOfWar {
    pub fn new() -> TagFogOfWar {
        TagFogOfWar {}
    }
}

impl Component for TagFogOfWar {
    type Storage = NullStorage<Self>;
}

/// This component is meant for player entities.
#[derive(Default, Debug)]
pub struct TagItem {}

impl TagItem {
    pub fn new() -> TagItem {
        TagItem {}
    }
}

impl Component for TagItem {
    type Storage = NullStorage<Self>;
}
