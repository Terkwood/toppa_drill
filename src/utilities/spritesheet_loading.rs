use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    ecs::prelude::World,
    renderer::{
        PngFormat, SpriteSheet, SpriteSheetHandle, Texture,
        TextureMetadata, SpriteSheetFormat, TextureHandle,
    },
};

#[derive(Debug, Clone)]
pub struct SpriteLoaderInfo {
    pub sprite_count:       (u64, u64,),
    pub sprite_render_size: (f32, f32,),
}

/// TODO: implement BmpFormat, JpgFormat, ...
pub fn load_image_png(
    world: &mut World,
    path: String,
) -> TextureHandle {
    let loader = world.read_resource::<Loader>();

    let texture_storage = world.read_resource::<AssetStorage<Texture,>>();

    loader.load(
        path,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &texture_storage,
    )
}

/// TODO: implement BmpFormat, JpgFormat, ...
pub fn load_image_png_tracked(
    world: &mut World,
    path: String,
    progress_counter_ref: &mut ProgressCounter
) -> TextureHandle {
    let loader = world.read_resource::<Loader>();

    let texture_storage = world.read_resource::<AssetStorage<Texture,>>();

    loader.load(
        path,
        PngFormat,
        TextureMetadata::srgb_scale(),
        progress_counter_ref,
        &texture_storage,
    )
}

pub fn load_spritesheet(
    world: &mut World,
    base_path: String,
) -> SpriteSheetHandle {
    #[cfg(feature = "debug")]
    debug!("Loading spritesheet without ProgressCounter.");

    let mut image_path = std::string::String::from(base_path.clone());
    image_path.push_str(".png");

    let mut sheet_path = std::string::String::from(base_path);
    sheet_path.push_str(".ron");

    let texture_handle = load_image_png(
        world,
        image_path
    );

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet,>>();

    loader.load(
        sheet_path,
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_storage
    )
}

pub fn load_spritesheet_tracked(
    world: &mut World,
    base_path: String,
    progress_counter_ref: &mut ProgressCounter,
) -> SpriteSheetHandle {
    #[cfg(feature = "debug")]
    debug!("Loading spritesheet with ProgressCounter.");

    let mut image_path = base_path.clone();
    image_path.push_str(".png");

    let mut sheet_path = base_path;
    sheet_path.push_str(".ron");

    let texture_handle = load_image_png_tracked(
        world,
        image_path,
        progress_counter_ref,
    );

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet,>>();

    loader.load(
        sheet_path,
        SpriteSheetFormat,
        texture_handle,
        progress_counter_ref,
        &sprite_sheet_storage
    )
}
