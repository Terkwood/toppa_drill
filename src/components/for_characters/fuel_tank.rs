use amethyst::ecs::prelude::{Component, VecStorage};

/// The ship's fuel tank.
/// Holds `fuel_level`, the current amount of fuel in the tank.
/// Holds `capacity`, the maximum amount of fuel carryable.
/// Holds `weight_per_fuel`, the weght of each unit of fuel, updated on every movement or when refilling.
#[derive(Debug, Default, Clone)]
pub struct FuelTank {
    /// Current fuel stored in this tank.
    pub fuel_level: f32,

    /// Maximum fuel storable by this tank.
    pub capacity: f32,

    /// weight of the fuel
    pub weight_per_fuel: f32,
}

impl FuelTank {
    pub fn new(fuel_level: f32, capacity: f32, weight_per_fuel: f32,) -> Self {
        FuelTank {
            fuel_level,
            capacity,
            weight_per_fuel,
        }
    }
}

impl Component for FuelTank {
    type Storage = VecStorage<Self,>;
}
