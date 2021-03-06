use amethyst::{
    core::{nalgebra::Vector2, transform::components::Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::physics::{Dynamics, PhysicalProperties},
    GRAVITATION,
};

/// NOTE: Currently only resets force vec and torque, no gravitational effect.
/// TODO: Get rotational diff local - world rotation.
#[derive(Default)]
pub struct GravitationSystem;

impl<'s,> System<'s,> for GravitationSystem {
    type SystemData = (
        WriteStorage<'s, Dynamics,>,
        ReadStorage<'s, Transform,>,
        ReadStorage<'s, PhysicalProperties,>,
    );

    fn run(&mut self, (mut dynamics, transforms, masses,): Self::SystemData,) {
        for (mut dynamic, transform, mass,) in (&mut dynamics, &transforms, &masses,).join() {
            let _grav_force2 = (mass.mass * -GRAVITATION * 20.0);
            /*
                * Vector2::new(
                    transform.orientation().up[0] as f32,
                    transform.orientation().up[1] as f32,
                );
            */ //* transform.up(); // (magnitude) * direction

            dynamic.force = Vector2::new(0.0, 0.0,); //Vector2::new(grav_force2[0],grav_force2[1]); //
            dynamic.torque = 0.0; //always reset
        }
    }
}
