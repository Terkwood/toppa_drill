use amethyst::{assets::ProgressCounter, prelude::*, renderer::SpriteRender};

use crate::{
    entities::EntitySpriteRender,
    resources::{
        ingame::game_world::{ChunkIndex, TileIndex},
        GameSprites, ToppaSpriteSheet,
    },
    utilities::{load_sprites_from_spritesheet, SpriteLoaderInfo},
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
pub fn prepare_spritesheet(
    world: &mut World,
    progress_counter_ref_opt: Option<&mut ProgressCounter,>,
) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.
    let loader_info = SpriteLoaderInfo {
        tex_id:             ToppaSpriteSheet::Tiles as u64,
        image_size:         (128, 128,),
        sprite_count:       (4, 4,),
        sprite_render_size: (64.0, 64.0,),
    };

    if let Some(ss_handle,) = load_sprites_from_spritesheet(
        world,
        "Assets/Textures/Tiles.png",
        loader_info.clone(),
        progress_counter_ref_opt,
    ) {
        let mut game_sprites = world.write_resource::<GameSprites>();

        for y in 0 .. loader_info.sprite_count.0 {
            for x in 0 .. loader_info.sprite_count.1 {
                let sprite_number = (y * loader_info.sprite_count.1 + x) as usize;
                let ore_sprite = SpriteRender {
                    sprite_sheet: ss_handle.clone(),
                    sprite_number,
                    flip_horizontal: false,
                    flip_vertical: false,
                };

                //TODO: Make generic, currently bound to the png-layout
                match sprite_number {
                    0 => {
                        game_sprites
                            .add(EntitySpriteRender::Ore(TileTypes::Magnetite,), ore_sprite,)
                    },
                    1 => {
                        game_sprites
                            .add(EntitySpriteRender::Ore(TileTypes::Pyrolusite,), ore_sprite,)
                    },
                    2 => {
                        game_sprites.add(EntitySpriteRender::Ore(TileTypes::Fossile,), ore_sprite,)
                    },
                    3 => {
                        game_sprites
                            .add(EntitySpriteRender::Ore(TileTypes::Molybdenite,), ore_sprite,)
                    },
                    4 => game_sprites.add(EntitySpriteRender::Ore(TileTypes::Lava,), ore_sprite,),
                    5 => game_sprites.add(EntitySpriteRender::Ore(TileTypes::Rock,), ore_sprite,),
                    6 => game_sprites.add(EntitySpriteRender::Ore(TileTypes::Gas,), ore_sprite,),
                    7 => game_sprites.add(EntitySpriteRender::Ore(TileTypes::Galena,), ore_sprite,),
                    8 => {
                        game_sprites.add(EntitySpriteRender::Ore(TileTypes::Bornite,), ore_sprite,)
                    },
                    9 => {
                        game_sprites.add(EntitySpriteRender::Ore(TileTypes::Chromite,), ore_sprite,)
                    },
                    10 => {
                        game_sprites
                            .add(EntitySpriteRender::Ore(TileTypes::Cassiterite,), ore_sprite,)
                    },
                    11 => {
                        game_sprites.add(EntitySpriteRender::Ore(TileTypes::Cinnabar,), ore_sprite,)
                    },
                    12 => game_sprites.add(EntitySpriteRender::Ore(TileTypes::Dirt,), ore_sprite,),
                    13 => game_sprites.add(EntitySpriteRender::Ore(TileTypes::Gold,), ore_sprite,),
                    14 => game_sprites.add(EntitySpriteRender::Ore(TileTypes::Empty,), ore_sprite,),
                    15 => {
                        game_sprites.add(EntitySpriteRender::Ore(TileTypes::Bauxite,), ore_sprite,)
                    },
                    _ => continue,
                };
            }
        }
    }
}
