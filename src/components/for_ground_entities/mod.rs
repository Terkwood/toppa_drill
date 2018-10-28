use amethyst::ecs::prelude::{Component, VecStorage};

use entities::tile::TileTypes;

/// This component is meant for tiles.
#[derive(Debug, Clone, Copy)]
pub struct TileBase {
    pub kind: TileTypes,
}

impl Component for TileBase {
    type Storage = VecStorage<Self>;
}
