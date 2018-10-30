use amethyst::{
    core::cgmath::{Rad, Vector2},
    ecs::prelude::{Component, VecStorage},
};

/// A physics-affected entity need to have either friction, air resistance or both,
/// otherwise it won't move.
#[derive(Debug, Default, Clone)]
pub struct PhysicalProperties {
    /// The weight of the entity itself, like a car's empty mass.
    pub mass: f64,
    /// The resistance against rotational acceleration.
    /// If it is `None`, this entity cannot be rotated.
    pub inertia: Option<f64>,
    /// The value of the friction coefficient.
    /// If it is `None`, this entity has supposedly no ground contact,
    /// e.g. the Ship, as it is supported by the Tracks.
    pub friction: Option<f64>,
    /// The value of the air-resistance coefficient.
    /// If it is `None`, this entity has supposedly no air resistance,
    /// e.g. the Drill, as it is supposed to be pulled into the Ship when flying.
    pub air_resistance: Option<f64>,
}

impl PhysicalProperties {
    pub fn new(
        mass: f64,
        inertia: Option<f64>,
        friction: Option<f64>,
        air_resistance: Option<f64>,
    ) -> Self {
        PhysicalProperties {
            mass,
            inertia,
            friction,
            air_resistance,
        }
    }
}

impl Component for PhysicalProperties {
    type Storage = VecStorage<Self>;
}
