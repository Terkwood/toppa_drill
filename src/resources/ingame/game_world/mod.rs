mod chunk;
mod planet;
mod tile;

pub use self::{
    chunk::*,
    tile::*,
    planet::*,
};

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub enum GameWorldError {
    #[allow(dead_code)]
    NotImplemented,
    ChunkProblem(ChunkError),
    TileProblem(TileError),
}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub enum ChunkError {
    #[allow(dead_code)]
    NotImplemented,
    IndexOutOfBounds,
    NotFound,
}

use crate::entities::EntitySpriteRender;

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub enum TileError {
    #[allow(dead_code)]
    NotImplemented,
    IndexOutOfBounds,
    SpriteRenderNotFound(EntitySpriteRender),
}

use amethyst::{
    core::transform::components::Transform,
    ecs::{prelude::*, storage::MaskedStorage, world::EntitiesRes, Storage},
    renderer::{SpriteRender,Flipped},
    shred::{DefaultProvider, FetchMut},
};

use crate::{
    components::{
        for_ground_entities::TileBase,
        IsIngameEntity,
    },
    resources::{GameSprites, RenderConfig},
};

/// Internal use only (for the Chunk-Hotloading), do not use!
pub struct TileGenerationStorages<'a> {
    pub entities: Read<'a, EntitiesRes>,
    pub tile_base: Storage<'a, TileBase, FetchMut<'a, MaskedStorage<TileBase>>>,
    pub sprite_render: Storage<'a, SpriteRender, FetchMut<'a, MaskedStorage<SpriteRender>>>,
    pub transform: Storage<'a, Transform, FetchMut<'a, MaskedStorage<Transform>>>,
    pub ingame_entity: Storage<'a, IsIngameEntity, FetchMut<'a, MaskedStorage<IsIngameEntity>>>,
    pub game_sprites: Read<'a, GameSprites, DefaultProvider>,
    pub render_config: Read<'a, RenderConfig, DefaultProvider>,
    pub flipped_vertical: Storage<'a, Flipped, FetchMut<'a, MaskedStorage<Flipped>>>, 
}