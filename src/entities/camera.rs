use amethyst::{
    core::transform::components::Transform,
    prelude::*,
    renderer::{Camera, Projection},
};

pub fn init(world: &mut World, view_dim: (f32, f32)) {
    let mut transform = Transform::default();
    transform.translation[2] = 10.0;

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -view_dim.0 / 2.0,
            view_dim.0 / 2.0,
            view_dim.1 / 2.0,
            -view_dim.1 / 2.0,
        ))).with(transform)
        .build();
}
