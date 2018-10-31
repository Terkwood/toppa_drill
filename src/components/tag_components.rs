use amethyst::ecs::prelude::{Component, NullStorage};

/// This component is meant for entities that can be stored inside the player's inventory.
#[derive(Default, Debug)]
pub struct IsIngameEntity;

impl Component for IsIngameEntity {
    type Storage = NullStorage<Self>;
}

/// This component is meant for entities that can be stored inside the player's inventory.
#[derive(Default, Debug)]
pub struct TagCarriable;

impl Component for TagCarriable {
    type Storage = NullStorage<Self>;
}

/// This component is meant for entities that can be bought or sold in the OreShop.
#[derive(Default, Debug)]
pub struct TagOreShopMerch;

impl Component for TagOreShopMerch {
    type Storage = NullStorage<Self>;
}

/// This component is meant for entities that can be bought or sold in the PartsShop.
#[derive(Default, Debug)]
pub struct TagPartsShopMerch;

impl Component for TagPartsShopMerch {
    type Storage = NullStorage<Self>;
}

/// This component is meant for ground entites the player cannot see yet.
#[derive(Default, Debug)]
pub struct TagFogOfWar;

impl Component for TagFogOfWar {
    type Storage = NullStorage<Self>;
}

/// This component is meant for player entities.
#[derive(Default, Debug)]
pub struct TagItem;

impl Component for TagItem {
    type Storage = NullStorage<Self>;
}
