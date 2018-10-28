use std::collections::HashMap;

use amethyst::renderer::SpriteRender;

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

    pub fn get(&self, name: EntitySpriteRender) -> Option<&SpriteRender> {
        self.sprite_renders.get(&name)
    }
}
