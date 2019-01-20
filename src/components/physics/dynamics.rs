use amethyst::{
    core::nalgebra::Vector2,
    ecs::prelude::{Component, VecStorage},
};

#[derive(Debug, Clone)]
pub struct Dynamics {
    /// The current velocity, consisting of an x-component and a y-component.
    /// The vectors length should be limited by the System using this, e.g. implicitely due to the movement equation.
    pub vel: Vector2<f32,>,

    /// Current rotational speed around the z-axes in rad/s
    /// Should be limited by the System using this.
    pub omega: f32,

    /// Force applied to a body/entity, has a direction.
    /// The movement system is responsible for detecting and handeling leverage,
    /// caused by forces not being applied to the center of mass of an entity.
    pub force: Vector2<f32,>,

    /// Torque applied on the z-axis.
    /// - positive = counter-clock-wise
    /// - negative = clock-wise
    pub torque: f32,
}

impl Dynamics {
    pub fn new(vel: Vector2<f32,>, omega: f32, force: Vector2<f32,>, torque: f32,) -> Self {
        Dynamics {
            vel,
            omega,
            force,
            torque,
        }
    }
}

impl Default for Dynamics {
    fn default() -> Self {
        Self::new(
            Vector2::new(0.0, 0.0,),
            0.0,
            Vector2::new(0.0, 0.0,),
            0.0,
        )
    }
}

impl Component for Dynamics {
    type Storage = VecStorage<Self,>;
}
