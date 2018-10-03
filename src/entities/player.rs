use amethyst::{
    prelude::*,
    assets::{
        ProgressCounter,
    },
    renderer::{
        Transparent,
        SpriteRender,
    },
    core::{
        transform::components::{
            Transform,
            GlobalTransform,
        },
        cgmath::{
            Vector3,
        }
    },
};

use utilities::{
    spritesheet_loading::{
        load_sprites_from_spritesheet,
        SpriteLoaderInfo,
    },
};

use entities::camera;

pub fn init(world: &mut World, progress_counter_ref: &mut ProgressCounter){
    camera::init(world, (512.0, 512.0));

    let loader_info = SpriteLoaderInfo{
        tex_id: 0,
        image_size: (128, 128),
        sprite_count: (1, 1),
        sprite_render_size: (64.0, 64.0),
    };
    
    if let Some(ss_handle) = load_sprites_from_spritesheet(
        world, "Textures/Drill.png", loader_info, progress_counter_ref)
    {
        let drill_sprite = SpriteRender {
            sprite_sheet: ss_handle,
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };
        let mut transform = Transform::default();
        transform.translation = Vector3::new(0.0, 0.0, 0.0);

        world.create_entity()
            .with(
                transform
            )
            .with(
                Transparent
            )
            .with(
                drill_sprite
            )
            .with(
                GlobalTransform::default()
            )
        .build();
    }
}