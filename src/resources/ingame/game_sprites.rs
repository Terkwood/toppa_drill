use std::collections::HashMap;

use amethyst::{
    ecs::prelude::World,
    renderer::{SpriteRender, SpriteSheetHandle},
};

use entities::EntitySpriteRender;

/// Contains all `SpriteRender`s used in this session,
/// **cannot be serialized**, since `SpriteRender`s contain `Handle`s.
#[derive(Debug, Default)]
pub struct GameSprites {
    /// A map of all `SpriteRender`s used in the current session.
    sprite_renders: HashMap<EntitySpriteRender, SpriteRender>,
}

impl GameSprites {
    pub fn add(&mut self, name: EntitySpriteRender, sprite: SpriteRender) {
        if let Some(prev_val) = self.sprite_renders.insert(name, sprite) {
            warn!("A GameSprite has been overriden: {:?}", prev_val);
        };
    }

    pub fn remove(&mut self, name: EntitySpriteRender) {
        self.sprite_renders.remove(&name);
    }

    pub fn get(&self, name: &EntitySpriteRender) -> Option<&SpriteRender> {
        self.sprite_renders.get(name)
    }
}

pub fn add_spriterender(
    entity_sprite_render: EntitySpriteRender,
    mut game_sprites: &mut GameSprites,
    spritesheet_handle: SpriteSheetHandle,
    sprite_number: usize,
    flip_horizontal: bool,
    flip_vertical: bool,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: spritesheet_handle,
        sprite_number,
        flip_horizontal,
        flip_vertical,
    };
    game_sprites.add(entity_sprite_render, sprite_render);
}

pub fn get_spriterender(
    world: &World,
    entity_sprite_render: EntitySpriteRender,
) -> Option<SpriteRender> {
    let game_sprites = &world.read_resource::<GameSprites>();
    let entity_sprite_render = entity_sprite_render;
    if let Some(sprite_render) = game_sprites.get(&entity_sprite_render) {
        Some(sprite_render.clone())
    } else {
        None
    }
}
