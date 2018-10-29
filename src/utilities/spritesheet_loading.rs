use std::vec::Vec;

use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    ecs::prelude::World,
    renderer::{
        MaterialTextureSet, PngFormat, Sprite, SpriteSheet, SpriteSheetHandle, Texture,
        TextureCoordinates, TextureMetadata, SpriteRender,
    },
};

use {
    entities::EntitySpriteRender,
    resources::ingame::GameSprites,
};

#[derive(Debug, Clone)]
pub struct SpriteLoaderInfo {
    pub tex_id: u64,
    pub image_size: (u64, u64),
    pub sprite_count: (u64, u64),
    pub sprite_render_size: (f32, f32),
}

/// TODO: implement BmpFormat, JpgFormat, ...
pub fn load_image_png(
    world: &mut World,
    path: &str,
    id: u64,
    progress_counter_ref_opt: Option<&mut ProgressCounter>,
) {
    let loader = world.read_resource::<Loader>();

    let texture_storage = world.read_resource::<AssetStorage<Texture>>();

    let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    if let Some(progress_counter_ref) = progress_counter_ref_opt {
        let texture_handle = loader.load(
            path,
            PngFormat,
            TextureMetadata::srgb(),
            progress_counter_ref,
            &texture_storage,
        );
        material_texture_set.insert(id, texture_handle);
    } else {
        let texture_handle = loader.load(
            path,
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &texture_storage,
        );
        material_texture_set.insert(id, texture_handle);
    }
}

/// Requires uniform grid spritesheet, where every sprite has the same size.
/// Anchors the sprites in their centre.
/// Stores all spritesheets in the world's MaterialTextureSet with the specified ID.
///
/// Failure of loading has to be tracked with a `ProgressCounter`.
pub fn load_sprites_from_spritesheet(
    world: &mut World,
    sheet_path: &str,
    loader_info: SpriteLoaderInfo,
    progress_counter_ref_opt: Option<&mut ProgressCounter>,
) -> Option<SpriteSheetHandle> {
    // TODO: FIX padding ! top-most and right-most borders are broken,
    // subtract them from image-size first, before calculating sprite width/height
    let sprites_in_x = loader_info.sprite_count.0;
    let sprites_in_y = loader_info.sprite_count.1;

    if (sprites_in_x == 0) || (sprites_in_y == 0) {
        return None;
    }

    let sprite_width = loader_info.sprite_render_size.0;
    let sprite_height = loader_info.sprite_render_size.1;
    let sprite_offset_x_in_image = 1.0 / sprites_in_x as f32;
    let sprite_offset_y_in_image = 1.0 / sprites_in_y as f32;

    let mut sprites = Vec::new();

    for y in 0..sprites_in_y {
        for x in 0..sprites_in_x {
            let left = x as f32 * sprite_offset_x_in_image;
            let right = (x + 1) as f32 * sprite_offset_x_in_image;

            let top = (y + 1) as f32 * sprite_offset_y_in_image;
            let bottom = y as f32 * sprite_offset_y_in_image;

            let tex_coords = TextureCoordinates {
                left,
                right,
                bottom,
                top,
            };

            let sprite = Sprite {
                width: sprite_width as f32,
                height: sprite_height as f32,
                offsets: [sprite_width as f32 / 2.0, sprite_height as f32 / 2.0],
                tex_coords,
            };

            sprites.push(sprite);
        }
    }

    let sprite_sheet = SpriteSheet {
        texture_id: loader_info.tex_id,
        sprites: sprites,
    };

    load_image_png(
        world,
        sheet_path,
        sprite_sheet.texture_id,
        None, //not necessary since the following loader.load uses a ProgressCounter
    );

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

        if let Some(progress_counter_ref) = progress_counter_ref_opt {
            #[cfg(feature = "debug")]
            debug!("Loading spritesheet with ProgressCounter.");
            loader.load_from_data(sprite_sheet, progress_counter_ref, &sprite_sheet_storage)
        } else {
            #[cfg(feature = "debug")]
            debug!("Loading spritesheet without ProgressCounter.");
            loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
        }
    };
    Some(sprite_sheet_handle)
}
