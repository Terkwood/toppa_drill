pub mod camera;
pub mod player;
pub mod tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EntitySpriteRender {
    Player,
    Ore(tile::TileTypes),
}
