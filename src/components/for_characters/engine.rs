use amethyst::{
    core::nalgebra::Vector2,
    ecs::prelude::{Component, VecStorage},
};

/// The ship's engine component.
/// Holds `max_force` providing maximum movement force in a given direction.
/// Holds `efficiency` at which consumed fuel is turned into force.
/// Holds `consumption`, which represents the fuel consumption rate per force.
///
/// Burning fuel results in heat generation.
///
/// Can be upgraded.
#[derive(Debug, Clone)]
pub struct Engine {
    /// Maximum force in x- and y-direction (right, upwards) of machine's local coordinate system,
    /// specific to this machine, as it acts as an helicopter.
    ///
    /// If the machine is tilted, the rotational degree is required to map local directions of force to global.
    pub max_force: Vector2<f32,>,

    /// Engine efficiency, defines how much fuel is transformed into force, and how much into heat.
    /// Values between 0.0 and 1.0 are legal.
    ///
    /// Based on:
    /// `actual_force = (efficiency) * fuel_used / (consumption_coeff * dt)`
    /// and
    /// `heat = (1 - engine_efficiency) * fuel_used / (consumption_coeff * dt)`
    pub efficiency: f32,

    /// Fuel consumption coefficient for
    /// `fuel_used = consumption_coeff * actual_force * dt / engine_efficiency`
    pub consumption: f32,
}

impl Engine {
    pub fn new(force: Vector2<f32,>, fuel_efficiency: f32, fuel_consumption: f32,) -> Engine {
        Engine {
            max_force:   force,
            consumption: fuel_consumption,
            efficiency:  fuel_efficiency,
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine::new(Vector2::new(10000.0, 10000.0,), 1.0, 1.0,)
    }
}

impl Component for Engine {
    type Storage = VecStorage<Self,>;
}
