use amethyst::{
    core::transform::components::{Parent, Transform},
    ecs::prelude::Entity,
    prelude::*,
    renderer::{Camera, Projection},
};

pub fn init(world: &mut World, view_dim: (u32, u32), parent: Entity) {
    let mut transform = Transform::default();
    transform.translation[2] += 100.0;

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -(view_dim.0 as f32) / 2.0,
            (view_dim.0 as f32) / 2.0,
            (view_dim.1 as f32) / 2.0,
            -(view_dim.1 as f32) / 2.0,
        )))
        .with(transform)
        .with(Parent { entity: parent })
        .build();
}
