use amethyst::{
    assets::ProgressCounter,
    core::{
        cgmath::Vector3,
        transform::components::{Transform},
    },
    prelude::*,
    renderer::{SpriteRender, Transparent},
};

use utilities::{load_sprites_from_spritesheet, SpriteLoaderInfo};
use resources::{
    ToppaSpriteSheet,
    ingame::planet::{TileIndex,ChunkIndex,},
};

/// An enumaration of all ground tile types.
#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum TileTypes {
    /// Nothing drilleable here.
    Empty,
    /// Some dirt blocking your vision, worthless.
    Dirt,
    /// Bed rock, indestructible.
    BedRock,

    /// Plain old rock. Worthless but hard,
    /// and can fall on your head.
    Rock,
    /// Gassy rock, explodes when it gets ignited, careful with your dynamite.
    Gas,
    /// Hot rock, ouch.
    Lava,

    /// A treasure, can be sold for a premium.
    TreasureChest,
    /// An artifact, maybe some museum is interested?
    Skeleton,
    /// An artifact, there should be a museum interested in this.
    Fossile,
    /// An artifact, every museum wants this!
    MeteoriteShard,

    /// `Ag.2 S` for production of silver
    Acanthite,
    /// `Ba S O.4` for production of barium
    Barite,
    /// `Al (O H.3) + Al O O H` for production of aluminium
    Bauxite,
    /// `Cu.5 Fe S.4` for production of copper
    Bornite,
    /// `Sn O.2` for production of tin
    Cassiterite,
    /// `Cu.2 S` for production of copper
    Chalcocite,
    /// `(Fe, Mg) Cr.2 O.4` for production of chrome
    Chromite,
    /// `Hg S` for production of mercury
    Cinnabar,
    /// `Pb S` for production of lead
    Galena,
    /// `Au`, native gold
    Gold,
    /// `Fe.2 O.3` for production of iron
    Hematite,
    /// `Fe.3 O.4` for production of iron
    Magnetite,
    /// `Mo S.2` for production of molybdenum
    Molybdenite,
    /// `Mn O.2` for production of manganese
    Pyrolusite,
    /// `Pt As.2` for production of platinum
    Sperrylite,
    /// `Zn S` for production of zinc
    Sphalerite,
}

/// Should run in a State and only once.
/// Loads the spritesheet from the hardcoded path "Assets/Textures/Ores.png"
/// and calculates the sprite-vec.
pub fn prepare_spritesheet(world: &mut World, progress_counter_ref: &mut ProgressCounter) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.
    let loader_info = SpriteLoaderInfo {
        tex_id: ToppaSpriteSheet::Tiles as u64,
        image_size: (128, 128),
        sprite_count: (4, 4),
        sprite_render_size: (64.0, 64.0),
    };

    if let Some(ss_handle) = load_sprites_from_spritesheet(
        world,
        "Assets/Textures/Ores.png",
        loader_info,
        progress_counter_ref,
    ){
        
    }
}

pub fn create_ore_with_world(world: &mut World, chunk: ChunkIndex, index: TileIndex, tiletype: TileTypes){
    // TODO: Get spriteSheetHandle from texture id.
    // TODO: Get sprite from TileType
    // TODO: Calculate transform based on TileIndex & ChunkIndex
    /*
    let drill_sprite = SpriteRender {
        sprite_sheet: ss_handle,
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };
    let mut transform = Transform::default();
    transform.translation = Vector3::new(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(transform)
        .with(Transparent)
        .with(drill_sprite)
        .build();
    */
    error!("Not implemented yet.");
}
