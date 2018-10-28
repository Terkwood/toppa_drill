use amethyst::{
    core::cgmath::{Vector2, Rad},
    ecs::prelude::{Component, VecStorage},
};

#[derive(Debug, Default, Clone)]
pub struct PhysicalProperties {
    /// The weight of the entity itself (if it has any), like a car's empty mass.
    pub mass: f64,
    /// The resistance against rotational acceleration.
    pub inertia: f64,
    /// The value of the friction coefficient
    pub fric: f64,
}

impl PhysicalProperties {
    pub fn new(mass: f64, inertia: f64, friction: f64) -> Self {
        PhysicalProperties {
            mass,
            inertia,
            fric: friction,
        }
    }
}

impl Component for PhysicalProperties {
    type Storage = VecStorage<Self>;
}
