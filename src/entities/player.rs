use amethyst::{
    assets::ProgressCounter,
    core::{cgmath::Vector3, transform::components::Transform},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};

use {
    components::for_characters::{TagGenerator, TagPlayer},
    resources::ToppaSpriteSheet,
    utilities::{load_sprites_from_spritesheet, SpriteLoaderInfo},
};

pub fn init(world: &mut World, progress_counter_ref_opt: Option<&mut ProgressCounter>) {
    // TODO: For moddability, not hardcoded path! Check some dir first, and fall back on hardcoded path if nothng is found.
    let loader_info = SpriteLoaderInfo {
        tex_id: ToppaSpriteSheet::Player as u64,
        image_size: (128, 128),
        sprite_count: (1, 1),
        sprite_render_size: (64.0, 64.0),
    };

    if let Some(ss_handle) = load_sprites_from_spritesheet(
        world,
        "Assets/Textures/Drill.png",
        loader_info,
        progress_counter_ref_opt,
    ) {
        let drill_sprite = SpriteRender {
            sprite_sheet: ss_handle,
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };
        let mut transform = Transform::default();
        transform.translation = Vector3::new(0.0, 0.0, 0.0);

        let mut player_tag = TagPlayer { id: 0 };
        {
            let mut player_tag_resource = world.write_resource::<TagGenerator>();
            player_tag = player_tag_resource.new_player_tag();
        }

        world
            .create_entity()
            .with(transform)
            .with(Transparent)
            .with(drill_sprite)
            .with(player_tag)
            .build();
    }
}
